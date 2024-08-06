use std::{cell::RefCell, error::Error, rc::Rc};

use pipewire::{
    context::Context,
    keys,
    main_loop::MainLoop,
    node::{Node, NodeChangeMask},
    proxy::ProxyT,
    types::ObjectType,
};
use proxies::Proxies;
use state::{State, StateChangeEvent};
use tokio::sync::broadcast;
use tracing::error;

pub mod node;
mod proxies;
pub mod state;

use std::thread::{self, JoinHandle};

pub fn create_pipewire_runner(
    state_broadcast: broadcast::Sender<StateChangeEvent>,
) -> JoinHandle<Result<(), Box<dyn Error + Send + Sync>>> {
    thread::spawn(move || {
        let mainloop = MainLoop::new(None)?;
        let context = Context::new(&mainloop)?;
        let core = context.connect(None)?;
        let registry = Rc::new(core.get_registry()?);
        let registry_clone = Rc::clone(&registry);

        let proxies = Proxies::new();

        let state = Rc::new(RefCell::new(State::new(state_broadcast)));

        let _register = registry
            .add_listener_local()
            .global(move |global| {
                let state = state.clone();
                if global.type_ == ObjectType::Node {
                    let node: Node = registry_clone.bind(global).unwrap();
                    let id = global.id;
                    let clone_state = state.clone();
                    let listener = node
                        .add_listener_local()
                        .info(move |info| {
                            let state = clone_state.clone();
                            let res: Result<(), &str> = (move || {
                                let mut state = (*state).borrow_mut();
                                if info.change_mask().contains(NodeChangeMask::PROPS) {
                                    let name = info
                                        .props()
                                        .ok_or("no props")?
                                        .get(&keys::NODE_NAME)
                                        .ok_or("no name")?;
                                    state.change_node(node::NodeValue {
                                        id,
                                        name: name.to_owned(),
                                        state: info.state().into(),
                                        in_ports: vec![],
                                        out_ports: vec![],
                                    });
                                }
                                if info.change_mask().contains(NodeChangeMask::STATE)
                                    && !info.change_mask().contains(NodeChangeMask::PROPS)
                                {
                                    state.get_node(info.id()).change_state(info.state().into());
                                }
                                Ok(())
                            })();
                            match res {
                                Ok(_) => (),
                                Err(e) => error!("error {}", e),
                            }
                        })
                        .register();
                    let remove_listener = node
                        .upcast_ref()
                        .add_listener_local()
                        .removed(move || {
                            (*state).borrow_mut().remove_node(id);
                        })
                        .register();

                    (*proxies)
                        .borrow_mut()
                        .add_proxy(node)
                        .add_listener(listener)
                        .add_listener(remove_listener);
                }
            })
            .register();

        mainloop.run();
        Ok(())
    })
}
