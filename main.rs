

fn main() {
    let mut message = String::from("Hello");
    let message_3 = message.clone();

    message.clear();

    println!("{}", message);
    println!("{}", message_3);

}
