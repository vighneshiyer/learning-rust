pub fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(10, 20), 200);
    let x: i8 = 13;
    let y: i16 = 1000;
    // let _z: i32 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y));
}

pub fn fib(n: u32) -> u32 {
    if n <= 2 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

#[test]
fn test_fib() {
    assert_eq!(fib(5), 5);
    assert_eq!(fib(6), 8);
}

pub fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(4), 24);
}

pub fn fizzbuzz(max: u32) -> () {
    for i in 1..=max {
        if i % 15 == 0 {
            println!("fizzbuzz")
        } else if i % 3 == 0 {
            println!("fizz")
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }
}

#[test]
fn test_fizzbuzz() {
    fizzbuzz(31);
}

pub fn collatz_length(mut n: i32) -> u32 {
    let mut length = 1;
    while n > 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
        length += 1;
    }
    length
}

#[test]
fn test_collatz_length() {
    println!("{}, {}", collatz_length(1), collatz_length(3));
    assert_eq!(collatz_length(1), 1);
    assert_eq!(collatz_length(3), 8);
}

#[test]
fn test_arrays() {
    let mut a: [i8; 10] = [42; 10]; // fill array with literal '42'
    a[5] = 0;
    println!("{a:?}");

    let t: (i8, bool) = (7, true); // play with tuples, quite ez, nice syntax
    println!("t.0: {}, t.1: {}", t.0, t.1);

    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }
}

pub fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut mat: [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            mat[i][j] = matrix[j][i]
        }
    }
    return mat;
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );

    let array: [[i32; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let _array = transpose(array);
}

#[test]
fn test_references() {
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("r: {}", *r);
    r = &b;
    println!("r: {}, {}", *r, r.is_ascii());

    let mut point = (1, 2);
    let x_coord = &mut point.0;
    *x_coord = 20;
    println!("point: {point:?}");

    // &mut i32 is not the same as mut &i32!!!
}

#[test]
fn test_slices() {
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    // Can modify a here
    a[3] = 400;
    let s: &[i32] = &a[2..4];
    // Can't modify a here
    println!("s: {s:?}");
    // Can modify a again
    a[1] = 900;
    println!("a: {a:?}");
}

#[test]
fn test_strings() {
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[s2.len() - s1.len()..];
    println!("s3: {s3}");

    println!("{:?}", b"abc");
}

// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.

pub fn magnitude(vector: &[f64; 3]) -> f64 {
    let mut sum: f64 = 0.;
    for dim in vector {
        sum += dim * dim;
    }
    sum.sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.

pub fn normalize(vector: &mut [f64; 3]) {
    let mag = magnitude(vector);
    for dim in vector {
        *dim = *dim / mag;
    }
}

// Use the following `main` to test your work.

#[test]
fn test_geometry() {
    println!(
        "Magnitude of a unit vector: {}",
        magnitude(&[0.0, 1.0, 0.0])
    );

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
    assert_eq!(magnitude(&v), 1.);
}

pub struct Person {
    name: String,
    age: u8,
}

pub fn describe(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

#[test]
fn test_structs() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    describe(&peter);
    peter.age = 28;
    describe(&peter);
    let jackie = Person {
        name: String::from("Jackie"),
        ..peter // take the rest of the fields from another struct instance
    };
    describe(&jackie);
}

pub struct Point(i32, i32);

// This is just a type alias effectively, the wrapper has no runtime cost
struct PoundsOfForce(f64);

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,
    Run(Direction),
    Teleport { x: u32, y: u32 },
}

#[repr(u32)]
enum Bar {
    A,
    B = 10000,
    C,
}

#[test]
fn test_repr() {
    println!("A: {}", Bar::A as u32);
    println!("B: {}", Bar::B as u32);
    println!("C: {}", Bar::C as u32);
}

#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
enum Event {
    EnterRequest { dir: ElevatorDirection, floor: i32 },
    ExitRequest { floor: i32 },
    CarArrival { floor: i32 },
    CarDoorsOpen,
    CarDoorsClose,
}

/// A direction of travel.
#[derive(Debug)]
enum ElevatorDirection {
    Up,
    Down,
}

/// The car has arrived on the given floor.
fn car_arrived(floor: i32) -> Event {
    Event::CarArrival { floor }
}

/// The car doors have opened.
fn car_door_opened() -> Event {
    Event::CarDoorsOpen
}

/// The car doors have closed.
fn car_door_closed() -> Event {
    Event::CarDoorsClose
}

/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: ElevatorDirection) -> Event {
    Event::EnterRequest { dir, floor }
}

/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::ExitRequest { floor }
}

#[test]
fn test_elevator() {
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, ElevatorDirection::Up)
    );
    println!(
        "The car has arrived on the ground floor: {:?}",
        car_arrived(0)
    );
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}
