use std::fs;
use crate::storage::{
    commit::load_commit,
    head::{write_head_branch,read_head_branch}
};

pub fn checkout_commit(commit_number: usize) {
    if commit_number == 0 {
        println!("Invalid commit number.");
        return;
    }

    if load_commit(commit_number).is_err() {
        println!("Commit {} does not exist.", commit_number);
        return;
    }

    // Detached HEAD mode
    if fs::write(".cotask/HEAD", commit_number.to_string()).is_err() {
        println!("Failed to detach HEAD.");
        return;
    }

    println!("HEAD detached at commit {}", commit_number);
}

pub fn checkout_branch(name: &str) {
    let path = format!(".cotask/refs/{}", name);

    if fs::metadata(&path).is_err() {
        println!("Branch '{}' does not exist.", name);
        return;
    }

    if let Ok(current) = read_head_branch() {
        if current == name {
            println!("Already on branch '{}'", name);
            return;
        }
    }

    if write_head_branch(name).is_err() {
        println!("Failed to switch branch.");
        return;
    }

    println!("Switched to branch '{}'", name);
}
