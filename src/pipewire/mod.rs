use std::{cell::RefCell, collections::HashMap, error::Error, rc::Rc};

use node::NodeTypeDirection;
use pipewire::{
    context::Context,
    keys,
    link::{Link, LinkChangeMask},
    main_loop::MainLoop,
    node::{Node, NodeChangeMask},
    port::{Port, PortChangeMask},
    proxy::ProxyT,
    types::ObjectType,
};
use proxies::Proxies;
use state::State;
use tracing::error;

pub mod node;
mod proxies;
pub mod state;

use std::thread::{self, JoinHandle};

fn handle_node(proxies: Rc<RefCell<Proxies>>, state: State, id: u32, node: Node) {
    let clone_state = state.clone();
    let listener = node
        .add_listener_local()
        .info(move |info| {
            let state = &clone_state;
            let res: Result<(), &str> = (move || {
                if info.change_mask().contains(NodeChangeMask::PROPS) {
                    let props = info.props().ok_or("no props")?;

                    let name = props
                        .get(&keys::NODE_DESCRIPTION)
                        .or_else(|| props.get(&keys::NODE_NAME))
                        .ok_or("no name")?;
                    let media = props.get(&keys::MEDIA_CLASS);
                    let category = props
                        .get(&keys::MEDIA_CATEGORY)
                        .filter(|name| name.contains("Duplex"))
                        .and(Some(NodeTypeDirection::Both));
                    state.change_node(node::NodeValue {
                        id,
                        media: media.into(),
                        name: name.to_owned(),
                        state: info.state().into(),
                        ports: HashMap::new(),
                        node_type: category.unwrap_or_else(|| media.into()),
                    });
                }
                if info.change_mask().contains(NodeChangeMask::STATE)
                    && !info.change_mask().contains(NodeChangeMask::PROPS)
                {
                    state.get_node(info.id()).and_then(|node| {
                        node.change_state(info.state().into());
                        Some(node)
                    });
                }
                Ok(())
            })();
            match res {
                Ok(_) => (),
                Err(e) => error!("error {}", e),
            }
        })
        .register();
    let clone_state = state.clone();
    let remove_listener = node
        .upcast_ref()
        .add_listener_local()
        .removed(move || {
            clone_state.remove_node(id);
        })
        .register();

    proxies
        .borrow_mut()
        .add_proxy(node)
        .add_listener(listener)
        .add_listener(remove_listener);
}

fn handle_port(
    proxies: Rc<RefCell<Proxies>>,
    state: State,
    id: u32,
    port: Port,
    original_node_id: u32,
) {
    state.add_map_port(id, original_node_id);
    let clone_state = state.clone();
    let listener = port
        .add_listener_local()
        .info(move |info| {
            let res: Result<(), Box<dyn Error>> = (|| {
                if info.change_mask().contains(PortChangeMask::PROPS) {
                    let props = info.props().ok_or("no props")?;
                    let name = props.get(&keys::PORT_NAME).ok_or("no name")?;
                    let new_node_id = props.get(&keys::NODE_ID).ok_or("no node id")?.parse()?;
                    let direction = info.direction();
                    let format = props.get(&keys::FORMAT_DSP);

                    let old_node_id = clone_state.get_map_port(id);
                    clone_state.modify_map_port(id, new_node_id);
                    if let Some(old_node_id) = old_node_id {
                        if old_node_id != new_node_id {
                            clone_state.get_node(old_node_id).and_then(|node| {
                                node.remove_port(id);
                                Some(node)
                            });
                        }
                    } else {
                        error!("old node not exist {}", id);
                    }
                    clone_state.get_node(new_node_id).and_then(|node| {
                        node.replace_or_add_port(node::Port {
                            id,
                            name: name.to_owned(),
                            direction: direction.into(),
                            format: format.into(),
                        });
                        Some(node)
                    });
                }
                Ok(())
            })();
            match res {
                Ok(_) => (),
                Err(e) => error!("error {}", e),
            };
        })
        .register();
    let clone_state = state.clone();
    let remove_listener = port
        .upcast_ref()
        .add_listener_local()
        .removed(move || {
            let node_id = clone_state.get_map_port(id);
            if let Some(node_id) = node_id {
                clone_state.get_node(node_id).and_then(|node| {
                    node.remove_port(id);
                    Some(node)
                });
            } else {
                error!("no node id for port {}", id);
            }
        })
        .register();
    proxies
        .borrow_mut()
        .add_proxy(port)
        .add_listener(listener)
        .add_listener(remove_listener);
}
fn handle_link(proxies: Rc<RefCell<Proxies>>, state: State, id: u32, link: Link) {
    let clone_state = state.clone();
    let listener = link
        .add_listener_local()
        .info(move |info| {
            let state = &clone_state;
            let res: Result<(), &str> = (move || {
                if info.change_mask().contains(LinkChangeMask::PROPS) {
                    state.change_link(node::LinkValue {
                        id,
                        node_from: info.output_node_id(),
                        node_to: info.input_node_id(),
                        port_from: info.output_port_id(),
                        port_to: info.input_port_id(),
                        state: info.state().into(),
                    });
                }
                if info.change_mask().contains(LinkChangeMask::STATE)
                    && !info.change_mask().contains(LinkChangeMask::PROPS)
                {
                    state.get_link(info.id()).change_state(info.state().into());
                }
                Ok(())
            })();
            match res {
                Ok(_) => (),
                Err(e) => error!("error {}", e),
            }
        })
        .register();
    let clone_state = state.clone();
    let remove_listener = link
        .upcast_ref()
        .add_listener_local()
        .removed(move || {
            clone_state.remove_link(id);
        })
        .register();

    proxies
        .borrow_mut()
        .add_proxy(link)
        .add_listener(listener)
        .add_listener(remove_listener);
}

pub fn create_pipewire_runner(
    state: State,
) -> JoinHandle<Result<(), Box<dyn Error + Send + Sync>>> {
    thread::spawn(move || {
        let mainloop = MainLoop::new(None)?;
        let context = Context::new(&mainloop)?;
        let core = context.connect(None)?;
        let registry = Rc::new(core.get_registry()?);
        let registry_clone = Rc::clone(&registry);

        let proxies = Proxies::new();

        let _register = registry
            .add_listener_local()
            .global(move |global| {
                let res: Result<(), Box<dyn Error>> = (|| {
                    let state = state.clone();
                    let id = global.id;
                    if global.type_ == ObjectType::Node {
                        let node: Node = registry_clone.bind(global).unwrap();
                        handle_node(proxies.clone(), state.clone(), global.id, node);
                    }
                    if global.type_ == ObjectType::Port {
                        let port: Port = registry_clone.bind(global).unwrap();

                        let original_node_id = global
                            .props
                            .ok_or("no props")?
                            .get(&keys::NODE_ID)
                            .ok_or("no node id")?
                            .parse()?;
                        handle_port(proxies.clone(), state.clone(), id, port, original_node_id);
                    }
                    if global.type_ == ObjectType::Link {
                        let link: Link = registry_clone.bind(global).unwrap();
                        handle_link(proxies.clone(), state, id, link);
                    }

                    Ok(())
                })();
                match res {
                    Ok(_) => (),
                    Err(e) => error!("error {}", e),
                }
            })
            .register();

        mainloop.run();
        Ok(())
    })
}
