struct Solution;
impl Solution {
  pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut container = Vec::with_capacity(num_rows as usize);
    container.insert(0, vec![1]);
    for i in 1..num_rows {
      let mut col = Vec::with_capacity(i as usize);
      col.insert(0, 1);
      col.insert(i as usize, 1);
      let pre_last = container.iter().last().unwrap();
      for u in 0..=pre_last.len() {
        col.insert(
          u + 1,
          pre_last.get(u).unwrap_or(&0) + pre_last.get(u - 1).unwrap_or(&0),
        );
      }

      container.push(col);
    }
    container
  }
}

#[cfg(test)]
mod test {
  use super::*;

  const TESTS: [(i32, &[&[i32]]); 2] = [
    (1, &[&[1]]),
    (
      5,
      &[&[1], &[1, 1], &[1, 2, 1], &[1, 3, 3, 1], &[1, 4, 6, 4, 1]],
    ),
  ];

  #[test]
  fn test() {
    for (num_row, res) in TESTS {
      assert_eq!(
        Solution::generate(num_row),
        res
          .into_iter()
          .map(|r| Vec::from(*r))
          .collect::<Vec<Vec<i32>>>()
      )
    }
  }
}
