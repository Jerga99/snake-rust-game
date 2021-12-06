

pub mod learnig_rust {

    mod top_level {
        pub fn hi_there() {
            println!("hi there!");
        }

        pub mod low_level {
            pub fn hello_world() {
                println!("hello world!");
            }
        }
    }


    pub trait Log {
        fn display_info(&self);
        fn alert_something(&self) {
            println!("Default implementation!!!!!!!")
        }
    }

    #[derive(Debug)]
    pub enum PersonId {
        Passport(u32),
        IndentityCard(u32, u32, u32),
    }

    pub struct Person {
        name: String, // fields
        last_name: String,
        age: u32,
        id: PersonId,
    }

    pub struct Animal(pub String);

    impl Log for Animal {
        fn display_info(&self) {
            println!("{}", self.0)
        }

        fn alert_something(&self) {
            println!("ANIMAL implementation!!!!!!!")
        }
    }

    impl Log for Person {
        fn display_info(&self) {

            // absolute path
            // crate points to -> src/lib.rs and src/main.rs
            crate::learnig_rust::top_level::hi_there();
            crate::learnig_rust::top_level::low_level::hello_world();

            // relative path
            top_level::hi_there();
            top_level::low_level::hello_world();

            println!("{} {} {} {:?}", self.name, self.last_name, self.age, self.id)
        }
    }

    impl Person {
        pub fn new() -> Person {
            Person {
                name: "Default".to_string(),
                last_name: "Default".to_string(),
                age: 0,
                id: PersonId::IndentityCard(540, 320, 100)
            }
        }

        pub fn from(name: String, last_name: String, age: u32, id: PersonId) -> Person {
            Person {
                name,
                last_name,
                age,
                id
            }
        }

        pub fn name(&self) -> &String {
            &self.name
        }

        pub fn change_age(&mut self, new_age: u32) {
            self.age = new_age;
        }
    }

    // impl makes the compiler determine type at the compile time
    // it will create multiple versions of the function, depending on
    // how many types Log trait implements (Person, Animal)
    pub fn log_info(val: impl Log) {
        val.alert_something();
    }


    // dyn is short for dynamic, and says that function should perform dynamic dispatch
    // decission of exactly which function to call at the runtime
    pub fn log_info_2(val: &dyn Log) {
        val.alert_something();
    }
}
