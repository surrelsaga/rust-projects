use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;

#[derive(Debug)]
struct Expense {
    date: String,
    category: String,
    amount: f64,
}

// Error handling practice from chap 09: write functions that return the error
// how to respond to the error is up to the caller

fn read_content_from_file(file_path: &str) -> Result<String, io::Error> {
    // Propagating error to caller

    let mut csv_file = File::open(file_path)?; // return Err here (if fail)
    let mut contents = String::new();
    csv_file.read_to_string(&mut contents)?; // return Err here (if fail)
    Ok(contents)
}

// not using &Vec<&str> since the raw_fields is no longer needed after the function
fn parse_fields_from_row(raw_fields: Vec<&str>) -> Result<Expense, String> {
    if raw_fields.len() < 3 {
        return Err( format!("expected at least 3 fields, found {}", raw_fields.len()) );
    }

    // validate the amount since that's the only thing we care about for now
    // parse() will based on type annotation and convert to that type from the string
    let cleaned_text = raw_fields[2].trim().trim_matches('"');
    let amount: f64 = match cleaned_text.parse() {
        Ok(number) => number,
        Err(_) => return Err(format!("amount {cleaned_text} is not a valid number.")),
    };

    // if the data passes all validation test, then push to the expenses vector
    let returnExpense = Expense {
        date: String::from(raw_fields[0].trim().trim_matches('"')),
        category: String::from(raw_fields[1].trim().trim_matches('"')),
        amount,
    };

    // only the value from the LAST expression in the function will be returned, so need to add 'return' to the Err above
    Ok(returnExpense)
}

// build a hashmap to store the total amount paied for each category
fn build_total_amount(expenses: &[Expense]) -> HashMap<&String, f64> {
    let mut category_to_amount = HashMap::new();

    for expense in expenses {
        let category: &String = &expense.category;
        let total_amount = category_to_amount.entry(category).or_insert(0.0);

        *total_amount += expense.amount;
    }

    category_to_amount
}

fn print_report(expenses_map: &HashMap<&String, f64>) {
    println!("Expense report here\n ---------------");
    // printing final result to the users
    for (category, total_amount) in expenses_map {
        println!("{category}: ${total_amount}");
    }
}

fn main() {

    // read from csv (panic - stops the program immediate if file not exist)
    let contents = read_content_from_file("expenses.csv")
                                .expect("could not read expenses.csv");
    println!("Loaded file successfully");

    // check the string content shape after loaded and read
    // println!("{contents}");

    // get the line containing all the categories
    // this will be used later file csv file with different categories
    // let firstline: &str = contents.split('\n').next().unwrap();
    // println!("{firstline}");

    let mut expenses: Vec<Expense> = Vec::new();

    for (index, row) in contents.split("\n").enumerate() {
        // first line only contains categories so skip
        if index == 0 {
            continue;
        }

        // signal executing parsing code
        // user only see the output so index matches with the line number
        println!("Parsing line {}...", index);

        // create a vector of &str
        let raw_fields: Vec<&str> = row.split(",").collect();

        // handle parsing errors
        match parse_fields_from_row(raw_fields) {
            Ok(expense) => {
                println!("Parsed successfully.");
                expenses.push(expense);
            },
            Err(reason) => {
                println!("Parsing failed, reason: {}", reason);
            },
        }
    }

    // println!("{:?}", expenses);

    let mut category_to_amount = build_total_amount(&expenses);
    // println!("{:?}", category_to_amount);

    print_report(&category_to_amount);
}
