fn main() {
    println!("stack test");

    let mut s = Solution::new();
    s.push(1);
    s.push(3);
    s.push(2);
    s.push(123);
    s.push(13);

    let v = s.pop();
    println!("v {}", v);

    println!("top {}", s.top());

    if let Some(min) = s.get_min() {
        println!("min {}", min);
    }
}

struct Solution<T> {
    buf: Vec<T>,
}

impl<T> Solution<T>
where
    T: PartialOrd,
{
    pub fn new() -> Self {
        Solution { buf: Vec::new() }
    }

    pub fn push(&mut self, v: T) {
        self.buf.push(v)
    }

    pub fn pop(&mut self) -> T {
        self.buf.remove(self.buf.len() - 1)
    }

    pub fn top(&mut self) -> T {
        self.buf.remove(0)
    }

    pub fn get_min(&mut self) -> Option<&T> {
        if let Some(mut first) = self.buf.get(0) {
            for i in 0..self.buf.len() {
                if self.buf[i] < *first {
                    first = &self.buf[i];
                }
            }
            return Some(first);
        } else {
            None
        }
    }
}
