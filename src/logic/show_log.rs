use std::collections::HashSet;

use crate::storage::{
    commit::load_commit,
    head::{read_branch_commit, read_head_branch},
};

pub fn show_log() {
    let branch = match read_head_branch() {
        Ok(b) => b,
        Err(_) => {
            println!("Repository not initialized.");
            return;
        }
    };

    let head_commit = match read_branch_commit(&branch) {
        Ok(c) => c,
        Err(_) => {
            println!("No commits yet.");
            return;
        }
    };

    println!("Commit History (branch: {}):\n", branch);

    let mut stack = vec![head_commit];
    let mut visited = HashSet::new();

    while let Some(commit_number) = stack.pop() {
        if visited.contains(&commit_number) {
            continue;
        }
        visited.insert(commit_number);

        match load_commit(commit_number) {
            Ok(commit) => {
                println!("Commit {}", commit_number);
                println!("Message: {}", commit.message);

                for task in &commit.tasks {
                    let status = if task.completed { "✓" } else { " " };
                    println!("[{}] {} - {}", status, task.id, task.text);
                }

                println!("----------------------");

                for parent in commit.parents {
                    stack.push(parent);
                }
            }
            Err(_) => println!("Failed to load commit {}", commit_number),
        }
    }
}
