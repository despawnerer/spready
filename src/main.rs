extern crate lalrpop_util;
#[macro_use]
extern crate lazy_static;
extern crate arrayvec;
extern crate regex;

mod cell;
mod evaluate;
mod formula;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod grammar;
mod graph;
mod reference;
mod spreadsheet;
mod syntax;
mod value;

use spreadsheet::Spreadsheet;

fn main() {
    let mut spreadsheet = Spreadsheet::new();
    spreadsheet.enter("A1", 10);
    spreadsheet.enter("A2", 20);
    spreadsheet.enter("A3", 30);

    spreadsheet.enter("B1", "=20");
    spreadsheet.enter("B2", "=30+50");

    spreadsheet.enter("C1", "=A1*2");
    spreadsheet.enter("C2", "=A1+A2+A3");
    spreadsheet.enter("C3", "=A4 + 20");

    spreadsheet.enter("F1", "=AB6001");

    for (reference, cell) in &spreadsheet.cells {
        println!("{:?} :: {:?}", reference, cell);
    }

    println!(
        "Order of evaluation: {:?}",
        spreadsheet.dependencies.to_topological_sort()
    );
}
