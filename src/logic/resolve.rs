use crate::models::commit_model::Commit;
use crate::storage::{
    commit::{load_commit, save_commit},
    head::{read_branch_commit, read_head_branch, write_branch_commit},
};
use std::fs;

pub fn resolve(task_id: usize, done: bool) {
    if fs::read_to_string(".cotask/MERGE_CONFLICT").is_err() {
        println!("No merge conflict to resolve");
        return;
    }

    let branch = read_head_branch().unwrap();
    let head = read_branch_commit(&branch).unwrap();
    let mut commit = load_commit(head).unwrap();

    for task in &mut commit.tasks {
        if task.id == task_id {
            task.completed = done;
        }
    }

    let new_commit_number = head + 1;
    let message = format!("Resolved conflict for task {}", task_id);

    let new_commit = Commit {
        parents: vec![head],
        message,
        tasks: commit.tasks,
    };

    save_commit(new_commit_number, &new_commit).unwrap();
    write_branch_commit(&branch, new_commit_number).unwrap();

    fs::remove_file(".cotask/MERGE_CONFLICT").unwrap();

    println!("Conflict resolved → new commit {}", new_commit_number);
}
