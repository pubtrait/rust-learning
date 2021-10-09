use add_one::one;

fn main() {
    println!("Hello, world!");

    let src = 33;
    let plus = one::add_one(src);

    println!("{} + 1 = {}", src, plus);
}
