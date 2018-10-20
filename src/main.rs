extern crate spready;

use spready::Spreadsheet;

fn main() {
    let mut spreadsheet = Spreadsheet::new();
    spreadsheet.enter("A1", 10);
    spreadsheet.enter("A2", 20);
    spreadsheet.enter("A3", 30);
    spreadsheet.enter("A5", "1.0");
    spreadsheet.enter("A6", 2.5);

    spreadsheet.enter("B1", "=20");
    spreadsheet.enter("B2", "=30+50");

    spreadsheet.enter("C1", "=A1*2");
    spreadsheet.enter("C2", "=A1+A2+A3");
    spreadsheet.enter("C3", "=A4 + 20");

    spreadsheet.enter("D1", "=A1 + A5");
    spreadsheet.enter("D2", "=A5 + A1");
    spreadsheet.enter("D3", "=A1 * A6");
    spreadsheet.enter("D4", "=A1 * 2.0");

    spreadsheet.enter("F1", "=AB6001");
    spreadsheet.enter("F2", "=A1 / 0");

    for (reference, cell) in spreadsheet.cells() {
        println!("{} | {} | {:?}", reference, cell.text(), cell.value);
    }
}
