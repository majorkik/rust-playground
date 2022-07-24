#[cfg(test)]
mod add_two_numbers_test {
    use super::*;

    // Definition for singly-linked list.
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

        #[inline]
        fn add_next(&mut self, value: i32) {
            self.next = Some(Box::new(ListNode::new(value)));
        }
    }

    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head_result: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut tail_result = &mut head_result;
        let mut l1 = l1;
        let mut l2 = l2;
        let mut remainder = 0;

        while l1.is_some() || l2.is_some() || remainder != 0 {
            if l1.is_none() && l2.is_none() && remainder == 0 {
                break;
            }

            let value_l1 = l1.as_ref().map_or(0, |node| node.val);
            let value_l2 = l2.as_ref().map_or(0, |node| node.val);

            l1 = match l1 {
                Some(node) => node.next,
                None => None,
            };

            l2 = match l2 {
                Some(node) => node.next,
                None => None,
            };

            let sum = value_l1 + value_l2 + remainder;
            remainder = sum / 10;

            if let Some(tail) = tail_result {
                tail.add_next(sum % 10);
                tail_result = &mut tail.next;
            }
        }

        head_result?.next
    }

    fn build_lists(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut head;

        for num in values {
            if let Some(node) = tail {
                node.add_next(num);
                tail = &mut node.next;
            }
        }

        head?.next
    }

    fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = vec![];
        let mut list = &list;

        while list.is_some() {
            match list {
                Some(node) => {
                    result.push(node.val);
                    list = &node.next;
                }
                _ => {}
            };
        }

        result
    }

    #[test]
    fn empty_lists() {
        let result = add_two_numbers(None, None);

        assert_eq!(result, None);
    }

    #[test]
    fn short_lists_of_equal_length() {
        let l1 = build_lists(vec![9, 9, 9]);
        let l2 = build_lists(vec![9, 9, 9]);
        let expected_result = vec![8, 9, 9, 1];

        let result = add_two_numbers(l1, l2);

        assert_eq!(list_to_vec(result), expected_result);
    }
}
