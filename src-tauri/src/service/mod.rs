pub mod tag;

use std::sync::Arc;

pub use tag::TagService;

pub struct ServiceManager {
    tag: Arc<TagService>,
}

impl ServiceManager {
    pub fn tag_service(&self) -> Arc<TagService> {
        self.tag.clone()
    }
}

#[derive(Default)]
pub struct ServiceManagerBuider {
    tag: Option<Arc<TagService>>,
}

impl ServiceManagerBuider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(&self) -> ServiceManager {
        let tag_service = self.tag.clone().unwrap();

        ServiceManager { tag: tag_service }
    }

    pub fn tag(&mut self, s: TagService) -> &Self {
        self.tag = Some(Arc::new(s));
        self
    }
}
