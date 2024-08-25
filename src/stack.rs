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
    ///
    /// ### Returns
    /// * An empty stack.
    ///
    /// ### Examples
    /// ```
    /// use maze_generation::stack::Stack;
    ///
    /// let stack = Stack::<i32>::new();
    /// assert!(stack.empty());
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { values: vec![] }
    }

    /// Constructs a stack from a value.
    ///
    /// ### Parameters
    /// * `value`: Some value to use in the stack.
    ///
    /// ### Returns
    /// * An empty stack.
    ///
    /// ### Examples
    /// ```
    /// use maze_generation::stack::Stack;
    ///
    /// let stack = Stack::<i32>::from(1);
    ///
    /// assert_eq!(stack.top().unwrap_or_default(), 1);
    /// ```
    #[inline]
    pub fn from(value: T) -> Self {
        Self {
            values: vec![value],
        }
    }

    /// Pushes a value onto the stack.
    ///
    /// ### Parameters
    /// * `value`: Some value to push on the stack.
    ///
    /// ### Examples
    /// ```
    /// use maze_generation::stack::Stack;
    ///
    /// let mut stack = Stack::<i32>::new();
    /// stack.push(1);
    /// assert_eq!(stack.top().unwrap_or_default(), 1);
    #[inline]
    pub fn push(&mut self, value: T) {
        self.values.push(value);
    }

    /// Gets the top value from the stack.
    ///
    /// ### Returns
    /// * The top value from the stack, or None if the stack is empty.
    ///
    /// ### Examples
    /// ```
    /// use maze_generation::stack::Stack;
    ///
    /// let stack = Stack::<i32>::from(1);
    /// assert_eq!(stack.top().unwrap_or_default(), 1);
    /// ```
    #[inline]
    #[must_use]
    pub fn top(&self) -> Option<T> {
        self.values.last().copied()
    }

    /// Pops a value from the stack.
    ///
    /// ### Returns
    /// * The popped value from the stack, or None if the stack is empty.
    ///
    /// ### Examples
    /// ```
    /// use maze_generation::stack::Stack;
    ///
    /// let mut stack = Stack::<i32>::from(1);
    /// let popped_value = stack.pop();
    /// assert_eq!(popped_value.unwrap_or_default(), 1);
    /// assert!(stack.empty());
    /// ```
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.values.pop()
    }

    /// Checks if the stack is empty.
    ///
    /// ### Returns
    /// * `true` if the stack is empty, otherwise `false`.
    ///
    /// ### Examples
    /// ```
    /// use maze_generation::stack::Stack;
    ///
    /// let mut stack = Stack::<i32>::from(1);
    /// assert!(!stack.empty());
    /// 
    /// let _ = stack.pop();
    /// assert!(stack.empty());
    /// ```
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
