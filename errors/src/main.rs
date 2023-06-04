use std::fs::File;
use std::io::{self, Read};
fn main() {
    // let file_result = File::open("hello.txt");
    // let file = match file_result {
    //     Ok(f) => f,
    //     Err(e) => panic!("error: {}", e),
    // };
    // let file2 = File::open("hello.txt").expect("");
    let c = last_char_of_first_line(String::from("sad asd").as_str());

    println!(
        "{}",
        match c {
            Some(char) => char,
            _ => ' ',
        }
    );
    let u = read_username_from_file();
    println!(
        "{:?}",
        match u {
            Ok(u) => u,
            Err(u) => u.to_string(),
        }
    )
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.tx")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn last_char_of_first_line(t: &str) -> Option<char> {
    t.lines().next()?.chars().last()
}
