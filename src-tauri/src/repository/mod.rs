pub mod sku;
pub mod sqlx;
pub mod tag;

pub use tag::TagRepository;

use std::sync::Arc;

use self::{sku::SkuRepository, tag::TagRepository1};

pub struct RepoManager {
    tag: Arc<TagRepository>,
}

impl RepoManager {
    pub fn tag(&self) -> Arc<TagRepository> {
        self.tag.to_owned()
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

pub struct RepoManager2 {
    tag: Arc<dyn TagRepository1 + Send + Sync>,
    sku: Arc<dyn SkuRepository + Send + Sync>,
}

impl RepoManager2 {
    pub fn tag(&self) -> Arc<dyn TagRepository1> {
        self.tag.to_owned()
    }

    pub fn sku(&self) -> Arc<dyn SkuRepository> {
        self.sku.to_owned()
    }
}

#[derive(Default)]
pub struct RepoManagerBuider2 {
    tag: Option<Arc<dyn TagRepository1 + Send + Sync>>,
    sku: Option<Arc<dyn SkuRepository + Send + Sync>>,
}

impl RepoManagerBuider2 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(&self) -> RepoManager2 {
        let tag = self.tag.clone().unwrap();
        let sku = self.sku.clone().unwrap();

        RepoManager2 { tag, sku }
    }

    pub fn tag<T>(&mut self, repo: T) -> &mut Self
    where
        T: TagRepository1 + 'static + Send + Sync,
    {
        self.tag = Some(Arc::new(repo));
        self
    }

    pub fn sku<T>(&mut self, repo: T) -> &mut Self
    where
        T: SkuRepository + 'static + Send + Sync,
    {
        self.sku = Some(Arc::new(repo));
        self
    }
}
