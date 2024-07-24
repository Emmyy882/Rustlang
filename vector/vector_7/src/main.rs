fn main() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(6),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(3.14),
    ];
    println!("row = {:?}", row);
}
