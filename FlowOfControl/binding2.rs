fn some_number() -> Option<u32> {
    Some(43)
}

fn main() {
    match some_number() {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }
}
