#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    } // when calling the method, Rust auto (de)references
      // the object to match the signature
    fn can_hold(&self, rect2: &Rectangle) -> bool {
        (self.width >= rect2.width && self.height >= rect2.height)
            || (self.width >= rect2.height && self.height >= rect2.width)
    }

}
// we can have multiple impl
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 1;
    let rect1 = Rectangle {
        width: dbg!(50 * scale),
        height: 30,
    };

    dbg!(&rect1);

    println!(
        "The area of the rectangle {:?} is {} square pixels",
        rect1,
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 40,
        height: 10,
    };

    let rect3 = Rectangle {
        width: 45,
        height: 60,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
