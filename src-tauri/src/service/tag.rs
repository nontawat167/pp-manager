use std::sync::Arc;

use crate::{domain::tag::NewTag, repository::RepoManager};

pub struct TagService {
    repos: Arc<RepoManager>,
}

impl TagService {
    pub fn new(repositories: RepoManager) -> Self {
        Self {
            repos: Arc::new(repositories),
        }
    }

    pub async fn get_all_tag(&self) -> Vec<NewTag> {
        let tag_repo = self.repos.tag();
        tag_repo.find_all().await.unwrap()
    }
}
