use serde::*; // 1.0.130
use serde::ser::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;


struct Item {
    foo: u64,
    bar: u64,
}

fn main() {
    let items_string: &str = "[[items]]\nfoo = 10\nbar = 100\n\n[[items]]\nfoo = 12\nbar = 144\n";
    let items_table: HashMap<String, Vec<Item>> = from_str(items_string).unwrap();
    let items: &[Item] = &items_table["items"];

    println!("{:?}", items_table);
    println!("{:?}", items);
}