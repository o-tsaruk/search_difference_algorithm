fn main() {
    let mut array = [99,100,1,2,3,5,7];
    let _tmp = search_max_difference(&mut array);
}

fn search_max_difference(nums: &mut [i32]) -> Vec<i32> {
    if nums.len() < 2 {
        return vec![]
    };

    let mut max_difference = 0;
    let mut index1= 1;
    let mut index2= 1;
    let mut i = 0;
    let mut min = nums[0];

    while i < nums.len() {
        if nums[i] < min {
            min = nums[i];
            index1 = i+1;
        }

        if nums[i] - min > max_difference {
            index2 = i+1;
            max_difference = nums[i] - min;
        }

        i += 1;
    }

    if max_difference == 0 {
        return vec![];
    }

    vec![index1 as i32,index2 as i32]
}

#[cfg(test)]
mod tests {
    use crate::search_max_difference;

    #[test]
    fn test_search_max_difference() {
        assert_eq!(search_max_difference(&mut [99,100,1,2,3,5,7]), vec![3,7]);
        assert_eq!(search_max_difference(&mut [9,5,0,100,1,15]), vec![3,4]);
        assert_eq!(search_max_difference(&mut [12,0,17,3,2,26,7,2,8]), vec![2,6]);
        assert_eq!(search_max_difference(&mut [4,2,1]), vec![]);
        assert_eq!(search_max_difference(&mut [1,2]), vec![1,2]);
        assert_eq!(search_max_difference(&mut [1,1]), vec![]);
        assert_eq!(search_max_difference(&mut [0]), vec![]);
    }
}