

#[derive(Debug)]
enum PersonId {
  Passport,
  IndentityCard,
}

struct Person {
    name: String, // fields
    last_name: String,
    age: u32,
    id: PersonId,
}

impl Person {
    fn new() -> Person {
        Person {
            name: "Default".to_string(),
            last_name: "Default".to_string(),
            age: 0,
            id: PersonId::IndentityCard
        }
    }

    fn from(name: String, last_name: String, age: u32, id: PersonId) -> Person {
        Person {
            name,
            last_name,
            age,
            id
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
        35,
        PersonId::Passport
    );

   println!("{:?}", person.id);
   println!("{:?}", person_2.id);
}
