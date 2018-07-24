Spready
=======

Toy spreadsheet written in Rust.

Features
--------

- Simple arithmetic formulas with integers and floats
- Automatic recalculation on every change
- Doesn't crash

Usage
-----

1. Look at code if you want.
2. Don't use in production
3. Unless you want to, in which case feel free to, I'm not your mom.

Example
-------

```rust
extern crate spready;

use spready::Spreadsheet;

fn main() {
    let mut spreadsheet = Spreadsheet::new();
    spreadsheet.enter("A1", 10);
    spreadsheet.enter("A2", 20);
    spreadsheet.enter("A3", 30);

    spreadsheet.enter("B1", "=20");
    spreadsheet.enter("B2", "=30+50");

    spreadsheet.enter("C1", "=A1 * 2");
    spreadsheet.enter("C2", "=A1 + A2 * 1.5");
    spreadsheet.enter("C3", "=A4 + 20");

    for (reference, cell) in spreadsheet.cells() {
        println!("{} | {} | {:?}", reference, cell.input, cell.value);
    }
}

```
