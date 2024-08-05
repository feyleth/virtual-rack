use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use pipewire::proxy::{Listener, ProxyListener, ProxyT};
use tracing::{error, info};

pub struct Proxies {
    proxies_t: HashMap<u32, Box<dyn ProxyT>>,
    listeners: HashMap<u32, Vec<Box<dyn Listener>>>,
    proxies_weak: Weak<RefCell<Self>>,
}

pub struct Proxie<'a>(&'a mut Vec<Box<dyn Listener>>);
impl<'a> Proxie<'a> {
    pub fn add_listener(self, listener: impl Listener + 'static) -> Self {
        self.0.push(Box::new(listener));
        self
    }
}

impl Proxies {
    pub fn new() -> Rc<RefCell<Self>> {
        let proxies = Self {
            proxies_t: HashMap::new(),
            listeners: HashMap::new(),
            proxies_weak: Weak::new(),
        };
        let ret = Rc::new(RefCell::new(proxies));
        ret.borrow_mut().proxies_weak = Rc::downgrade(&ret);
        ret
    }

    pub fn add_proxy(&mut self, proxy_t: impl ProxyT + 'static) -> Proxie<'_> {
        let proxy_id = {
            let proxy = proxy_t.upcast_ref();
            proxy.id()
        };

        let listener = self.create_remove_lithener(&proxy_t);
        self.proxies_t.insert(proxy_id, Box::new(proxy_t));

        let v = self.listeners.entry(proxy_id).or_default();
        Proxie(v).add_listener(listener)
    }

    fn create_remove_lithener(&self, proxy_t: &dyn ProxyT) -> ProxyListener {
        let proxy_id = proxy_t.upcast_ref().id();
        let proxy_weark = self.proxies_weak.clone();
        proxy_t
            .upcast_ref()
            .add_listener_local()
            .removed(move || {
                if let Some(proxies) = proxy_weark.upgrade() {
                    proxies.borrow_mut().remove(proxy_id);
                }
            })
            .register()
    }

    fn remove(&mut self, proxy_id: u32) {
        self.proxies_t.remove(&proxy_id);
        self.listeners.remove(&proxy_id);
    }
}
