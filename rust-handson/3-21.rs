struct Person {
    name: String,
    mail: String,
    age: i32,
}

fn print_person(p: Person) {
    println!("I'm {}({}). Mail to {}.", p.name, p.age, p.mail);
}

fn main() {
    let taro = Person {
        name: String::from("Taro"),
        mail: String::from("taro@example.com"),
        age: 39,
    };
    let hanako = Person {
        name: String::from("Hanako"),
        mail: String::from("hanako@example.com"),
        age: 28,
    };
    print_person(taro);
    print_person(hanako);
}
