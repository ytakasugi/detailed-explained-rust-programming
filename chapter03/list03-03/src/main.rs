#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new()
        }
    }
}

fn main() {
    let f3 = File::new("f3.text");

    let f3_name = &f3.name;
    let f3_length = &f3.data.len();

    println!("{:?}", f3);
    println!("{} is {} byte long", f3_name, f3_length);
}
