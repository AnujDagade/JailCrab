mod isolate;

use isolate::isolate_root;
use nix::libc::MS_REC;
use nix::mount::{MntFlags, MsFlags, mount, umount2};
use nix::unistd::{chdir, pivot_root};
use std::env;
use std::fmt::format;
use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};

fn main() {
    let _args: Vec<String> = env::args().collect();
    isolate_root().unwrap();
    pivot_root_fs(&PathBuf::from("/home/drcool/programming/new_root")).unwrap();
}

fn pivot_root_fs(root_path: &Path) -> Result<(), String> {
    mount(
        None::<&str>,
        "/",
        None::<&str>,
        MsFlags::MS_PRIVATE | MsFlags::MS_REC,
        None::<&str>,
    )
    .map_err(|e| format!("Failed to make root private {}", e))?;

    //What will happen when if mounted path without isolation
    mount(
        Some(root_path),
        root_path,
        None::<&str>,
        MsFlags::MS_BIND | MsFlags::MS_REC | MsFlags::MS_PRIVATE,
        None::<&str>,
    )
    .map_err(|e| format!("Failed to mount new root path {}", e))?;

    let old_root = root_path.join(".old_root");

    if !old_root.exists() {
        fs::create_dir_all(&old_root)
            .map_err(|e| format!("Failed to create a dir to store old root {}", e))?;
    }

    let pid = std::process::id();
    println!("Current PID: {}", pid);

    pivot_root(root_path, &old_root).map_err(|e| format!("Failed to pivot root {}", e))?;

    let curr_dir = env::current_dir().unwrap();

    println!("Current directory: {}", curr_dir.display());

    Ok(())
}
