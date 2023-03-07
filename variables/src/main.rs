use std::io;

fn main() {
    let a: [u32; 5] = [1, 2, 3, 4, 5];

    let mut b = String::new();

    io::stdin()
        .read_line(&mut b)
        .expect("could not read from stdin");

    let b: usize = b.trim().parse().expect("could not parse");

    println!("value at index {b} = {} ", a[b]);
}
