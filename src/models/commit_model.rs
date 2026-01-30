use serde::{Deserialize, Serialize};
use crate::models::task_model::Task;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Commit {
    pub parents: Vec<usize>, 
    pub message: String,  
    pub tasks: Vec<Task>,
}
