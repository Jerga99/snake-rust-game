
// use snake_game::Person;
// use snake_game::Animal;
// use snake_game::log_info;
// use snake_game::log_info_2;

// use snake_game::*;
use snake_game::{Person, Animal, log_info, log_info_2};

fn main() {
    let mut person = Person::new();
    let animal = Animal(String::from("dog"));

    person.change_age(38);

    log_info(person);
    log_info_2(&animal);
}
