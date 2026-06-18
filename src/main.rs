use nix::mount::{MntFlags, MsFlags, mount, umount2};
use nix::unistd::{chdir, pivot_root};
use std::env;
use std::fmt::format;
use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};

fn main() {
    let _args: Vec<String> = env::args().collect();
    pivot_root_fs(&PathBuf::from("/home/drcool/programming/new_root")).unwrap();
}

fn pivot_root_fs(root_path: &Path) -> Result<(), String> {
    //What will happen when if mounted path without isolation
    mount(
        Some(root_path),
        root_path,
        None::<&str>,
        MsFlags::MS_BIND | MsFlags::MS_REC,
        None::<&str>,
    )
    .map_err(|e| format!("Failed to mount new root path {}", e))?;

    let old_root = root_path.join(".old_root");

    if !old_root.exists() {
        fs::create_dir_all(&old_root)
            .map_err(|e| format!("Failed to create a dir to store old root {}", e))?;
    }

    pivot_root(root_path, &old_root).map_err(|e| format!("Failed to pivot root {}", e))?;

    Ok(())
}
