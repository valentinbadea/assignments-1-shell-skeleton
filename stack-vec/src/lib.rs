#![no_std]

#[cfg(test)]
mod tests;
use core::ops::Deref;
use core::ops::DerefMut;
use core::ops::Index;
use core::ops::IndexMut;
/// A contiguous array type backed by a slice.
///
/// `StackVec`'s functionality is similar to that of `std::Vec`. You can `push`
/// and `pop` and iterate over the vector. Unlike `Vec`, however, `StackVec`
/// requires no memory allocation as it is backed by a user-supplied slice. As a
/// result, `StackVec`'s capacity is _bounded_ by the user-supplied slice. This
/// results in `push` being fallible: if `push` is called when the vector is
/// full, an `Err` is returned.
pub struct StackVec<'a, T: 'a> {
    storage: &'a mut [T],
    len: usize,
    capacity: usize,
}

impl<'a, T: 'a> StackVec<'a, T> {
    /// Constructs a new, empty `StackVec<T>` using `storage` as the backing
    /// store. The returned `StackVec` will be able to hold `storage.len()`
    /// values.
    pub fn new(storage: &'a mut [T]) -> StackVec<'a, T> {
        let x = storage.len();
        StackVec {
            storage: storage,
            len: 0usize,
            capacity: x,
        }
    }

    /// Constructs a new `StackVec<T>` using `storage` as the backing store. The
    /// first `len` elements of `storage` are treated as if they were `push`ed
    /// onto `self.` The returned `StackVec` will be able to hold a total of
    /// `storage.len()` values.
    ///
    /// # Panics
    ///
    /// Panics if `len > storage.len()`.
    pub fn with_len(storage: &'a mut [T], len: usize) -> StackVec<'a, T> {
        let mut new_stack = Self::new(storage);
        new_stack.len += len;
        if new_stack.len > new_stack.capacity {
            panic! {"Len too big {} not fitting in capacity {} ",len, new_stack.capacity}
        }
        new_stack
    }

    /// Returns the number of elements this vector can hold.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Shortens the vector, keeping the first `len` elements. If `len` is
    /// greater than the vector's current length, this has no effect. Note that
    /// this method has no effect on the capacity of the vector.
    pub fn truncate(&mut self, len: usize) {
        if self.len > len {
            self.len = len;
        }
    }

    /// Extracts a slice containing the entire vector, consuming `self`.
    ///
    /// Note that the returned slice's length will be the length of this vector,
    /// _not_ the length of the original backing storage.
    pub fn into_slice(self) -> &'a mut [T] {
        &mut self.storage[..self.len]
    }

    /// Extracts a slice containing the entire vector.
    pub fn as_slice(&self) -> &[T] {
        &self.storage[..self.len]
    }

    /// Extracts a mutable slice of the entire vector.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.storage[..self.len]
    }

    /// Returns the number of elements in the vector, also referred to as its
    /// 'length'.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        0 == self.len
    }

    /// Returns true if the vector is at capacity.
    pub fn is_full(&self) -> bool {
        self.capacity == self.len
    }

    /// Appends `value` to the back of this vector if the vector is not full.
    ///
    /// # Error
    ///
    /// If this vector is full, an `Err` is returned. Otherwise, `Ok` is
    /// returned.
    pub fn push(&mut self, value: T) -> Result<(), ()> {
        if !self.is_full() {
            self.storage[self.len] = value;
            self.len += 1;
            Ok(())
        } else {
            Err(())
        }
    }
}

impl<'a, T: Clone + 'a> StackVec<'a, T> {
    /// If this vector is not empty, removes the last element from this vector
    /// by cloning it and returns it. Otherwise returns `None`.
    pub fn pop(&mut self) -> Option<T> {
        if !self.is_empty() {
            self.len = self.len - 1;
            Some(self.storage[self.len].clone())
        } else {
            None
        }
    }
}
impl<'a, T: Clone + 'a> Deref for StackVec<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.storage[..self.len]
    }
}

impl<'a, T: Clone + 'a> DerefMut for StackVec<'a, T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.storage[..self.len]
    }
}

impl<'a, T: Clone + 'a> Index<usize> for StackVec<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if self.len <= index {
            panic!("Index out of bounds");
        }
        &self.storage[index]
    }
}
impl<'a, T: Clone + 'a> IndexMut<usize> for StackVec<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.storage[index]
    }
}

pub struct StackVecIntoIterator<'a, T: 'a> {
    stackvec: StackVec<'a, T>,
    index: usize,
}

impl<'a, T: Clone + 'a> IntoIterator for StackVec<'a, T> {
    type Item = T;
    type IntoIter = StackVecIntoIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        StackVecIntoIterator {
            stackvec: self,
            index: 0,
        }
    }
}

impl<'a, T: Clone + 'a> Iterator for StackVecIntoIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.stackvec.pop().clone()
    }
}
// FIXME: Implement `Deref`, `DerefMut`, and `IntoIterator` for `StackVec`.
// FIXME: Implement IntoIterator` for `&StackVec`.
