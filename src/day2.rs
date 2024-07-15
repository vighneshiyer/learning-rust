struct Foo {
    x: (u32, u32),
    y: u32,
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
}
