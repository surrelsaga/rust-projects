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

## Later revisit
Project 04 comes back to this exact codebase after ch. 13 and rewrites the
totals/filtering logic with iterator chains instead of for-loops.
