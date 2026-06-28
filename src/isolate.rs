use nix::sched::{CloneFlags, unshare};

pub fn isolate_root() -> Result<(), String> {
    let flags = CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWUTS;

    unshare(flags).map_err(|e| format!("Failed to isolate process {}", e))?;

    println!("Successfully isolated into new namespaces!");
    Ok(())
}
