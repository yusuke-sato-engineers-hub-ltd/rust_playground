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

fn person(name: String, mail: String, age: i32) -> Person {
    Person { name, mail, age }
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

fn student(name: String, mail: String, grade: i32) -> Student {
    Student { name, mail, grade }
}

fn main() {
    let person1 = person("Alice".to_string(), "alice@example.com".to_string(), 30);
    let student1 = student("Bob".to_string(), "bob@example.com".to_string(), 90);
    print(&person1);
    print(&student1);
    print(&student1); // 借用のため、同じオブジェクトを再度使用できる
}

fn print(ob: &impl Print) {
    ob.print();
}
