

fn main() {
    let mut message = String::from("Hello");
    let slice = &message[2..4]; // 2 -> 3

    // message.clear();

    println!("{}", slice);
}

fn move_me(val: String) {}
