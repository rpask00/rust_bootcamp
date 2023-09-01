use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let content = file_content("./assets/random_import.csv");

    match content {
        Ok(c) => println!("{}", c),
        Err(e) => println!("{}", e),
    }

}

fn file_content(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?; // unwrap value, or return Error immediately
    let mut content = String::new();


    file.read_to_string(&mut content)?; // unwrap value, or return Error immediately

    return Ok(content);
}


