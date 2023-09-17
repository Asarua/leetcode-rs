use std::str::FromStr;

struct Solution;

#[derive(PartialEq)]
enum Roman {
  I,
  IV,
  V,
  IX,
  X,
  XL,
  L,
  XC,
  C,
  CD,
  D,
  CM,
  M,
}

const SHOULD_CHECK_AFTER_ROMANS: [Roman; 3] = [Roman::I, Roman::X, Roman::C];

fn should_check_after(char: &str) -> bool {
  SHOULD_CHECK_AFTER_ROMANS.contains(&Roman::from_str(char).unwrap())
}

impl FromStr for Roman {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "I" => Ok(Roman::I),
      "IV" => Ok(Roman::IV),
      "V" => Ok(Roman::V),
      "IX" => Ok(Roman::IX),
      "X" => Ok(Roman::X),
      "XL" => Ok(Roman::XL),
      "L" => Ok(Roman::L),
      "XC" => Ok(Roman::XC),
      "C" => Ok(Roman::C),
      "CD" => Ok(Roman::CD),
      "D" => Ok(Roman::D),
      "CM" => Ok(Roman::CM),
      "M" => Ok(Roman::M),
      _ => Err(()),
    }
  }
}

impl Roman {
  fn to_i32(self) -> i32 {
    match self {
      Self::I => 1,
      Self::IV => 4,
      Self::V => 5,
      Self::IX => 9,
      Self::X => 10,
      Self::XL => 40,
      Self::L => 50,
      Self::XC => 90,
      Self::C => 100,
      Self::CD => 400,
      Self::D => 500,
      Self::CM => 900,
      Self::M => 1000,
      _ => panic!(),
    }
  }
}

impl Solution {
  pub fn roman_to_int(s: String) -> i32 {
    let mut res = 0;
    let mut pre_check = String::new();
    let chars = s.chars();
    let char_size = chars.clone().count();
    for (index, char) in chars.clone().enumerate() {
      if pre_check.len() > 0 {
        let mut char_sum = String::from(pre_check.clone());
        char_sum.push(char);

        if let Ok(ramon) = Roman::from_str(&char_sum) {
          res += ramon.to_i32();
          pre_check = String::new();
          continue;
        } else {
          res += Roman::from_str(pre_check.as_str()).unwrap().to_i32();
          pre_check = String::new();
        }
      }

      let char_string = char.to_string();
      if should_check_after(char_string.as_str()) && pre_check.len() == 0 && index != char_size - 1
      {
        pre_check = char_string.clone();
      } else {
        res += Roman::from_str(char_string.as_str()).unwrap().to_i32();
      }
    }

    res
  }
}

#[cfg(test)]
mod test {
  use super::*;

  const tests: [(&str, i32); 3] = [("III", 3), ("LVIII", 58), ("MCMXCIV", 1994)];

  #[test]
  fn test() {
    for (s, res) in tests {
      assert_eq!(Solution::roman_to_int(s.to_string()), res)
    }
  }
}
