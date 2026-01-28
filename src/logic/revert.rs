use crate::storage::{
    head::{read_head_branch, read_branch_commit, write_branch_commit},
    commit::{load_commit, save_commit},
};
use crate::models::commit_model::Commit;

pub fn revert(commit_number: usize) {
    // Read current branch
    let branch = match read_head_branch() {
        Ok(b) => b,
        Err(_) => {
            println!("Repository not initialized.");
            return;
        }
    };

    // Get current commit of branch
    let current_head = match read_branch_commit(&branch) {
        Ok(c) => c,
        Err(_) => {
            println!("Branch commit not found.");
            return;
        }
    };

    if commit_number == 0 {
        println!("Invalid commit number!");
        return;
    }

    // Load target commit
    let old_commit = match load_commit(commit_number) {
        Ok(c) => c,
        Err(_) => {
            println!("Commit {} does not exist.", commit_number);
            return;
        }
    };

    let new_commit_number = current_head + 1;
    let message = format!("Reverted to state of commit {}", commit_number);

    let revert_commit = Commit {
        parents: vec![current_head],
        message,
        tasks: old_commit.tasks,
    };

    if save_commit(new_commit_number, &revert_commit).is_err() {
        println!("Failed to save revert commit.");
        return;
    }

    if write_branch_commit(&branch, new_commit_number).is_err() {
        println!("Failed to update branch.");
        return;
    }

    println!(
        "Reverted branch '{}' to state of commit {} → new commit {}",
        branch, commit_number, new_commit_number
    );
}
