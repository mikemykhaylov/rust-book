fn main() {
    // s is not valid here, since it's not declared in this scope
    {
        // still not valid, not yet declared
        let s = "hello"; // valid from this point on

        // do stuff with s
        println!("{s} from the nested scope");
    } // scope is now over, s is no longer valid

    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    let mut s1 = s; // s is no longer valid
    let s2 = s1.clone();
    s1.push_str(" from s1");
    println!("s1 = {s1}, s2 = {s2}");

    // ownership in functions
    let s1 = String::from("hello");

    takes_ownership(s1); // String got moved to function, invalid from here

    let x = 5;

    makes_copy(x); // i32 implements Copy trait, so is only copied

    let s2 = gives_ownership();

    let s3 = takes_n_gives_back(s2); // s2 was moved, s3 was returned
}

fn takes_ownership(s: String) {
    println!("{s}");
}

fn makes_copy(x: i32) {
    println!("{x}");
}

fn gives_ownership() -> String {
    String::from("I got moved to main!")
}

fn takes_n_gives_back(s: String) -> String {
    s
}
