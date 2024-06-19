mod webp;
use std::env;
use std::fs;
use std::fs::ReadDir;

fn main() {
    let args: Vec<String> = env::args().collect(); // Collect command line arguments
    let directory = &args[1]; // Grab the argument that contains the directory path
    let paths = fs::read_dir(directory.to_string()).unwrap(); // Read the directory

    match is_directory_empty(directory) {
        Ok(is_empty) => {
            if is_empty {
                println!("{} directory is empty.", directory);
            } else {
                process_image(paths);
            }
        }
        Err(err) => {
            println!("Error when checking if directory is empty: {}", err);
        }
    }
}

fn process_image(paths: ReadDir) {
// Iterate through the list and save a webp version
    for path in paths {
        let file_name: String = path.unwrap().path().display().to_string();
        if file_name.contains(".jpg") {
            let _ = webp::image_to_webp(&file_name);
            println!("Converted: {:?}", file_name)
        } else if file_name.contains(".png") {
            let _ = webp::image_to_webp(&file_name);
            println!("Converted: {:?}", file_name)
        }
    }
}

fn is_directory_empty(directory: &str) -> std::io::Result<bool> {
    let mut entries = fs::read_dir(directory)?;
    let first_entry = entries.next();
    Ok(first_entry.is_none())
}