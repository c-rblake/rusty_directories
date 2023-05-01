use std::fs::File;
use std::io::Write;




fn create_dummy_xml_file(filename: &str) {
    let data = 
    // Looks similar to bash EOF << syntax.
    r#"<?xml version="1.0" encoding="UTF-8"?>
    <stock_market>
    <stock>
    <symbol>AAPL</symbol>
    <company>Apple Inc.</company>
    <price>150.0</price>
    </stock>
    <stock>
    <symbol>GOOGL</symbol>
    <company>Alphabet Inc.</company>
    <price>2800.0</price>
    </stock>
    <stock>
    <symbol>MSFT</symbol>
    <company>Microsoft Corporation</company>
    <price>305.0</price>
    </stock>
    </stock_market>
    "#;

    let mut file = File::create(filename).expect("Unable to create file");
    file.write_all(data.as_bytes()).expect("Unable to write data");

}

fn main() {
    create_dummy_xml_file("stocks.xml");
    print!("Created file stocks.xml"); 
}