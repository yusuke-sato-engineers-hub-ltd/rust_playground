// Printトレイトの定義
// デフォルト実装を持つトレイトで、実装しない場合のフォールバック動作を提供
trait Print {
    fn print(&self) {
        println!("not implemented...");
    }
}

// Person構造体の定義
// 人物の基本情報を保持
struct Person {
    name: String,
    mail: String,
    age: i32,
}

// Student構造体の定義
// 学生の基本情報を保持（Personと同じフィールドを持つ）
struct Student {
    name: String,
    mail: String,
    age: i32,
}

// PersonにPrintトレイトを実装
// デフォルト実装を上書きして、Person用の出力形式を定義
impl Print for Person {
    fn print(&self) {
        println!(
            "Person: name={}, mail={}, age={}",
            &self.name, &self.mail, &self.age
        );
    }
}

// StudentにPrintトレイトを実装
// デフォルト実装を上書きして、Student用の出力形式を定義
impl Print for Student {
    fn print(&self) {
        println!(
            "Student: name={}, mail={}, age={}",
            &self.name, &self.mail, &self.age
        );
    }
}

// Personインスタンスを作成するファクトリ関数
// Box<Person>を返すことで、ヒープに配置されたPersonを返す
//
// なぜBoxでラップするのか：
// 1. 所有権の移動を柔軟にする
//    - Boxなしだと、Vecに入れる際に所有権が移動して元の変数が使えなくなる
// 2. メモリ効率
//    - Vec<Person>だと、Person全体がVec内にコピーされる
//    - Vec<Box<Person>>だと、ポインタ（8バイト）のみがVec内に格納される
// 3. 将来の拡張性
//    - Box<dyn Print>にすれば、異なる型を同じVecに入れられる
fn person(name: &str, mail: &str, age: i32) -> Box<Person> {
    Box::new(Person {
        name: String::from(name),
        mail: String::from(mail),
        age,
    })
}

// Studentインスタンスを作成するファクトリ関数
// Box<Student>を返すことで、ヒープに配置されたStudentを返す
fn student(name: &str, mail: &str, age: i32) -> Box<Student> {
    Box::new(Student {
        name: String::from(name),
        mail: String::from(mail),
        age,
    })
}

// ジェネリック関数：Printトレイトを実装する任意の型のVecを受け取る
// T: Print + ?Sized の意味：
// - T: Print → TはPrintトレイトを実装している必要がある
// - ?Sized → Tはサイズが不定でも良い（トレイトオブジェクトにも対応）
fn print_all<T: Print + ?Sized>(data: Vec<Box<T>>) {
    for item in data {
        item.print();
    }
}

fn main() {
    // Personインスタンスを作成
    let taro = person("Taro", "taro@example.com", 39);
    let jiro = person("Jiro", "jiro@example.com", 28);
    
    // Studentインスタンスを作成
    let hanako = student("Hanako", "hanako@example.com", 22);
    let sachiko = student("Sachiko", "sachiko@example.com", 19);
    
    // PersonのVecとStudentのVecを別々に作成
    // 異なる型なので、同じVecには入れられない
    
    // Vec<Box<Person>>のデータ構造：
    // - Vec: 動的配列（スタック上に配置）
    // - Box<Person>: Personへのポインタ（Vecの各要素）
    // - Person: 実際のデータ（ヒープ上に配置）
    //
    // メモリ配置のイメージ：
    // スタック            ヒープ
    // --------           --------
    // data_p [           
    //   ptr ─────────→ [Box] → Person { name: "Taro", ... }
    //   len: 2          [Box] → Person { name: "Jiro", ... }
    //   cap: 2
    // ]
    //
    // つまり、Personオブジェクトへのポインタ（Box）を複数格納するベクター
    //
    // Boxを使う利点：
    // - Vecの要素サイズが一定（ポインタサイズ）になる
    // - 大きな構造体でもVec内では小さなポインタのみ保持
    // - 将来Box<dyn Print>に変更すれば、PersonとStudentを同じVecに入れられる
    let data_p: Vec<Box<Person>> = vec![taro, jiro];
    let data_s: Vec<Box<Student>> = vec![hanako, sachiko];
    
    // ジェネリック関数print_allは、PersonでもStudentでも使える
    // これがジェネリクスの利点：コードの再利用
    print_all(data_p);    // Person用のprintメソッドが呼ばれる
    print_all(data_s);    // Student用のprintメソッドが呼ばれる
}