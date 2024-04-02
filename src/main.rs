use comprehensive_rust::{collatz_length, factorial, fizzbuzz, transpose};

fn main() {
    factorial(4);
    println!("{:?}", (100, 200));
    println!("{}, {}", collatz_length(1), collatz_length(3));

    let mut a: [i8; 10] = [42; 10]; // fill array with literal '42'
    a[5] = 0;
    println!("{a:#?}");

    let t: (i8, bool) = (7, true);
    println!("t.0: {}, t.1: {}", t.0, t.1);

    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }

    let array: [[i32; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let array = transpose(array);

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
