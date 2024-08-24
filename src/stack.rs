//! Functionality for a stack container.

/// A basic stack container.
pub struct Stack<T>
where
    T: Copy + Default + Sized,
{
    /// The values in the stack.
    values: Vec<T>,
}

impl<T> Stack<T>
where
    T: Copy + Default + Sized,
{
    /// Initializes an empty stack.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { values: vec![] }
    }

    /// Constructs a stack from a value.
    #[inline]
    pub fn from(value: T) -> Self {
        Self {
            values: vec![value],
        }
    }

    /// Pushes a value onto the stack.
    #[inline]
    pub fn push(&mut self, value: T) {
        self.values.push(value);
    }

    /// Gets the top value from the stack.
    #[inline]
    pub fn top(&mut self) -> Option<T> {
        self.values.last().copied()
    }

    /// Pops a value from the stack.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.values.pop()
    }

    /// Checks if the stack is empty.
    #[inline]
    #[must_use]
    pub fn empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl<T> Default for Stack<T>
where
    T: Copy + Default + Sized,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
