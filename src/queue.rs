#![allow(dead_code)]

use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

type QueueNodePtr<T> = Rc<RefCell<QueueNode<T>>>;

pub struct QueueNode<T> {
    value: T,
    next: Option<QueueNodePtr<T>>,
}

impl<T> QueueNode<T> {
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

impl<T> From<QueueNode<T>> for QueueNodePtr<T> {
    fn from(node: QueueNode<T>) -> Self {
        Rc::new(RefCell::new(node))
    }
}

pub struct Queue<T> {
    head: Option<QueueNodePtr<T>>,
    tail: Option<QueueNodePtr<T>>,
    size: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
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

    pub fn push(&mut self, value: T) {
        let new_node: QueueNodePtr<T> = QueueNode::new(value).into();
        if let Some(curr_tail) = self.tail.take() {
            curr_tail.borrow_mut().next = Some(new_node.clone());
            self.tail = Some(new_node);
            self.size += 1;
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
            self.size = 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|curr_head| {
            self.size -= 1;
            match curr_head.borrow_mut().next.take() {
                None => {
                    self.tail.take();
                }
                Some(new_head) => {
                    self.head = Some(new_head);
                }
            }
            Rc::try_unwrap(curr_head).ok().unwrap().into_inner().value
        })
    }

    pub fn peek(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|curr_head| Ref::map(curr_head.borrow(), |curr_head| &curr_head.value))
    }
}
