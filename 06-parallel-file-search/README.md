# 06 — Parallel File Search / Scraper

**Start after:** Rust Book ch. 16 (Concurrency)
**Prereq:** Project 03 (this parallelizes it)
**Concepts exercised:** `std::thread`, `mpsc` channels, shared state

## Pitch
Take project 03 (log/journal search) and parallelize the search across
files using threads and channels — one thread per file (or a small pool),
results sent back over an `mpsc` channel and collected in `main`.

Alternative: a tiny concurrent web scraper — fetch a handful of URLs in
parallel instead of files.

## Why it's fun
This is where Rust's ownership rules stop being annoying and start visibly
saving you from data races — a genuine "wow, it just works" moment compared
to how careful you'd have to be in other languages.

## Suggested shape
- Spawn one thread per file (or a fixed-size pool for many files)
- Each thread searches its file, sends `(filename, Vec<Match>)` over `mpsc::channel`
- Main thread collects and prints results as they arrive
- Compare wall-clock time against the single-threaded version from project 03

## Stretch goals
- Bound concurrency with a real thread pool (a preview of ch. 20's final project)
- `Arc<Mutex<_>>` for a shared running total instead of channels, just to feel the difference
