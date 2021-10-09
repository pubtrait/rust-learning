use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();

    let key = String::from("hello");
    m.insert(key, "world");
    let k1 = String::from("hello");
    let mv = m.get(&k1);
    println!("value: {}", mv.unwrap());

    for (k, v) in m {
        println!("key: {} value: {}", k, v);
    }
}
