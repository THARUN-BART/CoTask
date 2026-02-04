use crate::models::commit_model::Commit;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct RepoExport {
    pub commits: HashMap<usize, Commit>,
    pub branches: HashMap<String, usize>,
    pub tags: HashMap<String, usize>,
    pub head: String,
}
