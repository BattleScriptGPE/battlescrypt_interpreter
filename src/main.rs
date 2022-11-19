use std::fs::File;
use std::io::Read;

use log::info;

fn main() {
    info!("Entering RUST interpreter.");


    let file_path: String = String::from("./stash.amn");

    println!("File Path {}", file_path);

    let content: String = read_file_from_path(file_path);

    println!("Data retrieved -> \n{}", content);
}

fn read_file_from_path(path: String) -> String {
    let mut file: File = File::open(path)
    .expect("File not found");

    let mut data: String = String::new();

    file.read_to_string(&mut data)
    .expect("Error while reading file");

    data
}

fn interpreter() {

}
