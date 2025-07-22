#[derive(Debug)]
enum Kind {
    Person(Person),
    Student(Student),
}

#[derive(Debug)]
struct Person {
    name: String,
    mail: String,
    age: i32,
}

#[derive(Debug)]
struct Student {
    name: String,
    mail: String,
    age: i32,
}

impl Person {
    fn display(&self) {
        println!(
            "Person: {} ({} years old) - {}",
            self.name, self.age, self.mail
        );
    }
}

impl Student {
    fn display(&self) {
        println!(
            "Student: {} ({} years old) - {}",
            self.name, self.age, self.mail
        );
    }
}

fn person(name: &str, mail: &str, age: i32) -> Person {
    Person {
        name: String::from(name),
        mail: String::from(mail),
        age,
    }
}

fn student(name: &str, mail: &str, age: i32) -> Student {
    Student {
        name: String::from(name),
        mail: String::from(mail),
        age,
    }
}

fn print_all(data: Vec<Kind>) {
    for item in data {
        match item {
            Kind::Person(p) => p.display(),
            Kind::Student(s) => s.display(),
        }
    }
}

fn main() {
    let taro = Kind::Person(person("Taro", "taro@example.com", 30));
    let hanako = Kind::Student(student("Hanako", "hanako@example.com", 20));
    let jiro = Kind::Student(student("Jiro", "jiro@example.com", 22));
    let sachiko = Kind::Person(person("Sachiko", "sachiko@example.com", 25));
    let data: Vec<Kind> = vec![taro, hanako, jiro, sachiko];
    print_all(data);
}
