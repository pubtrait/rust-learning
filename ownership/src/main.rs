fn main() {
    println!("ownership");

    let v = vec!["Java", "Rust", "Python"];

    for s in v.into_iter() {
        match s {
            "Rust" => println!("Niubility"),
            _ => println!("{}", s),
        }
    }

    // println!("{:?}", v);
    //
    println!("{}", "===".repeat(20));

    let vr = vec!["Java", "Rust", "Python"];

    for s in vr.iter() {
        match s {
            &"Rust" => println!("Niubility"),
            _ => println!("{}", s),
        }
    }

    println!("output {:?}", vr);

    println!("{}", "===".repeat(20));

    let mut mvr = vec!["Java", "Rust", "Python"];

    for s in mvr.iter_mut() {
        match s {
            &mut "Rust" => *s = "Niubility",
            _ => println!("{}", s),
        }
    }

    println!("mut output {:?}", mvr);
}
