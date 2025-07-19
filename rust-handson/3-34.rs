#[derive(Debug)]
#[allow(dead_code)]
struct Sample<T> {
    name: String,
    value: T,
}

fn sample<T>(name: &str, value: T) -> Sample<T> {
    Sample {
        name: String::from(name),
        value,
    }
}

fn main() {
    let taro = sample("Taro", "this is a message.");
    let hanako = sample("Hanako", 1234);
    println!("Taro: {:#?}", taro);
    println!("Hanako: {:#?}", hanako);
}
