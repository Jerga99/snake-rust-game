


struct Person {
    name: String,
    last_name: String,
    age: u32,
}

fn main() {
   let person =  Person {
       name: "Filip".to_string(),
       last_name: "Jerga".to_string(),
       age: 30,
   };

   println!("{} {} {}", person.name, person.last_name, person.age);
}
