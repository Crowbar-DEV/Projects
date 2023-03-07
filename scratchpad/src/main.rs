fn main() {
    let mut s: String = String::from("some text");
    let word = first_word(&mut s);
    s.clear();
    println!("{}", word);
}
fn first_word(s: &String) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}
