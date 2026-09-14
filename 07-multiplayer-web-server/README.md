# 07 — Web Server Extension (REST API or Multiplayer)

**Start after:** Rust Book ch. 20 (the book's own multithreaded web server project)
**Prereq:** Project 01 (dungeon crawler) and/or Project 02 (expense tracker)
**Concepts exercised:** everything so far — the capstone

## Pitch
The book has you build a thread-pool-based web server. Once that's done,
put a real feature on top instead of leaving it as a toy:

- Serve project 02 (expense tracker) as a small REST API — `GET /expenses`,
  `POST /expenses`, category totals as JSON
- Or turn project 01 (dungeon crawler) into a server multiple people can
  connect to and play against

## Why it's fun
This is the payoff chapter — everything from ownership through concurrency
gets used together on something that actually runs as a service, not a
CLI you run once and forget.

## Suggested shape
- Start from the book's thread-pool server code as-is
- Route requests by path/method (simple string matching is fine, no framework)
- Reuse project 02's `Expense` struct + parsing logic, just swap file I/O
  for HTTP request bodies
- Or reuse project 01's `Player`/`Action`/`match` combat loop, driven by
  incoming connections instead of stdin

## Stretch goals
- Persist state between requests (write back to the file, or an in-memory
  `Arc<Mutex<_>>` store)
- Serve JSON (a minimal hand-rolled serializer is fine — or reach for
  `serde` here, first good excuse to use crates.io per ch. 14)
