trait Print {
    fn print(&self) {
        println!("PRINT is note yet implemented...");
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    mail: String,
    age: i32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Student {
    name: String,
    mail: String,
    grade: i32,
}

impl Print for Person {
    fn print(&self) {
        println!(
            "Name: {}, Mail: {}, Age: {}",
            self.name, self.mail, self.age
        );
    }
}

impl Print for Student {}

fn person(name: &str, mail: &str, age: i32) -> Person {
    Person {
        name: String::from(name),
        mail: String::from(mail),
        age,
    }
}

fn student(name: &str, mail: &str, grade: i32) -> Student {
    Student {
        name: String::from(name),
        mail: String::from(mail),
        grade,
    }
}

fn main() {
    let person1 = person("Alice", "alice@example.com", 30);
    let student1 = student("Bob", "bob@example.com", 90);
    person1.print();
    student1.print();
}
