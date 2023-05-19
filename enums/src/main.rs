enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn send(&self) {
        // do smth
    }
}

fn main() {
    let four = IpAddr::V4(127, 0, 0, 1);
    let six = IpAddr::V6(String::from("::1"));

    let msg = Message::Quit;
    msg.send();

    let some_i32 = Some(5);
    let none_i32: Option<i32> = None;

    if let Some(num) = some_i32 {
        println!("Some i32 was not None, but was {num}");
    }

    if let Some(num) = none_i32 {
        println!("Some i32 was not None, but was {num}");
    } else {
        println!("Some i32 was None");
    }

    if let Message::Write(string) = msg {
        println!("{string}");
    } else {
        println!("Message wasn't Write");
    }
}
