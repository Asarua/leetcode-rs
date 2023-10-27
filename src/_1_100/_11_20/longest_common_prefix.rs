struct Solution;
impl Solution {
  pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut list = strs.iter().map(|s| s.chars()).collect::<Vec<_>>();
    let mut container = String::new();

    while let Some(standard) = list[0].next() {
      for i in &mut list[1..] {
        if let Some(cur) = i.next() {
          if standard != cur {
            return container;
          }
        } else {
          return container;
        }
      }
      container.push(standard)
    }

    container
  }
}

#[cfg(test)]
mod test {
  use super::*;

  const tests: [([&str; 3], &'static str); 2] = [
    (["flower", "flow", "flight"], "fl"),
    (["dog", "racecar", "car"], ""),
  ];

  #[test]
  fn test() {
    for (list, res) in tests {
      let r = Solution::longest_common_prefix(Vec::from(list.map(|s| s.to_string())));
      assert_eq!(r, res.to_string())
    }
  }
}
