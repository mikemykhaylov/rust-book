// regular struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple struct aka named tuples
struct Color(i32, i32, i32);

// unit-like struct
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1
    }; // because username was not replaced and is a String
       // it has been moved to user2, so user1 became invalid

    let black = Color(0, 0, 0);
    let Color(r, g, b) = black;
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
