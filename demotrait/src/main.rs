


trait Descriptive {
    fn describe(&self) -> String {
        String::from("[Object]")
    }
}

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u8
}

// impl Descriptive for Person {}

impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}

fn output<T: Descriptive>(object: T) {
    println!("{}", object.describe());
}


fn person() -> impl Descriptive {
    Person {
        name: String::from("function person"),
        age: 30
    }
}

fn main() {
    println!("Hello, world!");

    let cali = Person {
        name: String::from("Cali"),
        age: 24
    };
    println!("{}", cali.describe());

    let person = person();
    output(person);
}
