struct Person {
    name: String,
    mail: String,
    age: i32,
}

fn person(name: String, mail: String, age: i32) -> Person {
    Person { name, mail, age }
}

impl Person {
    fn print(&self) {
        println!("{}<{}>{}", self.name, self.mail, self.age);
    }

    fn fields() -> [String; 3] {
        [
            String::from("name: String"),
            String::from("mail: String"),
            String::from("age: i32"),
        ]
    }
}

fn main() {
    let taro = person(String::from("Taro"), String::from("taro@example.com"), 39);
    let hanako = Person {
        name: String::from("Hanako"),
        mail: String::from("hanako@example.com"),
        age: 28,
    };
    taro.print();
    hanako.print();
    println!("Person fields: {:?}", Person::fields());
}
