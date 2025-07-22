// 警告が出る理由：
// - Debugトレイトで出力しているだけでは、フィールドが「使用された」とみなされない
// - Rustコンパイラは、構造体のフィールドが読み取られていないことを検出し、dead_code警告を出す
// - これは未使用のコードを検出してコードの品質を保つための機能
#[allow(dead_code)]
#[derive(Debug)]
enum Kind {
    Person(Person),
    Student(Student),
}

#[derive(Debug)]
#[allow(dead_code)]
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
    age: i32,
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
        println!("{item:?}");
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
