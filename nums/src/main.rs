use std::io::repeat;

fn main() {
    println!("sort nums");

    let mut ao = [0, 1, 0, 3, 12];
    let mut ar = [0; 5];

    println!("before {:?}", ao);

    let mut index = 0;

    for v in ao {
        if v != 0 {
            ar[index] = v;
            index += 1;
        }
    }

    println!("the end {:?}", ar);

    println!("{} 2nd {}", "==".repeat(15), "==".repeat(15));

    let mut vec: Vec<i32> = vec![0, 1, 0, 3, 12];
    Solution::move_zeros(&mut vec);
    println!(" after: {:?}", vec);
}

struct Solution;

impl Solution {
    pub fn move_zeros(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut j = 0;
        while i < nums.len() {
            if nums[i] != 0 {
                nums[j] = nums[i];
                j += 1;
            }
            i += 1;
        }

        while j < nums.len() {
            nums[j] = 0;
            j += 1;
        }
    }
}
