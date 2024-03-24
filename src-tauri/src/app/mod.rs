use std::sync::Arc;

use crate::service::ServiceManager;

pub struct AppContext {
    services: Arc<ServiceManager>,
}

impl AppContext {
    pub fn new(services: ServiceManager) -> Self {
        let s = Arc::new(services);
        AppContext { services: s }
    }

    pub fn services(&self) -> Arc<ServiceManager> {
        self.services.clone()
    }
}
