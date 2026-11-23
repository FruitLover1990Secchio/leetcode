pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut count = 0;
        let mut start = 0;
        if nums.len() == 0 {
            return 0;
        }
        let mut end = nums.len() - 1;

        while start <= end {

            if nums[start] == val {
                while nums[end] == val && end > start {
                    end -= 1;
                }
                if end != start {
                    count += 1;
                    let temp = nums[end];
                    nums[end] = nums[start];
                    nums[start] = temp;
                    end -= 1;
                }
            } else {
                count += 1;
            }
            start += 1;
        }

        return count;

    }

#[cfg(test)] // only compiled when running `cargo test`
mod tests {
    use super::*; // imports `remove_element` from the file above

    #[test]
    fn example_1() {
        let mut nums = vec![3, 2, 2, 3];
        let k = remove_element(&mut nums, 3);
        assert_eq!(k, 2);
    }
}