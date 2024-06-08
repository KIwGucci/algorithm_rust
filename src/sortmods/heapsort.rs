use std::fmt::Debug;

#[derive(Debug)]
struct MyBina<T: Ord> {
    data: Vec<T>,
}
impl<T: Ord + Clone> MyBina<T> {
    fn new(data: &mut [T]) -> Self {
        let mut item = Self {
            data: Vec::from(data),
        };
        item.sorted_swap_child(0);
        item
    }
    fn len(&self) -> usize {
        self.data.len()
    }

    fn parent(&self, current: usize) -> Option<usize> {
        if current < 1 {
            None
        } else {
            Some((current - 1) / 2)
        }
    }

    fn left(&self, current: usize) -> Option<usize> {
        let left_index = 2 * current + 1;
        if self.len() < 1 || left_index > (self.len() - 1) {
            None
        } else {
            Some(left_index)
        }
    }
    fn right(&self, current: usize) -> Option<usize> {
        let right_index = 2 * current + 2;
        if self.len() < 2 || right_index > self.len() - 1 {
            None
        } else {
            Some(right_index)
        }
    }
    fn sorted_swap(&mut self, current: usize) {
        let mut is_sorted = false;

        if let Some(left) = self.left(current) {
            if self.data[current] < self.data[left] {
                self.data.swap(current, left);
                is_sorted = true;
            }
        };

        if let Some(right) = self.right(current) {
            if self.data[current] < self.data[right] {
                self.data.swap(current, right);
                is_sorted = true;
            }
        };

        if is_sorted {
            if let Some(parent) = self.parent(current) {
                self.sorted_swap(parent);
            }
        }
    }
    fn sorted_swap_child(&mut self, current: usize) {
        if let Some(left) = self.left(current) {
            if self.data[current] < self.data[left] {
                self.data.swap(current, left);
            }
            self.sorted_swap_child(left);
        };

        if let Some(right) = self.right(current) {
            if self.data[current] < self.data[right] {
                self.data.swap(current, right);
            }
            self.sorted_swap(right);
            self.sorted_swap_child(right);
        };
    }
    fn sorted_pop(&mut self) -> Option<T> {
        if !self.data.is_empty() {
            let last = self.len() - 1;
            self.data.swap(0, last);
            let item = self.data.pop();
            self.sorted_swap_child(0);
            item
        } else {
            None
        }
    }
}

pub fn sort<R: Ord + Debug + Clone>(x: &mut [R]) -> Vec<R> {
    let mut sort_heap = MyBina::new(x);
    let mut sort_vec = Vec::new();
    'l: loop {
        match sort_heap.sorted_pop() {
            Some(item) => sort_vec.push(item),
            None => break 'l,
        }
    }
    sort_vec
}
