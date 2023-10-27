use std::collections::HashMap;

/// 暴力递归，最慢
fn fib_1(num: i32) -> i32 {
  if num == 1 || num == 2 {
    1
  } else {
    fib_1(num - 1) + fib_1(num - 2)
  }
}

/// 递归增加缓存，自顶向下，性能与fib_3差不多
fn fib_2(num: i32) -> i32 {
  let mut cache = HashMap::<i32, i32>::new();
  fn executor(cache: &mut HashMap<i32, i32>, num: i32) -> i32 {
    if num == 1 || num == 2 {
      return 1;
    }
    match cache.get(&num) {
      Some(r) => *r,
      None => {
        let r = executor(cache, num - 1) + executor(cache, num - 2);
        cache.insert(num, r);
        r
      }
    }
  }

  executor(&mut cache, num)
}

/// 自底向上，从小开始计算，使用循环而不是递归，依然有缓存，性能与fib_2差不多
fn fib_3(num: i32) -> i32 {
  if num == 0 {
    return 0;
  }
  let mut dp: HashMap<i32, i32> = HashMap::from([(0, 0), (1, 1)]);

  for i in 2..=num {
    dp.insert(i, dp.get(&(i - 1)).unwrap() + dp.get(&(i - 2)).unwrap());
  }

  *dp.get(&num).unwrap()
}

/// 根据逻辑得出fib逻辑依赖的变量为两个，因此将HashMap缓存缩减为两个变量，性能最快
fn fib_4(num: i32) -> i32 {
  if num == 0 || num == 1 {
    return num;
  }

  let mut dp_i_1 = 1;
  let mut dp_i_2 = 0;
  for _ in 2..=num {
    let res = dp_i_1 + dp_i_2;
    dp_i_2 = dp_i_1;
    dp_i_1 = res;
  }

  dp_i_1
}

#[cfg(test)]
mod test {
  use super::*;

  const TESTS: [(i32, i32); 5] = [(5, 5), (10, 55), (20, 6765), (17, 1597), (40, 102334155)];

  #[test]
  fn test_fib_1() {
    for (n, r) in TESTS {
      assert_eq!(fib_1(n), r)
    }
  }

  #[test]
  fn test_fib_2() {
    for (n, r) in TESTS {
      assert_eq!(fib_2(n), r)
    }
  }

  #[test]
  fn test_fib_3() {
    for (n, r) in TESTS {
      assert_eq!(fib_3(n), r)
    }
  }

  #[test]
  fn test_fib_4() {
    for (n, r) in TESTS {
      assert_eq!(fib_4(n), r)
    }
  }
}
