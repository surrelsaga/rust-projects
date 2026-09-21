use std::fs::File;
use std::io::{self, Read};

fn read_content_from_file(file_path: &str) -> Result<String, io::Error> {
    // Propagating error to caller

    let mut csv_file = File::open(file_path)?; // return Err here (if fail)
    let mut contents = String::new();
    csv_file.read_to_string(&mut contents)?; // return Err here (if fail)
    Ok(contents)
}

fn main() {
    // read from csv (panic - stops the program immediate if file not exist)
    let contents = read_content_from_file("expenses.csv")
                                .expect("could not read expenses.csv");
    println!("Loaded file successfully");

    // check the string content shape after loaded and read
    println!("{contents}");
}
