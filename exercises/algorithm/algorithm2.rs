/*
    double linked list reverse
    This problem requires you to reverse a doubly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::mem::swap;
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_mut(&mut self, index: i32) -> Option<&mut T> {
        self.get_ith_node_mut(self.start, index)
    }

    fn get_ith_node_mut(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&mut T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &mut (*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node_mut(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    /// 交换首尾对应结点里面的值来实现反转，作弊了
    pub fn reverse_val(&mut self) {
        let mut i: i32 = 0;
        let mut j: i32 = (self.length - 1) as i32;
        while i < j {
            let a = self.get_mut(i).unwrap() as *mut T;
            let b = self.get_mut(j).unwrap() as *mut T;
            unsafe {
                swap(&mut *a, &mut *b); // 通过指针操作交换值
            }
            i += 1;
            j -= 1;
        }
    }
    /// 还是应该通过指针的操作，改变拓扑结构，这样能适应val的类型更复杂的情况
    pub fn reverse(&mut self) {
        // 双指针
        let mut cur = self.start;
        let mut prev = None;
        while let Some(mut now) = cur {
            let next;
            unsafe {
                // next最后会为None，也可以正确运行
                next = now.as_mut().next;
                (now.as_mut().next, now.as_mut().prev) = (prev, next);
            }
            (prev, cur) = (cur, next);
        }
        // 最后要把self的头尾给换过来
        std::mem::swap(&mut self.start, &mut self.end);
        // 下面的更适用于于最简单的两值交换，会生成临时元组，开销大一点
        // (self.start, self.end) = (self.end, self.start);
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_reverse_linked_list_1() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![2, 3, 5, 11, 9, 7];
        let reverse_vec = vec![7, 9, 11, 5, 3, 2];
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_reverse_linked_list_2() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![34, 56, 78, 25, 90, 10, 19, 34, 21, 45];
        let reverse_vec = vec![45, 21, 34, 19, 10, 90, 25, 78, 56, 34];
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }
}
