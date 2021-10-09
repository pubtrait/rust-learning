fn main() {
    let input = vec![1, 3, -1, -3, 5, 3, 6, 7];
    // expected result: [3,3,5,5,6,7]
    let res = Solution::new().find_max(input, 3);
    println!("res: {:?}", res);
}

struct Solution;

impl Solution {
    pub fn new() -> Solution {
        Solution
    }

    pub fn find_max(&self, vi: Vec<i32>, k: usize) -> Vec<i32> {
        if vi.len() == 0 {
            return vi;
        }
        let mut res = Vec::<i32>::new();

        for i in k..=vi.len() {
            res.push(self.get_max(&vi[0 + i - k..i]));
        }
        res
    }

    fn get_max(&self, vi: &[i32]) -> i32 {
        let mut max = vi[0];
        for i in 0..vi.len() {
            if max < vi[i] {
                max = vi[i];
            }
        }
        max
    }
}
