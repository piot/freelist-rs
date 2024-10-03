/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/freelist-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use freelist_rs::{FreeList, FreeListError};

#[test_log::test]
fn single_allocations() {
    let mut free_list = FreeList::new(4);
    assert_eq!(free_list.allocate(), Some(0));
    assert_eq!(free_list.allocate(), Some(1));
    assert_eq!(free_list.free(1), Ok(()));
    assert_eq!(free_list.free(1), Err(FreeListError::ItemNotInTheList));
    assert_eq!(free_list.allocate(), Some(2));
    assert_eq!(free_list.allocate(), Some(3));
    assert_eq!(free_list.allocate(), Some(1));
    assert_eq!(free_list.allocate(), None);
}

#[test_log::test]
fn multiple_allocations() {
    let mut free_list = FreeList::<u8>::new(4);
    assert_eq!(free_list.allocate_count(3), Some([0, 1, 2].into()));
    assert_eq!(free_list.free_slice(&[0, 1, 2]), Ok(()));
    assert_eq!(free_list.len(), 4);
    assert_eq!(free_list.allocate(), Some(3));
    assert_eq!(free_list.allocate(), Some(0));
    assert_eq!(free_list.len(), 2);
}
