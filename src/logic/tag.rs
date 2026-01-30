use std::fs;
use crate::storage::head::{read_head_branch, read_branch_commit};

pub fn create_tag(name: &str) {
    let branch = read_head_branch().unwrap();
    let commit = read_branch_commit(&branch).unwrap();

    let path = format!(".cotask/tags/{}", name);

    if fs::metadata(&path).is_ok() {
        println!("Tag '{}' already exists.", name);
        return;
    }

    fs::write(&path, commit.to_string()).unwrap();

    println!("Tag '{}' created at commit {}", name, commit);
}

pub fn read_tag_commit(name: &str) -> Option<usize> {
    let path = format!(".cotask/tags/{}", name);

    match std::fs::read_to_string(path) {
        Ok(c) => c.trim().parse().ok(),
        Err(_) => None,
    }
}


pub fn list_tags() {
    match fs::read_dir(".cotask/tags") {
        Ok(entries) => {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    println!("{}", name);
                }
            }
        }
        Err(_) => println!("No tags found."),
    }
}
