#[derive(Debug)]
struct Address {
    street: u32,
    city: String,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    address: Address
}

fn person_summary(person: &Person) -> String {
    return format!("{} is {} years old and lives at {}, {}", person.name, person.age, person.address.street, person.address.city);
}

fn is_adult(person: &Person) -> bool {
    person.age >= 18
}

fn main() {

    let mike = Person {
        name: String::from("Mike"),
        age: 17,
        address: Address { street: 123, city: String::from("Bogotá") }
    };
    let james = Person {
        name: String::from("James"),
        age: 25,
        address: Address { street: 999, city: String::from("Barranquilla") }
    };

    println!("{}", person_summary(&mike));
    println!("{} is adult: {}", mike.name, is_adult(&mike));
    println!();
    println!("{}", person_summary(&james));
    println!("{} is adult: {}", james.name, is_adult(&james));
}