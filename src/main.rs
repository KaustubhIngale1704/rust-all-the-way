use std::io;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let predefined_array = [1, 7, 19, 23, 56, 89, 104, 155];

    let mut num = String::new();

    loop {
        println!("Please enter an input number for search");

        io::stdin()
            .read_line(&mut num)
            .expect("Expected a valid input");

        let num: i32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{num} is not a valid number. Please enter valid input");
                num = String::new();
                continue;
            }
        };

        let mut low: usize = 0;
        let mut high: usize = predefined_array.len();

        while low < high {
            let mid = (low + high) / 2;

            if predefined_array[mid] == num {
                println!("Element {num} is found at index {mid}");
                return;
            } else if num > predefined_array[mid] {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        println!("Eleemnt {num} is not found in the array");

        break;
    }

    println!("Enter which fibonacci number you want to print");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Valid input is needed");

    let num: i32 = match num.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("{num} is not a valid number. Error {err}");
            return;
        }
    };

    let mut i: i32 = 0;
    let mut k = 0;
    let mut prev = 0;
    while i <= num {
        if i == 0 {
            i = i + 1;
            print!("{prev}");
            continue;
        }

        if i == 1 {
            prev += 1;
            i += 1;
            print!("{prev}");
            continue;
        }

        let j = prev;
        prev = prev + k;
        k = j;

        i = i + 1;

        print!("{prev}");
    }

    println!("\n");

    println!("The fibonacci number at given index is: {prev}");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 30,
    };

    let rect3 = Rectangle {
        width: 50,
        height: 70,
    };

    let ar = Rectangle::square(4);

    println!("Area of a rectangle is {}", ar.area());

    println!("Rect1 can hold: {}", rect1.can_hold(&rect2));

    println!("Rect2 can hold: {}", rect2.can_hold(&rect3));
    println!("The area of rectangle is {}", rect1.area());

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));

    let r = Message::Move { x: 3, y: 4 };

    r.call();

    m.call();

    let ps = value_in_cents(Coin::Quarter(UsState::Alaska));

    println!("The value of the input coin in cents is {ps}");

    let five = Some(5);

    let six = plus_one(five);
    let none = plus_one(None);

    println!("The result of the sum 1 is {six:?}");

    println!("The result of the sum1 is {none:?}");

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remoe_fancy_hat(),
        _ => reroll(),
    }
}

fn add_fancy_hat() {}

fn remoe_fancy_hat() {}

fn reroll() {}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl Message {
    fn call(&self) {}
}
