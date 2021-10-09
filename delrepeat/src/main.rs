fn main() {
    println!("Hello, world!");
    let mut ns1 = vec![1, 1, 2];
    println!("before: {:?}", ns1);
    Solution::remove_repeat(&mut ns1);
    println!("after: {:?}", ns1);

    let mut ns2 = vec![0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 3, 3, 4];
    println!("before: {:?}", ns2);
    Solution::remove_repeat(&mut ns2);
    println!("after: {:?}", ns2);
}

struct Solution;

impl Solution {
    pub fn remove_repeat(nums: &mut Vec<i32>) -> usize {
        if nums.len() == 0 {
            return 0;
        }

        let mut index = 0;
        loop {
            if index == nums.len() {
                break;
            }
            if let Some(v) = nums.get(index) {
                for i in 0..index {
                    if *v == nums[i] {
                        nums.remove(index);
                        index -= 1;
                        break;
                    }
                }
            }
            index += 1;
        }
        nums.len()
    }
}
