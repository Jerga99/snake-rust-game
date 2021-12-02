


struct Person {
    name: String, // fields
    last_name: String,
    age: u32,
}

impl Person {
    // associated function
    fn some_function() {
        println!("some_function");
    }

    // method
    // first parameter is always self, which represents the instance of the struct the
    // method is being called on
    // Within an impl block, the type Self is an alias for the current type
    fn display_age(&self) {
       println!("Current Age: {}", self.age);
    }
}

fn main() {
    Person::some_function();

    let person =  Person {
       name: "Filip".to_string(),
       last_name: "Jerga".to_string(),
       age: 30,
    };

    let person_2 =  Person {
        name: "John".to_string(),
        last_name: "Snow".to_string(),
        age: 35,
     };

    person.display_age();
    person_2.display_age();

    println!("{} {} {}", person.name, person.last_name, person.age);
}
