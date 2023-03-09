#![allow(dead_code)]

// https://rtoch.com/posts/rust-doubly-linked-list/

use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

type DequeNodePtr<T> = Rc<RefCell<DequeNode<T>>>;

pub struct DequeNode<T> {
    value: T,
    prev: Option<DequeNodePtr<T>>,
    next: Option<DequeNodePtr<T>>,
}

impl<T> DequeNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            prev: None,
            next: None,
        }
    }
}

impl<T> From<DequeNode<T>> for DequeNodePtr<T> {
    fn from(node: DequeNode<T>) -> Self {
        Rc::new(RefCell::new(node))
    }
}

pub struct Deque<T> {
    head: Option<DequeNodePtr<T>>,
    tail: Option<DequeNodePtr<T>>,
    size: usize,
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Deque {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push_front(&mut self, value: T) {
        let new_node: DequeNodePtr<T> = DequeNode::new(value).into();

        if let Some(curr_head) = self.head.take() {
            curr_head.borrow_mut().prev = Some(new_node.clone());
            new_node.borrow_mut().next = Some(curr_head);
            self.head = Some(new_node);
            self.size += 1;
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
            self.size = 1;
        }
    }

    pub fn push_back(&mut self, value: T) {
        let new_node: DequeNodePtr<T> = DequeNode::new(value).into();

        if let Some(curr_tail) = self.head.take() {
            curr_tail.borrow_mut().next = Some(new_node.clone());
            new_node.borrow_mut().prev = Some(curr_tail);
            self.head = Some(new_node);
            self.size += 1;
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
            self.size = 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|curr_head| {
            self.size -= 1;
            match curr_head.borrow_mut().next.take() {
                None => {
                    // deque is now empty after popping curr_head
                    self.tail.take();
                }
                Some(new_head) => {
                    // set curr_head.next as the new head
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
            }
            Rc::try_unwrap(curr_head).ok().unwrap().into_inner().value
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|curr_tail| {
            self.size -= 1;
            match curr_tail.borrow_mut().prev.take() {
                None => {
                    // deque is now empty after popping curr_tail
                    self.head.take();
                }
                Some(new_tail) => {
                    // set curr_tail.prev as the new tail
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
            }
            Rc::try_unwrap(curr_tail).ok().unwrap().into_inner().value
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|curr_head| Ref::map(curr_head.borrow(), |curr_head| &curr_head.value))
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|curr_tail| Ref::map(curr_tail.borrow(), |curr_tail| &curr_tail.value))
    }
}
