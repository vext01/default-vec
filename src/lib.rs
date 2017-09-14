#![feature(alloc)]

extern crate alloc;

#[cfg(test)] mod tests;

use alloc::raw_vec::RawVec;
use std::{mem, ptr};

/// A vector with a default value. All uninitialized values are set to `T::default()`.
pub struct DefaultVec<T: Default> {
    raw: RawVec<T>,
}

impl<T: Default> DefaultVec<T> {
    /// Creates an empty vector.
    pub fn new() -> Self {
        DefaultVec { raw: RawVec::new() }
    }

    /// Creates an empty vector with capacity for `cap` values.
    pub fn with_capacity(cap: usize) -> Self {
        let raw = RawVec::with_capacity(cap);
        for i in 0..raw.cap() {
            unsafe {
                ptr::write(raw.ptr().offset(i as isize), T::default());
            }
        }
        DefaultVec { raw }
    }

    /// Returns the total number of values the vector stores.
    pub fn capacity(&self) -> usize {
        self.raw.cap()
    }

    /// Resizes the vector to contain at least `new_cap` values.
    pub fn resize(&mut self, new_cap: usize) {
        let old_cap = self.capacity();
        self.raw.reserve(old_cap, new_cap.saturating_sub(old_cap));
        for i in old_cap..self.capacity() {
            unsafe {
                ptr::write(self.raw.ptr().offset(i as isize), T::default());
            }
        }
    }

    /// Get a reference to the value in position `idx`, panicking if the index is out of bounds.
    pub fn get(&self, idx: usize) -> &T {
        assert!(idx < self.capacity());
        unsafe {
            &*self.raw.ptr().offset(idx as isize)
        }
    }

    /// Get a reference to the value in position `idx`, resizing if the index is out of bounds.
    pub fn get_mut(&mut self, idx: usize) -> &mut T {
        if idx >= self.capacity() {
            self.resize(idx + 1);
        }
        unsafe {
            &mut *self.raw.ptr().offset(idx as isize)
        }
    }

    /// Inserts a value into the vector, returning the old value.
    pub fn insert(&mut self, idx: usize, val: T) -> T {
        mem::replace(self.get_mut(idx), val)
    }

    /// Removes a value from the vector.
    pub fn remove(&mut self, idx: usize) -> T {
        if idx < self.capacity() {
            mem::replace(self.get_mut(idx), T::default())
        } else {
            T::default()
        }
    }

    /// Starting from `start`, check each value in the vector and returns the index of the first one
    /// that satisfies it, or `capacity()` if none is found.
    pub fn find<F: FnMut(&T) -> bool>(&self, start: usize, mut pred: F) -> usize {
        for i in start..self.capacity() {
            if pred(self.get(i)) {
                return i;
            }
        }
        self.capacity()
    }
}