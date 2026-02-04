use crate::models::export_model::RepoExport;
use crate::storage::commit::save_commit;
use std::fs;

pub fn import_repo(file: &str) {
    let data = fs::read_to_string(file).unwrap();
    let repo: RepoExport = serde_json::from_str(&data).unwrap();

    fs::create_dir_all(".cotask/commits").unwrap();
    fs::create_dir_all(".cotask/refs").unwrap();
    fs::create_dir_all(".cotask/tags").unwrap();

    // Restore commits
    for (num, commit) in repo.commits {
        save_commit(num, &commit).unwrap();
    }

    // Restore branches
    for (name, commit) in repo.branches {
        fs::write(format!(".cotask/refs/{}", name), commit.to_string()).unwrap();
    }

    // Restore tags
    for (name, commit) in repo.tags {
        fs::write(format!(".cotask/tags/{}", name), commit.to_string()).unwrap();
    }

    fs::write(".cotask/HEAD", repo.head).unwrap();

    println!("Repository imported successfully.");
}
