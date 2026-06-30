/*
 *generate random id which will be linked to container
 *we can use that to created new roots for that container
 *and manage that containers other data
 */

use std::fs;
use std::io::Write;
use std::path::Path;

//For now i am going to pass path later going to
//find a way to use path defined at single place
//everywhere
pub fn create_id_map(container_name: &str, data_path: &Path) -> std::io::Result<()> {
    let meta_data_file_path = data_path.join("container_map.txt");
    let mut metadata_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(meta_data_file_path)?;

    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("back to past")
        .as_millis();

    print!("Current timestamp {}", now_ms);

    writeln!(metadata_file, "{} {}", container_name, now_ms)?;

    Ok(())
}
