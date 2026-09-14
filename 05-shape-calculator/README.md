# 05 — Plugin-style Calculator / Shape Toy

**Start after:** Rust Book ch. 10 & 11 (Generics, Traits, Testing)
**Concepts exercised:** traits, trait objects (`Box<dyn Trait>`), generics, unit tests

## Pitch
Define an `Operation` trait (or `Shape` trait if you'd rather do geometry:
area/perimeter). Implement it for several concrete types. Dispatch through
`Vec<Box<dyn Operation>>` so adding a new operation/shape means adding one
`impl` block, nothing else changes.

## Why it's fun
"Make it do a new thing by adding one impl block" is a satisfying toy, and
it's the first project where trait objects actually earn their keep instead
of feeling like ceremony.

## Suggested shape
- `trait Operation { fn apply(&self, a: f64, b: f64) -> f64; fn name(&self) -> &str; }`
- Implement for `Add`, `Subtract`, `Multiply`, `Divide` (handle divide-by-zero with `Result`)
- `Vec<Box<dyn Operation>>`, loop over them and print each result
- Write `#[test]` functions for each operation, including edge cases

## Stretch goals
- Generic `Shape` trait with `area()`/`perimeter()`, implement for `Circle`, `Rectangle`, `Triangle`
- Read operation + operands from CLI args
