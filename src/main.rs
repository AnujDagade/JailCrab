mod isolate;
mod metadata;

use std::env;
use std::fs;
use std::path::Path;

use crate::metadata::create_id_map;

fn main() {
    /*
     * figure out how to use single path with join
     * the hell with borrow checker
     */
    let data_path = Path::new("/var/lib/jail_crab");

    let new_root = data_path.join("roots");

    /*
     * later this fun is going to get called after creating a container first 22:02
     *
     */
    create_id_map("tmp_name", data_path);

    if !new_root.exists() {
        fs::create_dir_all(&new_root)
            .map_err(|e| format!("Failed to create new roots dir {}", e))
            .unwrap();
    }

    let _args: Vec<String> = env::args().collect();
    isolate::isolate_root().unwrap();
    isolate::pivot_root_fs(&new_root).unwrap();
}
