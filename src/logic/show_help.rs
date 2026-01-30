pub fn show_help() {
    println!("cotask — Versioned Task Manager (Mini Git for Tasks)\n");

    println!("Usage:");
    println!("  cotask <command> [arguments]\n");

    println!("Core Commands:");
    println!("  init                         Initialize repository");
    println!("  add <text>                   Add new task");
    println!("  list                         List current tasks");
    println!("  done <id>                    Mark task as completed\n");

    println!("History & Navigation:");
    println!("  log                          Show commit history");
    println!("  checkout <id|branch|tag>     Switch commit, branch, or tag");
    println!("  diff <c1> <c2>               Compare two commits\n");

    println!("Branching & Releases:");
    println!("  branch <name>                Create branch");
    println!("  branches                     List branches");
    println!("  branch-delete <name>         Delete branch");
    println!("  tag <name>                   Create tag at current commit");
    println!("  tags                         List tags\n");

    println!("History Manipulation:");
    println!("  merge <branch>               Merge branch into current");
    println!("  rebase <branch>              Reapply commits onto target branch");
    println!("  revert <commit>              Revert to commit state\n");

    println!("Maintenance:");
    println!("  stash                        Save working state");
    println!("  stash-pop                    Apply latest stash");
    println!("  gc                           Clean unreachable commits");
    println!("  export                       Backup full repository state");
    println!("  import <file>                Restore repository from backup\n");

    println!("Example:");
    println!("  cotask add \"Learn Rust\"");
}
