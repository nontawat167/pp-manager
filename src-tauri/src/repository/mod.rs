pub mod tag;

pub use tag::TagRepository;

use std::sync::Arc;

pub struct RepoManager {
    tag: Arc<TagRepository>,
}

impl RepoManager {
    pub fn tag(&self) -> Arc<TagRepository> {
        self.tag.clone()
    }
}

#[derive(Default)]
pub struct RepoManagerBuider {
    tag: Option<Arc<TagRepository>>,
}

impl RepoManagerBuider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(&self) -> RepoManager {
        let tag_repo = self.tag.clone().unwrap();

        RepoManager { tag: tag_repo }
    }

    pub fn tag(&mut self, t: TagRepository) -> &Self {
        self.tag = Some(Arc::new(t));
        self
    }
}
