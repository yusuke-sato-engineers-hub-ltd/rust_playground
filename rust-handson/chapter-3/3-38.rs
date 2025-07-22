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
// 3. 異なる型を同じVecに入れる（トレイトオブジェクト）
//    - Box<dyn Print>として、PersonとStudentを同じVecに入れられる
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
//
// 注意：この関数は実際にはトレイトオブジェクト（dyn Print）で呼び出される
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

    // ★重要な変更点：PersonとStudentを同じVecに入れる
    // Vec<Box<dyn Print>>を使用することで、異なる型を同じVecに格納
    //
    // Vec<Box<dyn Print>>のデータ構造：
    // - Vec: 動的配列（スタック上に配置）
    // - Box<dyn Print>: Printトレイトを実装する任意の型へのポインタ
    // - Person/Student: 実際のデータ（ヒープ上に配置）
    //
    // メモリ配置のイメージ：
    // スタック            ヒープ
    // --------           --------
    // data [
    //   ptr ─────────→ [Box] → Person { name: "Taro", ... }
    //   len: 4          [Box] → Person { name: "Jiro", ... }
    //   cap: 4          [Box] → Student { name: "Hanako", ... }
    // ]                 [Box] → Student { name: "Sachiko", ... }
    //
    // dyn Printによるトレイトオブジェクトの利点：
    // - 異なる型（PersonとStudent）を同じVecに格納できる
    // - 実行時にどちらの型のprintメソッドを呼ぶか決定（動的ディスパッチ）
    // - 新しい型を追加してもprint_all関数は変更不要
    let data: Vec<Box<dyn Print>> = vec![taro, jiro, hanako, sachiko];

    // print_all関数にトレイトオブジェクトのVecを渡す
    // 各要素の実際の型に応じて、適切なprintメソッドが呼ばれる
    print_all(data);
}
