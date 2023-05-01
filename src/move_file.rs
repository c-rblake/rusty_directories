use std::fs;
use std::path::Path;

fn move_files(src: &str, dest: &str){
    let src_path = Path::new(src);
    let dest_path = Path::new(dest);

    if !src_path.exists(){
     eprintln!("Source path does not exist");
     return;
    }

    if !dest_path.exists(){
        // mkdir -p in Bash
        fs::create_dir_all(dest_path).expect("Failed to create directory");
    }

    let dest_file_path = dest_path.join(src_path.file_name().unwrap());
    // This method returns an Option<&OsStr> containing the file name of the source file path (src_path). If the path terminates in .., has no file name, or is empty, it returns None
    // unwrap(): This method is called on the Option<&OsStr> returned by file_name(). It returns the contained value (the file name) if the Option is Some. If it's None, the program will panic.
    // Bash var=/path/to/file.txt -> file.txt then (basename var)


    fs::rename(&src_path, &dest_file_path).expect("Failed to move file");
    // like bash move command
    println!("Moved file from {} to {}", src, dest);


}

fn main() {
    move_files("stocks.xml", "xml_files")
}