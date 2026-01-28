use std::fs;
use std::io;

pub fn read_head_branch() -> io::Result<String> {
    Ok(fs::read_to_string(".cotask/HEAD")?.trim().to_string())
}

pub fn write_head_branch(branch: &str) -> io::Result<()> {
    fs::write(".cotask/HEAD", branch)?;
    Ok(())
}

pub fn read_branch_commit(branch: &str) -> io::Result<usize> {
    let path = format!(".cotask/refs/{}", branch);
    Ok(fs::read_to_string(path)?.trim().parse().unwrap())
}

pub fn write_branch_commit(branch: &str, commit: usize) -> io::Result<()> {
    let path = format!(".cotask/refs/{}", branch);
    fs::write(path, commit.to_string())?;
    Ok(())
}
