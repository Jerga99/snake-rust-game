


struct Person {
    name: String, // fields
    last_name: String,
    age: u32,
}

impl Person {
    fn new() -> Person {
        Person {
            name: "Default".to_string(),
            last_name: "Default".to_string(),
            age: 0
        }
    }

    fn from(name: String, last_name: String, age: u32) -> Person {
        Person {
            name,
            last_name,
            age
        }
    }

    fn change_age(&mut self, new_age: u32) {
       self.age = new_age;
    }
}

fn main() {
    let mut person = Person::new();
    let person_2 = Person::from(
        String::from("John"),
        String::from("Snow"),
        35
    );

    person.change_age(50);

    println!("{} {} {}", person.name, person.last_name, person.age);
    println!("{} {} {}", person_2.name, person_2.last_name, person_2.age);
}
