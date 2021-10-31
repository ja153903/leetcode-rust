#![allow(dead_code)]

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}

type Link = Option<Box<ListNode>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct List {
    pub head: Link,
}

impl List {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, val: i32) {
        let new_node = Box::new(ListNode {
            val,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }
}

pub fn create_linked_list_from_vec(v: &[i32]) -> List {
    let mut list = List::new();

    for &num in v.iter().rev() {
        list.push(num);
    }

    list
}

pub fn create_vec_from_linked_list(mut list: List) -> Vec<i32> {
    let mut v = Vec::new();
    let mut head = list.head.as_mut();

    while let Some(node) = head {
        v.push(node.val);
        head = node.next.as_mut();
    }

    v
}
