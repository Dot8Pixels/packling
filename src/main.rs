use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use walkdir::WalkDir;
use zip::write::FileOptions;

fn main() -> zip::result::ZipResult<()> {
    // Get user input for the path
    println!("Please enter the path to the directory you want to zip:");
    let mut input_path = String::new();
    io::stdin()
        .read_line(&mut input_path)
        .expect("Failed to read input");
    let input_path = input_path.trim();

    // Get user input for included extensions
    println!("Please enter the file extensions to include (comma separated, e.g., txt,jpg):");
    let mut include_exts = String::new();
    io::stdin()
        .read_line(&mut include_exts)
        .expect("Failed to read input");
    let include_exts: Vec<&str> = include_exts.trim().split(',').collect();

    // Get user input for excluded extensions
    println!("Please enter the file extensions to exclude (comma separated, e.g., log,tmp):");
    let mut exclude_exts = String::new();
    io::stdin()
        .read_line(&mut exclude_exts)
        .expect("Failed to read input");
    let exclude_exts: Vec<&str> = exclude_exts.trim().split(',').collect();

    // Create zip file
    let zip_file = File::create("output.zip").expect("Could not create zip file");
    let mut zip = zip::ZipWriter::new(zip_file);

    let options = FileOptions::<()>::default().compression_method(zip::CompressionMethod::Deflated);

    // Read the directory and add files to the zip archive, including subdirectories
    let path = Path::new(&input_path);
    if path.is_dir() {
        for entry in WalkDir::new(path) {
            let entry = entry.expect("Could not read entry");
            let file_path = entry.path();
            if file_path.is_file() {
                if let Some(extension) = file_path.extension() {
                    let ext_str = extension.to_str().unwrap();
                    if (!include_exts.is_empty() && !include_exts.contains(&ext_str))
                        || exclude_exts.contains(&ext_str)
                    {
                        continue;
                    }
                } else {
                    continue;
                }

                let name = file_path.strip_prefix(path).unwrap().to_str().unwrap();
                let mut f = File::open(file_path).expect("Could not open file");
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer).expect("Could not read file");
                zip.start_file(name, options)?;
                zip.write_all(&buffer)?;
            }
        }
    } else {
        println!("The provided path is not a directory");
    }

    zip.finish()?;
    println!("Files have been successfully zipped into output.zip");

    Ok(())
}
