use std::env;
use std::fs;
use std::path::Path;



fn main() {
    //Get command line arguments, skipping the first one which is the program name
    let args: Vec<String> = env::args().skip(1).collect();

    //Check if a path was provided
    if args.is_empty() {
        eprint!("No path provided as a command line argument");
        return;

    }
    let path: &String = &args[0]; // & reference, similar to pinater

    create_directory_if_not_exists(path);

}

fn create_directory_if_not_exists<P: AsRef<Path>>(path: P) {   //  generic function parameter with a trait bound.
    // This means that any type P passed to the function must implement the AsRef<Path> trait.

    let path = path.as_ref();

    if !path.exists() {
        match fs::create_dir(path) {
            Ok(_) => println!("Directory {:?} created", path),
            Err(e) => println!("Error creating directory {:?}: {}", path,  e),
        } 
    }
    else {
        println!("The path {:?} already exists", path);
    }
}
