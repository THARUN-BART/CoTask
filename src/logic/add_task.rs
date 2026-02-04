use crate::models::{commit_model::Commit, task_model::Task};
use crate::storage::{
    commit::{load_commit, save_commit},
    head::{read_branch_commit, read_head_branch, write_branch_commit},
};

pub fn add_task(text: &str) {
    //  Get current branch
    let branch = match read_head_branch() {
        Ok(b) => b,
        Err(_) => {
            println!("Repository is not initialized");
            return;
        }
    };

    // Get current commit
    let head_commit = read_branch_commit(&branch).unwrap_or(0);

    let mut tasks = if head_commit == 0 {
        Vec::new()
    } else {
        match load_commit(head_commit) {
            Ok(commit) => commit.tasks,
            Err(_) => {
                println!("Failed to load previous commit");
                return;
            }
        }
    };

    let new_id = tasks.len() + 1;

    tasks.push(Task {
        id: new_id,
        text: text.to_string(),
        completed: false,
    });

    let new_commit_number = head_commit + 1;
    let message = format!("Added task '{}'", text);

    let new_commit = Commit {
        parents: if head_commit == 0 {
            vec![]
        } else {
            vec![head_commit]
        },
        message,
        tasks,
    };

    if save_commit(new_commit_number, &new_commit).is_err() {
        println!("Failed to save commit.");
        return;
    }

    if write_branch_commit(&branch, new_commit_number).is_err() {
        println!("Failed to update branch.");
        return;
    }

    println!("Task added in commit {}", new_commit_number);
}

pub fn mark_done(id: usize) {
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
            println!("No tasks found!");
            return;
        }
    };

    let mut commit = match load_commit(head_commit) {
        Ok(c) => c,
        Err(_) => {
            println!("Failed to load commit.");
            return;
        }
    };

    // Find task
    let task = match commit.tasks.iter_mut().find(|t| t.id == id) {
        Some(t) => t,
        None => {
            println!("Task with id {} not found.", id);
            return;
        }
    };

    if task.completed {
        println!("Task {} is already completed.", id);
        return;
    }

    task.completed = true;

    let new_commit_number = head_commit + 1;
    let message = format!("Marked task {} as done", id);
    let new_commit = Commit {
        parents: vec![head_commit],
        message,
        tasks: commit.tasks,
    };

    if save_commit(new_commit_number, &new_commit).is_err() {
        println!("Failed to save commit.");
        return;
    }

    if write_branch_commit(&branch, new_commit_number).is_err() {
        println!("Failed to update branch.");
        return;
    }

    println!("Task {} marked as done in commit {}", id, new_commit_number);
}
