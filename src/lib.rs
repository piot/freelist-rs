use core::fmt;
use std::{error::Error, fmt::Display};

/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/nimble-rust/nimble
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
pub struct FreeList<T> {
    free_numbers: Vec<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FreeListError {
    ItemNotInTheList,
}

impl Display for FreeListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "freelist-error:{:?}", self)
    }
}

impl Error for FreeListError {}

#[allow(unused)]
impl<T: Copy + PartialEq + fmt::Debug + From<usize>> FreeList<T> {
    pub fn new(count: usize) -> Self {
        let mut free_numbers = Vec::with_capacity(count);
        for i in (0..count).rev() {
            free_numbers.push(i.into());
        }
        Self { free_numbers }
    }

    pub fn allocate(&mut self) -> Option<T> {
        self.free_numbers.pop()
    }

    pub fn allocate_count(&mut self, num: usize) -> Option<Vec<T>> {
        if num == 0 {
            return Some(Vec::new());
        }
        if self.free_numbers.len() < num {
            return None;
        }

        let mut allocated = Vec::with_capacity(num); // Pre-allocate space for the new Vec
        for _ in 0..num {
            let val = self
                .free_numbers
                .pop()
                .expect("we have checked the len() previously");
            allocated.push(val);
        }
        Some(allocated)
    }

    pub fn free(&mut self, id: T) -> Result<(), FreeListError> {
        if self.free_numbers.contains(&id) {
            Err(FreeListError::ItemNotInTheList)
        } else {
            self.free_numbers.insert(0, id);
            Ok(())
        }
    }

    pub fn free_slice(&mut self, ids: &[T]) -> Result<(), FreeListError> {
        for id in ids {
            if self.free_numbers.contains(id) {
                return Err(FreeListError::ItemNotInTheList);
            } else {
                self.free_numbers.insert(0, *id);
            }
        }
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.free_numbers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.free_numbers.is_empty()
    }
}
