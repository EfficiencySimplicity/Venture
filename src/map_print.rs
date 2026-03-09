use std::fs;

// https://doc.rust-lang.org/book/ch12-02-reading-a-file.html

pub fn print_map() {
    let contents = fs::read_to_string("assets/map-ascii.txt")
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}