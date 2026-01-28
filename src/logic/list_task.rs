use crate::storage::{
    head::{read_head_branch, read_branch_commit},
    commit::load_commit,
};

pub fn list_tasks() {
    // 1️⃣ Get current branch
    let branch = match read_head_branch() {
        Ok(b) => b,
        Err(_) => {
            println!("Repository not initialized.");
            return;
        }
    };

    // 2️⃣ Get commit number of branch
    let commit_number = match read_branch_commit(&branch) {
        Ok(c) => c,
        Err(_) => {
            println!("No tasks yet.");
            return;
        }
    };

    // 3️⃣ Load commit
    let commit = match load_commit(commit_number) {
        Ok(c) => c,
        Err(_) => {
            println!("Failed to load latest commit.");
            return;
        }
    };

    println!("Tasks:\n");

    for task in commit.tasks {
        let status = if task.completed { "✓" } else { " " };
        println!("[{}] {}. {}", status, task.id, task.text);
    }
}
