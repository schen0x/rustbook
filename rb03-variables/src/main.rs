fn main() {
    const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let x = 5;
    println!("The value of x is: {x}");
    let x = 6;
    println!("The value of x is: {x}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0; // tuple.0, first element
    let _six_point_four = x.1;
    let _one = x.2;

    let _a: [i32; 5] = [1, 2, 3, 4, 5]; // array
    let _a = [3; 5]; // [3, 3, 3, 3, 3]
    assert_eq!(_a[4], 3);

    let y = { // A statement: instructions that perform some action and do not return a value
        let x = 3;
        x + 1 // An expression: evaluate to a resultant value. no semicolon, return a value
    };

    println!("The value of y is: {y}");
    let x = _plus_two(2);
    println!("The value of 2+2 is: {x}");

    loop0();
}

fn _plus_one(x: i32) -> i32 {
    x + 1
}
fn _plus_two(x: i32) -> i32 {
    return x + 2;
}

fn loop0() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..=3).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
