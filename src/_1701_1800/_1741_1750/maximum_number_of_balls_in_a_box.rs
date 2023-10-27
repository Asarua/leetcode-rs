use std::collections::HashMap;

struct Solution;

impl Solution {
  pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
    let mut container = HashMap::<u32, i32>::new();

    for i in low_limit..=high_limit {
      let sum = i
        .to_string()
        .chars()
        .fold(0, |a, b| a + b.to_digit(10).unwrap());
      container.entry(sum).and_modify(|u| *u += 1).or_insert(1);
    }

    container.iter().map(|(k, v)| *v).max().unwrap_or_default()
  }
}

#[cfg(test)]
mod test {
  use super::*;

  const tests: [(i32, i32, i32); 3] = [(1, 10, 2), (5, 15, 2), (19, 28, 2)];

  #[test]
  fn test() {
    for (a, b, c) in tests {
      assert_eq!(Solution::count_balls(a, b), c)
    }
  }
}
