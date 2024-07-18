use std::{cmp::Ordering, fmt::Display, time::Duration};

struct Foo {
    x: (u32, u32),
    y: u32,
}

enum RResult {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> RResult {
    if n % 2 == 0 {
        RResult::Ok(n / 2)
    } else {
        RResult::Err(format!("cannot divide {n} into two equal parts"))
    }
}

#[test]
fn test_matching() {
    let input = 'x';
    match input {
        'q' => println!("quitting"),
        'a' | 's' => println!("moving"),
        '0'..='9' => println!("number"),
        key if key.is_lowercase() => println!("lowercase {key}"),
        _ => println!("else"),
    }
    let foo = Foo { x: (1, 2), y: 3 };
    let x = match foo {
        Foo { x: (1, b), y } => None,
        Foo { y: 2, x: i } => Some(i.0),
        Foo { y, .. } => Some(y),
    };
    let n = 100;
    let result = divide_in_two(n);
    match &result {
        RResult::Ok(half) => println!("{n} div 2 {half}"),
        RResult::Err(msg) => println!("sorry error {msg}"),
    }
}

fn sleep_for(secs: f32) {
    if let Ok(dur) = Duration::try_from_secs_f32(secs) {
        std::thread::sleep(dur);
        println!("slept for {dur:?}");
    } // in the other implicit arm 'Err' don't do anything
}

#[test]
fn test_sleep() {
    sleep_for(-10.0);
    //sleep_for(0.8)
}

/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// A literal value
    Value(i64),
}

fn eval(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Value(val) => Result::Ok(val),
        Expression::Op { op, left, right } => {
            let l = eval(*left);
            let r = eval(*right);
            match (l, r) {
                (Result::Ok(left_val), Result::Ok(right_val)) => match op {
                    Operation::Add => Ok(left_val + right_val),
                    Operation::Sub => Ok(left_val - right_val),
                    Operation::Mul => Ok(left_val * right_val),
                    Operation::Div => {
                        if right_val == 0 {
                            Err(format!("division by zero"))
                        } else {
                            Ok(left_val / right_val)
                        }
                    }
                },
                (e @ Err(_), _) => return e,
                (_, e @ Err(_)) => return e,
                _ => panic!(), // oh very cool, it already knows this is unreachable
            }
        }
    }
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), Ok(19));
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        Ok(30)
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        Ok(85)
    );
}

#[test]
fn test_error() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(99)),
            right: Box::new(Expression::Value(0)),
        }),
        Err(String::from("division by zero"))
    );
}

struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            laps: Vec::new(),
        }
    }

    fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    fn print_laps(&self) {
        println!("Recorded {} laps for {}", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    fn finish(self) {
        let total: i32 = self.laps.iter().sum();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

#[test]
fn test_race() {
    let mut race = Race::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();
}

trait Animal {
    fn leg_count(&self) -> u32;
}

trait Pet: Animal {
    // 'supertrait' requires always implementing another trait too
    fn talk(&self) -> String;
    fn greet(&self) {
        println!("What's your name? {}", self.talk());
    }
}

struct Dog {
    name: String,
    age: i8,
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("My name is {}!", self.name)
    }
}

impl Animal for Dog {
    fn leg_count(&self) -> u32 {
        4
    }
}

#[test]
fn test_traits() {
    let fido = Dog {
        name: String::from("Fido"),
        age: 5,
    };
    fido.greet();
}

#[derive(Debug)]
struct Meters(i32);
#[derive(Debug)]
struct MetersSquared(i32);

trait Multiply {
    type Output; // associated type
    fn multiply(&self, other: &Self) -> Self::Output;
}

impl Multiply for Meters {
    type Output = MetersSquared; // associated type pinned by the trait implementor
    fn multiply(&self, other: &Self) -> Self::Output {
        MetersSquared(self.0 * other.0)
    }
}

#[test]
fn test_assc_type() {
    println!("{:?}", Meters(10).multiply(&Meters(20)))
}

#[derive(Debug, Clone, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

#[test]
fn test_deriving() {
    let p1 = Player::default(); // from default derive
    println!("{p1:?}");
    let mut p2 = p1.clone(); // from clone derive
    p2.name = String::from("special name");
    println!("{:?} vs {:?}", p1, p2);
}

pub trait Logger {
    fn log(&self, verbosity: u8, message: impl Display); // impl Display means some struct that
                                                         // implements the Display trait
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: impl Display) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

fn do_things(logger: &impl Logger) {
    logger.log(5, "FYI");
    logger.log(2, "uhoh");
}

struct VerbosityFilter {
    max_verbosity: u8,
    inner: StderrLogger,
    //inner: Box<dyn Logger>,
}

impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: impl Display) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}

#[test]
fn test_logger() {
    let l = VerbosityFilter {
        max_verbosity: 3,
        inner: StderrLogger,
    };
    do_things(&l);
}

fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

fn dup2<T>(a: T) -> (T, T)
where
    T: Clone, // same as above, but here T can be a more complex (nested) type
{
    (a.clone(), a.clone())
}

// shorthand for <T: Into<i32>>(x: T)
fn add_const(x: impl Into<i32>) -> i32 {
    x.into() + 40_000_000
}

fn pair_of(x: u32) -> impl std::fmt::Debug {
    (x + 1, x - 1)
} // the return type is 'some' concrete type that implements Debug. the concrete type isn't exposed
  // in the public API of the function.

