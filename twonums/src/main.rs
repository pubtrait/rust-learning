use std::collections::HashMap;

fn main() {
    println!("find valid two numbers");
    let input = vec![2, 7, 11, 15];
    println!("got {:?}", Solution::plus_two(input, 9));

    let input = vec![2, 7, 11, 15, 12, 36];
    println!("target 26,  got {:?}", Solution::plus_two(input, 26));
}

struct Solution;

impl Solution {
    pub fn plus_two(nums: Vec<i32>, target: i32) -> Box<[usize]> {
        let mut vs = Box::new([0 as usize; 2]);
        let mut m = HashMap::<i32, usize>::new();

        for v in nums.iter().enumerate() {
            m.insert(*v.1, v.0);
        }

        for v in nums.iter() {
            let ano = target - v;
            if let Some(ai) = m.get(&ano) {
                if let Some(index) = m.get(v) {
                    vs[0] = *index;
                }
                vs[1] = *ai;
            }
        }
        return vs;
    }
}
