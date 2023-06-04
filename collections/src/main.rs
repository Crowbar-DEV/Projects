use std::collections::HashMap;

fn main() {
    let mut hm: HashMap<&str, i32> = HashMap::new();
    hm.insert("1", 1);
    hm.entry("1").or_insert(3);
    // println!("{:?}", hm);

    // println!("value: {}", hm.entry("1").or_insert(3));

    let text = "key1 key2 key3";
    for i in text.split_whitespace() {
        let val = hm.entry(i).or_insert(1);
        println!("{:?}", val)
    }

    let v: Vec<i32> = Vec::new();
    let mut t = vec![1, 2, 3];
    //t.push(6);

    for i in &mut t {
        *i = *i * 2;
        println!("{i}");
    }

    let second: Option<&i32> = t.get(2);

    let first: &i32 = &t[0];
    println!(
        "{first},{}",
        match second {
            Some(second) => second,
            _ => &0,
        }
    );
}
