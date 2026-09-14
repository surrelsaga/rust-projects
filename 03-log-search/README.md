# 03 — Personal Log / Journal Search Tool

**Start after:** Rust Book ch. 12 (the book's own minigrep project)
**Concepts exercised:** CLI args, file I/O, `Result`, modules

## Pitch
The book has you build `minigrep` in this chapter — same skills, but instead
of reimplementing grep, build a search tool over your own text files (a
journal, notes, logs). Search by keyword, optionally by date if your files
are dated.

## Why it's fun
More motivating output than grep-cloning: you're searching your own stuff
and actually get a useful hit.

## Suggested shape
- Follow the book's minigrep structure (args → config → search fn → main)
- Swap the "search a single file for a string" logic for "search all files
  in a directory, print filename + line + match"
- Case-insensitive option (the book already covers this)

## Stretch goals
- Search multiple keywords (AND/OR)
- Highlight the match in the printed line
- Sort results by file date

## Later revisit
Project 06 parallelizes this across files with threads once you've covered
concurrency (ch. 16).
