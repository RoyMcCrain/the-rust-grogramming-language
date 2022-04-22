fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    another_func(5, 11);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    println!("The value of five is: {}", add(10, 100));

    let number = 4;

    if number < 5 {
        println!("True")
    } else {
        println!("False")
    }

    let cond = true;
    let number = if cond { 6 } else { 7 };
    println!("let if: {}", number);

    for element in 1..10 {
        println!("the value is: {}", element);
    }
}

fn another_func(x: i32, y: i32) {
    println!("x: {}", x);
    println!("y: {}", y);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
