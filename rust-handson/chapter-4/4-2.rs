fn main() {
    let mut data = vec![];
    for n in 0..5 {
        data.push(Some(n));
    }
    print_all(data);
}

fn print_all(data: Vec<Option<i32>>) {
    for item in data {
        println!("{item:?}");
    }
}
