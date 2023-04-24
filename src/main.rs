use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::exit;

use markdown::to_html;

fn main() {
    // Parse command arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: <bin> <filepath>");
        exit(1);
    }

    // Parse the path from the first command line argument
    let file_path = &args[1];
    let mut path = PathBuf::from(file_path);
    println!("Looking for path: {}", file_path);

    // Open the file, and get the contents as a string
    let mut file = File::open(&path).expect("unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("failed to read file contents");

    // Print some debug stuff
    println!("Input:\n{}", contents);
    println!("Converting...\n");

    // Convert to markdown
    let markdown_bytes = to_html(&contents).into_bytes();

    // Make it a HTML file
    path.set_extension("html");

    // Write the new file
    File::create(path)
        .expect("unable to create new file")
        .write_all(&markdown_bytes)
        .expect("unable to create new file");

    // Finish up, and print some more output
    println!("Output:\n{}", String::from_utf8(markdown_bytes).expect("unable to convert bytes from output into string for output, file has been succesfuly written."))
}
