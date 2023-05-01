use std::env;
use std::fs;

fn delete_file(file_path: &str) {
    match fs::remove_file(file_path) {
        // This function takes the file path as input and tries to delete the file using fs::remove_file
        // Example
        Ok(_) => println!("File deleted {}", file_path),
        Err(e) => println!("Error delting file {}: {}", file_path, e),
    }



    
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    // Skipping the program name like Bash opargs shift 

    if args.is_empty() {
        eprintln!("No file path provided on the command line");
        return;
    }

    let file_path = &args[0];
    delete_file(file_path)
}

