trait Print {
    fn print(&self);
}

struct Person {
    name: String,
    mail: String,
    age: i32,
}

impl Print for Person {
    fn print(&self) {
        println!(
            "Name: {}, Mail: {}, Age: {}",
            self.name, self.mail, self.age
        );
    }
}

fn person(name: &str, mail: &str, age: i32) -> impl Print {
    Person {
        name: String::from(name),
        mail: String::from(mail),
        age,
    }
}

struct Student {
    name: String,
    mail: String,
    grade: i32,
}

impl Print for Student {
    fn print(&self) {
        println!(
            "Name: {}, Mail: {}, Grade: {}",
            self.name, self.mail, self.grade
        );
    }
}

fn student(name: &str, mail: &str, grade: i32) -> impl Print {
    Student {
        name: String::from(name),
        mail: String::from(mail),
        grade,
    }
}

fn main() {
    let person1 = person("Alice", "alice@example.com", 30);
    let student1 = student("Bob", "bob@example.com", 90);
    print(&person1);
    print(&student1);
}

fn print(ob: &impl Print) {
    ob.print();
}
