use std::collections::HashMap;

mod ui;
mod expense;
use expense::Expense;
mod loader;
use loader::{read_content_from_file, build_expenses_from_csv};
mod report;
use report::{build_total_amount};

fn main() {
    use std::env;

    // csv file will be loaded into the program during cargo run
    // cargo run -- data/plain.csv (e.g)
    let file_path: String = env::args().nth(1).expect("To load csv, use: cargo run -- <file.csv>");

    // read from csv (panic - stops the program immediate if file not exist)
    let contents: String = read_content_from_file(&file_path)
                                .expect("could not read expenses.csv");
    println!("Loaded file successfully");

    // check the string content shape after loaded and read
    // println!("{contents}");

    // get the line containing all the categories
    // this will be used later file csv file with different categories
    // let firstline: &str = contents.split('\n').next().unwrap();
    // println!("{firstline}");

    let expenses: Vec<Expense> = build_expenses_from_csv(contents);
    println!("Parsing completed.");

    let category_to_amount: HashMap<&String, f64> = build_total_amount(&expenses);

    ui::pause_and_clear();
    ui::print_report(&category_to_amount);
}
