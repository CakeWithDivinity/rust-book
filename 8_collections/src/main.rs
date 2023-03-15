use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = vec![1, 2, 3];

    v.push(4);

    let third: i32 = v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(value) => println!("The third element is {value}"),
        None => println!("There is no third element"),
    }

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    for i in &mut v {
        *i += 50;
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(42.69),
    ];

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50); 

    scores.entry(String::from("Blue")).or_insert(50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