// generic, static dispatch, monomorphized specialized function instance per Pet type
fn generic(pet: &impl Pet) {
    println!("Hello who are you? {}", pet.talk());
}

// dynamic dispatch with vtable, only one instance of this function is ever emitted
fn dynamic(pet: &dyn Pet) {
    println!("Hello who are you? {}", pet.talk());
}

/// This is a doc for the min function
pub fn min<T: Ord>(a: T, b: T) -> T {
    //a.min(b)
    //! Documentation for inner pieces of min
    match a.cmp(&b) {
        Ordering::Less | Ordering::Equal => a,
        Ordering::Greater => b,
    }
}

#[test]
fn test_min() {
    assert_eq!(min("hello", "goodbye"), "goodbye");
}

#[test]
fn test_option() {
    let name = "My name";
    let mut position = name.find('n');
    println!("{position:?}");
    assert_eq!(position.unwrap(), 3);
}

#[test]
fn test_vec() {
    let mut v3 = vec![0, 0, 1, 2, 3, 4];
    v3.retain(|x| x % 2 == 0);
    println!("{v3:?}");
    v3.dedup();
    println!("{v3:?}");
    v3.sort_unstable();
}

use std::collections::HashMap;
use std::hash::Hash;

/// Counter counts the number of times each value of type T has been seen.
struct Counter<T: Hash + Eq> {
    values: HashMap<T, u64>,
}

impl<T: Hash + Eq> Counter<T> {
    /// Create a new Counter.
    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    /// Count an occurrence of the given value.
    fn count(&mut self, value: T) {
        let entry = self.values.entry(value).or_insert(0);
        *entry += 1;
        // if self.values.contains_key(&value) {
        //     *self.values.get_mut(&value).unwrap() += 1;
        // } else {
        //     self.values.insert(value, 1);
        // }
    }

    /// Return the number of times the given value has been seen.
    fn times_seen(&self, value: T) -> u64 {
        self.values.get(&value).copied().unwrap_or_default()
    }
}

#[test]
fn test_counter() {
    let mut ctr = Counter::new();
    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);

    for i in 10..20 {
        println!("saw {} values equal to {}", ctr.times_seen(i), i);
    }

    let mut strctr = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");
    println!("got {} apples", strctr.times_seen("apple"));
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

// operator overloading
impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// support borrowing the operands, not consuming them
impl std::ops::Add for &Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// we can also support adding any types as we specify
impl std::ops::Add<(i32, i32)> for Point {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

// Read abstracts over any bytestream
use std::io::{BufRead, BufReader, Read};

fn count_lines<R: Read>(reader: R) -> usize {
    let buf_reader = BufReader::new(reader);
    buf_reader.lines().count()
}

#[test]
fn test_read() {
    let slice: &[u8] = b"foo\nbar\nbaz\n"; // type annotation is required, so this becomes a byte
                                           // slice rather than a byte array
    println!("lines in slice {}", count_lines(slice));
}

// Write abstracts over any sink for a bytestream
use std::io::Write;

fn log<W: Write>(writer: &mut W, msg: &str) {
    writer.write_all(msg.as_bytes());
    writer.write_all("\n".as_bytes()); // returns Result<()>
}

#[test]
fn test_write() {
    let mut buffer = Vec::<u8>::new();
    log(&mut buffer, "Hello");
    log(&mut buffer, "World");
    println!("logged: {:?}", buffer);
}

// Default trait defines ::default()
#[derive(Debug, Default)]
struct Derived {
    x: u32,
    y: String,
    z: Implemented,
}

#[derive(Debug)]
struct Implemented(String);

impl Default for Implemented {
    fn default() -> Self {
        Self("John Smith".into())
    }
}

#[test]
fn test_default() {
    let default_struct = Derived::default();
    println!("{default_struct:#?}");

    let almost_default = Derived {
        y: "y is set!".into(),
        ..Derived::default()
    };
    println!("{almost_default:#?}");

    let nothing: Option<Derived> = None;
    println!("{:#?}", nothing.unwrap_or_default());
}

// Closures

fn apply_with_log(func: impl FnOnce(i32) -> i32, input: i32) -> i32 {
    println!("Calling function on {input}");
    func(input)
}

#[test]
fn test_closures() {
    let add_3 = |x| x + 3;
    println!("add_3 {}", apply_with_log(add_3, 10));
    println!("add_3 {}", apply_with_log(add_3, 20));

    let mut v = Vec::new();
    let mut accumulate = |x: i32| {
        v.push(x);
        v.iter().sum::<i32>()
    };
    println!("accumulate: {}", apply_with_log(&mut accumulate, 4));
    println!("accumulate: {}", apply_with_log(&mut accumulate, 5));

    let multiply_sum = |x| x * v.into_iter().sum::<i32>();
    println!("multuply sum {}", apply_with_log(multiply_sum, 3));
}

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

// Implement the `Read` trait for `RotDecoder`.
impl<R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // read as much of input into buf as possible
        let size = self.input.read(buf)?;
        // then mutate buf to perform rot13
        for b in &mut buf[..size] {
            if b.is_ascii_alphabetic() {
                let base = if b.is_ascii_uppercase() { 'A' } else { 'a' } as u8;
                *b = (*b - base + self.rot) % 26 + base;
            }
        }
        Ok(size)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot = RotDecoder {
            input: "Gb trg gb gur bgure fvqr!".as_bytes(),
            rot: 13,
        };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> {
            input: input.as_ref(),
            rot: 13,
        };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}
