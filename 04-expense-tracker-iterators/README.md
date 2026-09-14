# 04 — Refactor: Expense Tracker with Iterators

**Start after:** Rust Book ch. 13 (Closures/Iterators)
**Prereq:** Project 02 (this is a revisit, not a fresh build)
**Concepts exercised:** `filter`, `map`, `fold`, iterator chains, closures

## Pitch
Don't build something new — copy or reopen project 02 and rewrite the
totals/filtering logic using iterator chains instead of for-loops.

## Why it's fun
This is the checkpoint where Rust idioms click. Watching your own ugly loop
collapse into one clean iterator chain is more satisfying than starting
fresh, because you already know the "before."

## Suggested shape
- Replace manual `for` + `HashMap::entry` totaling with `fold`
- Replace manual filtering loops with `.filter(...)`
- Try writing category totals as one chained expression, no intermediate `mut` vars
- Compare line count and readability against the ch. 9 version — keep both if useful

## Stretch goals
- Sort categories by total spend using `sort_by`/`sort_by_key`
- Top-N categories via `.take(n)` after sorting
