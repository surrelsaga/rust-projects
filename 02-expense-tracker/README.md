# 02 — Budget / Expense Tracker (CLI)

**Start after:** Rust Book ch. 9 (Collections + Error Handling)
**Concepts exercised:** `Vec`, `HashMap`, `Result`/`?`, file I/O, parsing untrusted input

## Pitch
A CLI tool that reads expenses from a file (your own bank CSV export,
anonymized, or made-up data), stores them as `Vec<Expense>`, and totals them
per category with a `HashMap`.

## Why it's fun
A tool you'd actually use. Error handling stops being abstract — a malformed
CSV row is a real failure you have to handle instead of `.unwrap()`.

## Suggested shape
- `struct Expense { date: String, category: String, amount: f64 }`
- Parse each line with `Result`, propagate errors with `?`, don't panic on bad rows
- `HashMap<String, f64>` for per-category totals
- Print a summary report to stdout

## Stretch goals
- Monthly breakdown, not just category totals
- A simple budget limit per category with a warning if exceeded
- Support both CSV and a simpler custom format
- any custom format of csv -> read the category first then build struct on it

## Later revisit
Project 04 comes back to this exact codebase after ch. 13 and rewrites the
totals/filtering logic with iterator chains instead of for-loops.


## Design decision
- Loading file 
if not exist, panic (not creating a csv because this CLI program is suppose to read from a csv, user don't ever see the code) +
printline if reading file succeeded

- all csv file that is passed into the program must has the same shape
-> so the Expense struct shape can work (for now, if develop further, can read the category first)

- Step 01: parsing and validate data to put inside a Vector => for any field that the data is not valid, panic -> immediately report back to user

- Step 02: cleaned data in Vector => loop over this clean data and build a hash map

- Step 03: From hashmap -> print in terminal


## Lesson learnt

- only the last expression will be evaluated to return a value for a function so no need `;`

- in order to `parse` a string to a number (any i32, u16, f32,..) using `.parse()`, the string MUST NOT INCLUDE quotes. Not "42", just 42

-> in source code, must add `""` so compiler knows where does the string start and end but in the case to parse from a string to number (e.g: taking from input), quotes cannot be included

-> solution: `.trim_matches(pattern)`, this will remove leadind/trailing characters `pattern`

## Structure

src/
  main.rs      // wiring only: args → load → build → report
  expense.rs   // struct Expense
  loader.rs    // read_content_from_file, parse_fields_from_row, build_expenses_from_csv
  report.rs    // build_total_amount
  ui.rs        // pause_and_clear (already there) + print_report
