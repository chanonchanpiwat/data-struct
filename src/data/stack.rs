// LIFO
#[allow(warnings)]
use std::{vec::Vec};

struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
   pub fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, element: T) {
        self.data.push(element);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let top = self.data.pop();
        self.size -= 1;
        top
    }

    pub fn top(&self) -> Option<&T> {
      self.data.last()
  }
}

#[cfg(test)]
mod test {
    use super::Stack;

  #[test]
  fn test() {
    let mut stack: Stack<i32> = Stack::new();
    
    assert!(stack.top() == None, "Initial");

    stack.push(1);
    stack.push(2);

    assert!(stack.size == 2, "Stack size should be 2");

    let top = stack.pop();

    assert!(top == Some(2),"Top element");
    assert!(stack.size == 1, "Stack size should be 1");


  }
}
