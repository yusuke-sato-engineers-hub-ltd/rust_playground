use rand::Rng;

// 独自のエラー型を定義
// Result<T, E>のE（エラー型）として使用する
enum ErrKind {
    Caution,  // 注意レベルのエラー
    Danger,   // 危険レベルのエラー（パニックを起こす）
}

// ランダムにOption<i32>を生成する関数
// 0が出たらNone、それ以外はSome(数値)を返す
fn random() -> Option<i32> {
    let n = rand::thread_rng().gen_range(0..10);
    match n {
        0 => None,
        _ => Some(n),
    }
}

fn main() {
    // Option<i32>のベクターを作成
    let mut data = vec![];
    for _ in 0..10 {
        data.push(random());
    }
    print_all(data);
}

fn print_all(data: Vec<Option<i32>>) {
    for item in data {
        // print関数はResult<String, ErrKind>型を返す
        // Result型は成功(Ok)または失敗(Err)のどちらかの値を持つ
        let res = print(item);
        
        // match式でResult型の中身を取り出す
        match res {
            // Ok(s)の場合: print関数が成功し、String型の値sが返された
            Ok(s) => println!("--- {s} ---"),
            
            // Err(k)の場合: print関数が失敗し、ErrKind型のエラーkが返された
            // kにはCautionかDangerのどちらかが入っている
            Err(k) => match k {
                ErrKind::Caution => {
                    println!("*** CAUTION!! ***");
                }
                ErrKind::Danger => {
                    println!("DANGER!!");
                    panic!("DANGER ERRROR.");
                }
            },
        }
    }
}

// Option<i32>を受け取って、Result<String, ErrKind>を返す関数
// Result<String, ErrKind>の意味：
// - 成功時: Ok(String) - 文字列を返す
// - 失敗時: Err(ErrKind) - 独自定義したエラー型を返す
fn print(item: Option<i32>) -> Result<String, ErrKind> {
    match item {
        // Noneの場合はDangerエラーを返す
        None => Err(ErrKind::Danger),
        
        // Some(n)の場合は値を出力して、条件に応じて結果を返す
        Some(n) => {
            println!("No, {n}");
            if n == 1 {
                // 1の場合はCautionエラーを返す
                Err(ErrKind::Caution)
            } else {
                // それ以外は成功として"OK"文字列を返す
                Ok(String::from("OK"))
            }
        }
    }
}
