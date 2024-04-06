use super::Tag;
use crate::{repository::tag::TagRepository1, Result};

pub async fn get_all_tag(repository: &dyn TagRepository1) -> Result<Vec<Tag>> {
    repository.find_all().await
}
