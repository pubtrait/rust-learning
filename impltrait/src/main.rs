trait Reader {
    fn read(&self) -> &String;
}

struct Demo {
    name: String,
}

impl Demo {
    fn new(name: String) -> impl Reader {
        Demo { name }
    }
}

impl Reader for Demo {
    fn read(&self) -> &String {
        &self.name
    }
}

fn print(r: &dyn Reader) {
    println!("got {:?}", r.read());
}

fn main() {
    let d = Demo::new(String::from("hello trait"));
    print(&d);
}
