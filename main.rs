

fn main() {
   let message = String::from("Hello");
   print_message(message);
}

fn print_message(a: String) {
    println!("{}", a);
    let c = a;
}

