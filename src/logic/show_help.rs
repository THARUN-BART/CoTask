pub fn show_help() {
    println!("cotask — Versioned Task Manager\n");

    println!("Usage:");
    println!("  cotask <command> [arguments]\n");

    println!("Commands:");
    println!("  init                   Initialize repository");
    println!("  add <text>             Add new task");
    println!("  list                   List current tasks");
    println!("  done <id>              Mark task as completed");
    println!("  log                    Show commit history");
    println!("  checkout <id|branch>   Switch commit or branch");
    println!("  branch <name>          Create branch");
    println!("  branches               List branches");
    println!("  branch-delete <name>   Delete branch");
    println!("  merge <branch>         Merge branch");
    println!("  rebase <branch>        Reapply commits onto target branch");
    println!("  revert <commit>        Revert to commit state");
    println!("  diff <c1> <c2>         Compare two commits");
    println!("  gc                     Clean unreachable commits\n");

    println!("Example:");
    println!("  cotask add \"Learn Rust\"");
}
