use crate::models::commit_model::Commit;
use crate::storage::{
    commit::{load_commit, save_commit},
    head::{read_branch_commit, read_head_branch, write_branch_commit},
};

pub fn rebase_onto(target_branch: &str) {
    let current_branch = read_head_branch().unwrap();
    let current_head = read_branch_commit(&current_branch).unwrap();
    let target_head = read_branch_commit(target_branch).unwrap();

    if current_branch == target_branch {
        println!("Cannot rebase branch onto itself.");
        return;
    }

    println!("Rebasing '{}' onto '{}'", current_branch, target_branch);

    // Collect commits from current branch (simple linear assumption)
    let mut commits_to_replay = Vec::new();
    let mut ptr = current_head;

    while ptr != target_head {
        let commit = load_commit(ptr).unwrap();
        commits_to_replay.push(commit.clone());

        if commit.parents.is_empty() {
            break;
        }
        ptr = commit.parents[0];
    }

    commits_to_replay.reverse();

    let mut new_parent = target_head;
    let mut new_commit_number = target_head;

    for old_commit in commits_to_replay {
        new_commit_number += 1;

        let new_commit = Commit {
            parents: vec![new_parent],
            message: format!("(rebased) {}", old_commit.message),
            tasks: old_commit.tasks.clone(),
        };

        save_commit(new_commit_number, &new_commit).unwrap();
        new_parent = new_commit_number;
    }

    write_branch_commit(&current_branch, new_commit_number).unwrap();

    println!(
        "Rebase complete. '{}' now at commit {}",
        current_branch, new_commit_number
    );
}
