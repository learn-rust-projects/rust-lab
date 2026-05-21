use std::fs;

fn main() {
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.
                println!("{:?}", entry.file_name());
                println!("{:?}", entry.path());
                println!("{:?}", entry.path().display());
            }
        }
    }
}
