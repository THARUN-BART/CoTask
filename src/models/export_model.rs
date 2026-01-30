use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::models::commit_model::Commit;

#[derive(Serialize, Deserialize)]
pub struct RepoExport {
    pub commits: HashMap<usize, Commit>,
    pub branches: HashMap<String, usize>,
    pub tags: HashMap<String, usize>,
    pub head: String,
}
