struct Solution;
impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
    if let Ok(y) = x
      .to_string()
      .chars()
      .rev()
      .collect::<String>()
      .parse::<i32>()
    {
      return x == y;
    }
    false
  }
}

#[cfg(test)]
mod test {
  use super::*;

  const tests: [(i32, bool); 3] = [(121, true), (-121, false), (10, false)];

  #[test]
  fn test() {
    for (num, res) in tests {
      assert!(Solution::is_palindrome(num) == res)
    }
  }
}
