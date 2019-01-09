use std::path::Path;
//use std::error::Error;

fn main() {
    let path = Path::new("rust_file.txt");
    let display = path.display();
    println!("Path:{}", display);
}