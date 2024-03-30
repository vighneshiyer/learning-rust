use comprehensive_rust::{collatz_length, fizzbuzz};

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn fib(n: u32) -> u32 {
    if n <= 2 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

fn main() {
    factorial(4);
    println!("{:?}", (100, 200));
    println!("{}, {}", collatz_length(1), collatz_length(3));
    // let x: i8 = 13;
    // let y: i16 = 1000;
    // let _z: i32 = 1000;

    // println!("{x} * {y} = {}", multiply(x.into(), y));

    // let n = 20;
    // println!("fib({n}) = {}", fib(n));

    // for x in 1..5 {
    //     println!("x: {x}");
    // }

    // for elem in [1, 2, 3, 4, 5] {
    //     println!("elem: {elem}");
    // }

    // let mut i = 0;
    // loop {
    //     i += 1;
    //     println!("{i}");
    //     if i > 10 {
    //         break;
    //     }
    // }
}
