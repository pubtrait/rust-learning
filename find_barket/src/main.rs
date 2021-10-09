fn main() {
    println!("check symbol valid");
    let mut so = Solution::new();

    let i1 = "()[]{}";
    println!("val: {} got {}", i1, so.check(i1.to_string()));

    let i2 = "{[]}";
    println!("val: {} got {}", i2, so.check(i2.to_string()));

    let i3 = "([)]";
    println!("val: {} got {}", i3, so.check(i3.to_string()));
}

struct Solution {
    stack: Vec<u8>,
}

impl Solution {
    pub fn new() -> Self {
        Solution {
            stack: Vec::<u8>::new(),
        }
    }

    fn push(&mut self, symbol: u8) {
        self.stack.push(symbol)
    }

    fn pop(&mut self) -> Option<u8> {
        self.stack.pop()
    }

    pub fn check(&mut self, s: String) -> bool {
        for c in s.as_bytes() {
            if *c == '[' as u8 || *c == '{' as u8 || *c == '(' as u8 {
                self.push(*c);
            } else {
                if let Some(r) = self.pop() {
                    if *c == '}' as u8 && r != '{' as u8 {
                        return false;
                    }
                    if *c == ')' as u8 && r != '(' as u8 {
                        return false;
                    }
                    if *c == ']' as u8 && r != '[' as u8 {
                        return false;
                    }
                }
            }
        }
        true
    }
}
