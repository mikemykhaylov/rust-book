use std::collections::HashMap;

fn main() {
    // Vector
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);

    let fourth = &v[3];
    println!("The fourth element is {fourth}");

    let fifth = v.get(4);
    match fifth {
        Some(int) => println!("The fifth element is {int}"),
        _ => println!("There is no fifth element"),
    }

    v.push(5);
    v.pop();

    // println!("{}", fourth); // doesn't compile because fourth is an immutable reference
    // that in turn doesn't work because while pushing, vector could've moved to another
    // location in memory, and thus fourth will point to deallocated memory

    let mut v = vec![100, 32, 57];
    for i in &v {
        print!("{i} ");
    }
    println!();

    for i in &mut v {
        *i *= 50; // * is dereference operator
    }

    for i in &v {
        // v.push(*i); // doesn't compile, because we try to modify v while
        // an immutable borrow is held by the loop
        print!("{i} ");
    }
    println!();

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    };

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // String

    let s1 = String::from("s1");
    let s2 = String::from("s2");
    let s3 = s1 + &s2; // + took ownership of s1, appended s2 and assigned to s3
                       // therefore, s1 is not valid after this. &s2 is coerced from &String to &str, turning to &s2[..]

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); // tic-tac-toe

    // getting individual chars is complicated, because not every byte is a unique UTF-8 char
    // therefore, we cannot do &s[0]. If we try to slice, be can potentially break things
    // as a byte might not be a valid character boundary. thus, we can do one of two things
    println!("{}", "Зд".chars().next().unwrap());
    println!("{}", "Зд".bytes().next().unwrap());

    // HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, val) in &scores {
        println!("{}: {}", key, val);
    }

    scores.entry(String::from("Green")).or_insert(50);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
