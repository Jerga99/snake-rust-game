


fn main() {
  let message = "Hello World";
  let message_2 = print_welcome(message);
  println!("{}", message_2);
}

fn print_welcome(text: &str) -> &str {
  println!("{}", text);
  let new_message = "Hi There";
  // return new_message;
  new_message
}
