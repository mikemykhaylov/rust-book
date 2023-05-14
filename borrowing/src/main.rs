fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("{len}");

    let mut s2 = s1;

    modify(&mut s2);

    println!("{s2}");

    // multiple references
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");

    let r3 = &mut s;
    println!("{r3}");

    // slicing demo
    // string literals ARE string slices

    let literal = "sliced string";

    let word = first_word(literal);

    println!("{word}");

    let s1 = String::from(literal);

    // a reference to a String is just a full slice
    let word = first_word(&s1);

    println!("{word}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn modify(s: &mut String) {
    s.push_str(", world");
}

// this is invalid, as &s is a "dangling reference"
// it references s, but s goes out of scope
// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }

// slices
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &chr) in bytes.iter().enumerate() {
        if chr == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
