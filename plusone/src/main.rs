fn main() {
    println!("plus one");

    let mut nums: Vec<i32> = vec![1, 0, 8, 7, 9];
    println!("nums {:?}", nums);
    nums.reverse();
    println!("rns {:?}", nums);
    nums.push(9);
    println!("rns {:?}", nums);

    println!("{} 2nd {}", "==".repeat(15), "==".repeat(15));
    let mut ns1: Vec<i32> = vec![1, 2, 3];
    let mut ns2: Vec<i32> = vec![9, 9, 9];

    println!("ns1 before {:?}", ns1);
    Solution::plus_one(&mut ns1);
    println!("ns1 after {:?}", ns1);

    println!("ns2 before {:?}", ns2);
    Solution::plus_one(&mut ns2);
    println!("ns2 after {:?}", ns2);
}

struct Solution;

impl Solution {
    pub fn plus_one(nums: &mut Vec<i32>) {
        let mut tmp = 0;
        nums.reverse();
        for i in 0..nums.len() {
            if let Some(v) = nums.get(i) {
                tmp = (v + 1) % 10;
                if tmp == 0 {
                    if i < nums.len() - 1 {
                        nums[i] = 0;
                    } else {
                        nums[i] = 0;
                        nums.push(1);
                    }
                    continue;
                }
                nums[i] = tmp;
                break;
            }
        }
        nums.reverse()
    }
}
