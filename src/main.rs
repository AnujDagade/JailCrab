mod isolate;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let new_root = Path::new("/var/lib/jail_crab/roots");

    if !new_root.exists() {
        fs::create_dir_all(new_root)
            .map_err(|e| format!("Failed to create new roots dir {}", e))
            .unwrap();
    }

    let _args: Vec<String> = env::args().collect();
    isolate::isolate_root().unwrap();
    isolate::pivot_root_fs(new_root).unwrap();
}
