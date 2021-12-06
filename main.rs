

use snake_game::learnig_rust::{Person, PersonId};
// use snake_game::learnig_rust::top_level;

fn main() {
    let person = Person::new();
    // person.display_info();

    let id = PersonId::Passport(432);
    println!("{:?}", id);
    println!("{}", person.name());
}
