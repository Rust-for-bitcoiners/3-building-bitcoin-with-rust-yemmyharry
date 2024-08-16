#![allow(unused)]

/* This module will be taught in the class */

pub struct MyLinkedList<T> {
    pub head: Option<Node<T>>,
}

pub struct Node<T> {
   pub val: T,
   pub next: Option<Box<Node<T>>>,
}

impl<T> MyLinkedList<T> {
    pub fn new() -> Self {
        MyLinkedList { head: None }
    }

    // Add more methods as needed
}
