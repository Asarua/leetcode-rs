struct Solution;
impl Solution {
  pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    fn executor(coins: &Vec<i32>, amount: i32) -> i32 {
      if amount == 0 {
        return 0;
      }
      if amount < 0 {
        return -1;
      }

      let mut res = i32::MAX;
      for coin in coins {
        let sub_problem = executor(coins, amount - coin);
        if sub_problem == -1 {
          continue;
        }
        res = [res, sub_problem + 1].into_iter().min().unwrap();
      }

      if res == i32::MAX {
        -1
      } else {
        res
      }
    }
    executor(&coins, amount)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  struct Item<const T: usize> {
    coins: [i32; T],
    amount: i32,
    res: i32,
  }

  enum Items {
    One(Item<3>),
    Two(Item<1>),
    Three(Item<1>),
  }

  const TESTS: [Items; 3] = [
    Items::One(Item {
      coins: [1, 2, 5],
      amount: 11,
      res: 3,
    }),
    Items::Two(Item {
      coins: [2],
      amount: 3,
      res: -1,
    }),
    Items::Three(Item {
      coins: [1],
      amount: 0,
      res: 0,
    }),
  ];

  #[test]
  fn test() {
    for item in TESTS {
      match item {
        Items::One(res) => check(res),
        Items::Two(res) => check(res),
        Items::Three(res) => check(res),
      }
    }

    fn check<const T: usize>(res: Item<T>) {
      assert_eq!(
        Solution::coin_change(Vec::from(res.coins), res.amount),
        res.res
      )
    }
  }
}
