#[derive(Debug)]
struct Person {
    name: String,
    mail: String,
    age: i32,
}

fn person(name: &str, mail: &str, age: i32) -> Person {
    Person {
        name: String::from(name),
        mail: String::from(mail),
        age,
    }
}

#[derive(Debug)]
struct Student {
    name: String,
    mail: String,
    grade: i32,
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
    println!("Person: {:#?}", person1);
    println!("Student: {:#?}", student1);
}
