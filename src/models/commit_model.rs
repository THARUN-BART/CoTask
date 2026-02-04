use crate::models::task_model::Task;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Commit {
    pub parents: Vec<usize>,
    pub message: String,
    pub tasks: Vec<Task>,
}
