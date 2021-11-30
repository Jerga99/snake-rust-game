

fn main() {
   let message = String::from("Hello");
   let message = extend_message(message);

   let age = 30;
   extend_age(age);
   println!("{}", age);

   println!("{}", message);
}

fn extend_message(mut a: String) -> String {
    a.push_str(" World");
    a
}

fn extend_age(mut a: u32)  {
    a += 100;
}

