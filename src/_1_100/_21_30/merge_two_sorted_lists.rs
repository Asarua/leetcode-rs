#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

struct Solution;
impl Solution {
  pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    // let mut dummy = ListNode::new(-1);
    // let l = &mut dummy;

    // while let (Some(res1), Some(res2)) = (list1.as_ref(), list2.as_ref()) {
    //   l.next = if res1.val > res2.val {
    //     res2
    //   } else {
    //     res1
    //   };
    // }

    // if let Some(res1) = list1 {

    // }

    // if let Some(res2) = list2 {}

    None
  }
}
