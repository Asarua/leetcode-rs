use std::collections::HashMap;

struct Solution;
impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut container = HashMap::<i32, i32>::new();
    for (index, item) in nums.into_iter().enumerate() {
      if container.contains_key(&item) {
        return vec![*container.get(&item).unwrap(), index as i32];
      } else {
        container.insert(target - item, index as i32);
      }
    }
    vec![]
  }
}

#[cfg(test)]
mod test {
  use super::*;

  struct TestItem {
    nums: Vec<i32>,
    target: i32,
    res: Vec<i32>,
  }

  impl TestItem {
    fn assert_eq(&self) -> bool {
      let two_sum_res = Solution::two_sum(self.nums.clone(), self.target);
      if two_sum_res.len() == 2 {
        return two_sum_res == self.res
          || two_sum_res.into_iter().rev().collect::<Vec<_>>() == self.res;
      }
      false
    }
  }

  #[test]
  fn test() {
    let tests = vec![
      TestItem {
        nums: vec![2, 7, 11, 15],
        target: 9,
        res: vec![0, 1],
      },
      TestItem {
        nums: vec![3, 2, 4],
        target: 6,
        res: vec![1, 2],
      },
      TestItem {
        nums: vec![3, 3],
        target: 6,
        res: vec![1, 0],
      },
    ];

    for test in tests {
      assert!(test.assert_eq())
    }
  }
}
