use std::{fmt::Debug, mem::MaybeUninit, slice::Iter};

// I have no clue if any of this code causes UB I just hope it doesn't

/// a wrapper around an array on the stack with a stack interface only allowing pushing and popping
///
/// # Panics
///
/// will panic if more is pushed to the stack than it can hold,
/// or if more is popped than pushed

#[derive(Clone)]
pub(crate) struct Stack<T, const SIZE: usize>
where
    T: Copy,
{
    array: [MaybeUninit<T>; SIZE],
    index: usize,
}
impl<T, const SIZE: usize> Drop for Stack<T, SIZE>
where
    T: Copy,
{
    // drop implementation due to the requirements of `MaybeUninit`
    fn drop(&mut self) {
        for i in &self.array[0..self.index] {
            unsafe {
                i.assume_init();
            }
        }
    }
}
impl<T, const SIZE: usize> Stack<T, SIZE>
where
    T: Copy,
{
    /// constructs a new `Stack` with size N
    pub fn new() -> Self {
        Self {
            array: [MaybeUninit::uninit(); SIZE],
            index: 0,
        }
    }
    /// adds an element to the top of stack
    pub fn push(&mut self, val: T) {
        if self.index >= self.array.len() {
            panic!("attempted to push more than the stack can hold");
        }
        self.array[self.index].write(val);
        self.index += 1;
    }
    /// removes an element from the top of the stack and returns it
    pub fn pop(&mut self) -> T {
        self.index -= 1;
        if self.index == usize::MAX {
            panic!("attempted to pop more than the stack contained");
        }
        let temp = unsafe { self.array[self.index].assume_init() };
        self.array[self.index] = MaybeUninit::uninit();
        temp
    }
    /// clears the stack, droping every element
    pub fn clear(&mut self) {
        for i in &mut self.array[0..self.index] {
            unsafe {
                i.assume_init();
                *i = MaybeUninit::uninit();
            }
        }
        self.index = 0;
    }
    /// returns an iter over the elements of the stack
    pub fn iter(&self) -> Iter<T> {
        unsafe { MaybeUninit::slice_assume_init_ref(&self.array[0..self.index]) }.iter()
    }
    /// returns a slice of the values contained
    pub fn slice(&self) -> &[T] {
        unsafe { MaybeUninit::slice_assume_init_ref(&self.array[0..self.index]) }
    }
    pub fn full_array(&self) -> Option<[T; SIZE]> {
        if self.index == SIZE {
            unsafe { Some(MaybeUninit::array_assume_init(self.array)) }
        } else {
            None
        }
    }
    /// returns the length of the stack
    pub fn len(&self) -> usize {
        self.index
    }
}

impl<T, const N: usize> Debug for Stack<T, N>
where
    T: Copy + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stack")
            .field("array", &self.slice())
            .field("index", &self.index)
            .finish()
    }
}

impl<T, const SIZE: usize> From<&[T]> for Stack<T, SIZE> where T: Copy {
    fn from(slice: &[T]) -> Self {
        if slice.len() > SIZE {
            panic!("slice bigger than stack");
        }
        let stack = Self {
            array: [MaybeUninit::uninit(); SIZE],
            index: 0,
        };
        for item in slice {
            stack.push(*item);
        }
        stack
    }
}
