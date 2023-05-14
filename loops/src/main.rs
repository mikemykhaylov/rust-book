fn main() {
    // returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // loop labels and nested loops

    let mut count = 0;

    'counting_up: loop {
        println!("count={count}");

        let mut remaining = 10;
        loop {
            println!("remaining={remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // for loop
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");

    // for-in loop
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("{}", element);
    }

    // Fibonacci exercise
    let mut prev_prev = 1;
    let mut prev = 1;
    for i in 0..20 {
        let new = prev_prev + prev;
        prev_prev = prev;
        prev = new;

        println!("The {} fibonacci number is {}", i + 3, new);
    }
}
