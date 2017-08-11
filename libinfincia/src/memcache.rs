/* Infincia, Copyright 2011-2017 Stephen Oliver */
/* Version 1.0 */

use std::collections::HashMap;
use std::path::{PathBuf};
use std::fs::File;
use std::io::Read;

use walkdir::WalkDir;


// Use globbing
lazy_static! {
    pub static ref STATIC: HashMap<PathBuf, Vec<u8>> = {
        let map = load();

        map
    };
}

fn load() -> HashMap<PathBuf, Vec<u8>> {
    let mut folder_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // crate root
    folder_path.pop();
    // load all dist assets
    folder_path.push("dist/");

    let mut map: HashMap<PathBuf, Vec<u8>> = HashMap::new();

    for item in WalkDir::new(&folder_path).into_iter().filter_map(|e| e.ok()) {

        // We only care about actual files

        let full_path = item.path();
        if &full_path == &folder_path {
            continue; // don't bother doing anything more for the root directory of the folder
        }
        let relative_path = full_path.strip_prefix(&folder_path).expect("failed to unwrap relative path");


        let md = match ::std::fs::metadata(&full_path) {
            Ok(m) => m,
            Err(_) => { continue },
        };

        let is_file = md.file_type().is_file();

        if is_file {
            let mut file = File::open(full_path).unwrap();

            let mut buf: Vec<u8> = Vec::new();

            let _ = file.read_to_end(&mut buf);

            map.insert(relative_path.to_owned(), buf);

        }
    }

    map
}