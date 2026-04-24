fn main() {
    let nums = [3, 1, 7, 9, 5];
    let nums1 = [1, 2, 4, 8, 16];
    let nums2 = [1.5, 3, 0, 4.5, 6];
    let nums3 = [5, -1, 3, 1, -3];
    let all_nums = [nums, nums1, nums2, nums3];
    println!("Hello, world!");
}

struct Acc {
    prev: Option<i32>,
    is_prog: Bool,
}

fn handle_nums(nums: [i32; 5]) {
    nums.iter().fold(
        Acc {
            prev: None,
            prev_diff: None
            is_prog: false,
        },
        |acc, curr| -> Acc {
            let Acc { prev, is_prog } = acc;
            if let num = Some(prev) {
                let diff = num - prev

            } else {
                Acc {
                    prev: Some(curr),
                    is_prog: false,
                }
            }
        },
    )
}
