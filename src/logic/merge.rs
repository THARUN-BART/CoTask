use std::collections::HashMap;
use crate::storage::{
    head::{read_head_branch, read_branch_commit, write_branch_commit},
    commit::{load_commit, save_commit},
};
use crate::models::commit_model::Commit;
use crate::models::task_model::Task;

pub fn merge(target_commit: usize) {
    // 1️⃣ Get current branch
    let branch = match read_head_branch() {
        Ok(b) => b,
        Err(_) => {
            println!("Repository not initialized.");
            return;
        }
    };

    // 2️⃣ Get current commit of branch (OURS)
    let head_commit = match read_branch_commit(&branch) {
        Ok(c) => c,
        Err(_) => {
            println!("Failed to read branch commit.");
            return;
        }
    };

    // 3️⃣ Load commits
    let current = match load_commit(head_commit) {
        Ok(c) => c,
        Err(_) => {
            println!("Failed to load current commit.");
            return;
        }
    };

    let target = match load_commit(target_commit) {
        Ok(c) => c,
        Err(_) => {
            println!("Target commit does not exist.");
            return;
        }
    };

    // 4️⃣ Merge tasks
    let mut task_map: HashMap<usize, Task> = HashMap::new();

    for task in current.tasks {
        task_map.insert(task.id, task);
    }

    for task in target.tasks {
        task_map
            .entry(task.id)
            .and_modify(|t| {
                t.completed = t.completed || task.completed;
            })
            .or_insert(task);
    }

    let merged_tasks: Vec<Task> = task_map.into_values().collect();

    // 5️⃣ Create merge commit
    let new_commit_number = head_commit + 1;
    let message = format!("Merged commit {}", target_commit);

    let new_commit = Commit {
        parents: vec![head_commit, target_commit],
        message,
        tasks: merged_tasks,
    };

    // 6️⃣ Save commit
    if save_commit(new_commit_number, &new_commit).is_err() {
        println!("Failed to save merged commit.");
        return;
    }

    // 7️⃣ Move branch pointer
    if write_branch_commit(&branch, new_commit_number).is_err() {
        println!("Failed to update branch pointer.");
        return;
    }

    println!(
        "Merged commit {} into branch '{}' → new commit {}",
        target_commit, branch, new_commit_number
    );
}
