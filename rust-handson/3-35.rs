#[derive(Debug)]
#[allow(dead_code)]
struct Sample<T: core::fmt::Debug> {
    name: String,
    values: Vec<T>,
}

impl<T: core::fmt::Debug> Sample<T> {
    fn print_values(&self) {
        println!("*** {} ***", &self.name);
        for item in &self.values {
            println!("{item:?}");
        }
    }
}

fn sample<T: core::fmt::Debug>(name: &str, values: Vec<T>) -> Sample<T> {
    Sample {
        name: String::from(name),
        values,
    }
}

fn main() {
    let taro = sample("Taro", vec![123, 456, 789]);
    taro.print_values();
    let hanako = sample("Hanako", vec!["Hello", "Welcome", "Bye!"]);
    hanako.print_values();
}
