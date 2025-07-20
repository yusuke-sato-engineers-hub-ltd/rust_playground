use rand::Rng;

#[derive(Debug, Clone, Copy)]
enum Kind {
    Person,
    Student,
}

impl Kind {
    fn random() -> Kind {
        let list = [Kind::Person, Kind::Student];
        let index = rand::thread_rng().gen_range(0..list.len());
        list[index]
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Mydata {
    name: String,
    kind: Kind,
}

fn mydata(name: &str) -> Mydata {
    Mydata {
        name: String::from(name),
        kind: Kind::random(),
    }
}

fn print_all(data: Vec<Mydata>) {
    for item in data {
        println!("{:?}", item);
    }
}

fn main() {
    let taro = mydata("Taro");
    let hanako = mydata("Hanako");
    let sachiko = mydata("Sachiko");
    let data = vec![taro, hanako, sachiko];
    print_all(data);
}
