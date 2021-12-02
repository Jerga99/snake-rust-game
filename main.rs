

fn main() {
    let mut message = String::from("Hello");
    let slice = &message[2..4]; // 2 -> 3
    // H E L L O
    // 0 1 2 3 4

    println!("{}", slice);
}
