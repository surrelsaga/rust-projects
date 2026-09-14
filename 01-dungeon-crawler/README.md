# 01 — Text Adventure / Combat Sim

**Start after:** Rust Book ch. 7 (you already have everything you need)
**Concepts exercised:** structs, enums, `match`, ownership, control flow, stdin input

## Pitch
A tiny dungeon crawler. `Player` and `Monster` structs hold HP/stats, an
`Action` enum (`Attack`, `Defend`, `Flee`) drives a turn loop, `match`
resolves combat outcomes. Read player choices from stdin each turn.

## Why it's fun
First checkpoint where "concepts I just learned" turns into "a thing that
reacts to what I type." Win/lose states are immediate and visible.

## Suggested shape
- `struct Player { hp: i32, attack: i32, ... }`, same for `Monster`
- `enum Action { Attack, Defend, Flee }`
- Game loop: print state → read action → `match` → apply damage → check win/lose
- Multiple monster types (just different struct instances / a small `Vec<Monster>` roster)

## Game loop (confirmed)
1. Print player HP and monster HP.
2. Player picks an action from stdin (`attack` / `defend` / `flee`).
3. Monster's action is chosen randomly each turn (needs the `rand` crate —
   first external dependency, add it in `Cargo.toml`).
4. Resolve both actions with `match`:
   - `Attack` vs anything but `Defend` → full damage
   - `Attack` vs `Defend` → half damage (defender takes reduced damage that turn)
   - `Defend` deals no damage itself
5. `Flee` isn't guaranteed — roll it too (e.g. 50% chance via `rand`). On
   success, end the encounter immediately with no winner. On failure, the
   flee attempt does nothing and the monster's action still lands this turn.
6. Whoever's HP hits 0 first loses.
7. Repeat from step 1 until there's a winner or the player has fled successfully.

## Stretch goals
- Inventory (`Vec<Item>`)
- Multiple rooms (simple state machine over an enum)
- Save/load run state to a file (ties into ch. 9 later)

## Later revisit
Project 07 extends this into a multiplayer server once you've covered
concurrency and the book's web server chapter.
