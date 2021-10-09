fn main() {
    println!("Hello, world!");

    let v = vec![1, 2, 3];

    let vs: Vec<u32> = v
        .iter()
        .map(|x| x * 2)
        .filter(|x| {
            if x % 2 == 0 {
                return true;
            } else {
                return false;
            }
        })
        .collect();

    println!("got {:?}", vs);

    let vs: Vec<u32> = v
        .iter()
        .map(|x| x + 3)
        .filter(|x| {
            if x % 3 == 0 {
                return true;
            }
            false
        })
        .collect();

    println!("vs {:?}", vs);

    let vstr = vec!["hello", "world", "sunsc", "suncm"];

    let vres = vstr.iter().any(|x| {
        if x == &"suncm" {
            return true;
        }
        false
    });

    println!("vres {}", vres);
}
