use rand::Rng;

#[derive(Debug, Clone, Copy)]
enum Kind {
    Person,
    Student,
}

struct Person {
    name: String,
    mail: String,
    age: i32,
    struct_kind: Kind,
}

struct Student {
    name: String,
    mail: String,
    age: i32,
    struct_kind: Kind,
}

fn person(name: &str, mail: &str, age: i32) -> Box<Person> {
    Box::new(Person {
        name: String::from(name),
        mail: String::from(mail),
        age,
        struct_kind: Kind::random(),
    })
}

fn student(name: &str, mail: &str, age: i32) -> Box<Student> {
    Box::new(Student {
        name: String::from(name),
        mail: String::from(mail),
        age,
        struct_kind: Kind::random(),
    })
}

trait Print {
    fn kind(&self) -> &Kind;
    fn to_string(&self) -> String;
}

impl Print for Person {
    fn kind(&self) -> &Kind {
        &self.struct_kind
    }
    fn to_string(&self) -> String {
        String::from(&self.name) + "<" + &self.mail + ">(" + &self.age.to_string() + ")"
    }
}

impl Print for Student {
    fn kind(&self) -> &Kind {
        &self.struct_kind
    }
    fn to_string(&self) -> String {
        String::from(&self.name) + "<" + &self.mail + ">(" + &self.age.to_string() + ")"
    }
}

impl Kind {
    fn random() -> Kind {
        let list = [Kind::Person, Kind::Student];
        let index = rand::thread_rng().gen_range(0..list.len());
        list[index]
    }
}

fn print_all<T: Print + ?Sized>(data: Vec<Box<T>>) {
    for item in data {
        match item.kind() {
            Kind::Person => println!("Person: {}", item.to_string()),
            Kind::Student => println!("Student: {}", item.to_string()),
        }
    }
}

fn main() {
    let taro = person("Taro", "taro@example.com", 30);
    let hanako = student("Hanako", "hanako@example.com", 20);
    let sachiko = person("Sachiko", "sachiko@example.com", 25);
    let data: Vec<Box<dyn Print>> = vec![taro, hanako, sachiko];
    print_all(data);
}
