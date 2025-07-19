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

fn person(name: &str, mail: &str, age: i32) -> Box<dyn Print> {
    Box::new(Person {
        name: String::from(name),
        mail: String::from(mail),
        age,
    })
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

fn student(name: &str, mail: &str, grade: i32) -> Box<dyn Print> {
    Box::new(Student {
        name: String::from(name),
        mail: String::from(mail),
        grade,
    })
}

fn main() {
    let person1 = person("Alice", "alice@example.com", 30);
    let student1 = student("Bob", "bob@example.com", 90);
    // Box<dyn Print>に対して、&*で参照外しをしてから&dyn Printとして渡す
    print(&*person1);
    print(&*student1);
}

// &Box<dyn Print>を&dyn Printに変更すべき理由：
// 1. 不要な間接参照
//    - Box<dyn Print>は既にヒープ上のトレイトオブジェクトへのポインタ
//    - &Box<dyn Print>は「ポインタへの参照」で二重の間接参照
//    - &dyn Printで十分（直接トレイトオブジェクトへの参照）
//
// 2. 自動型強制
//    - Rustは&Box<T>を自動的に&Tに変換できる（Deref trait）
//    - つまり、&Box<dyn Print>を受け取る関数は&dyn Printでも動作する
//
// 3. 柔軟性
//    - &dyn Printなら、Box以外の方法で作られたトレイトオブジェクトも受け取れる
//    - 例：&person1（スタック上の値への参照）も渡せる
//
// 図解:
// &Box<dyn Print> → &[Box] → [dyn Print]  // 2段階
// &dyn Print      → [dyn Print]            // 1段階
fn print(ob: &dyn Print) {
    ob.print();
}
