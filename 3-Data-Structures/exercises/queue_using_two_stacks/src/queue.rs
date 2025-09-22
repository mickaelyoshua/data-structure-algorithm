#[derive(Default)]
struct Stack {
    elems: Vec<i32>,
}

impl Stack {
    fn new() -> Self {
        Stack { elems: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        self.elems.push(val);
    }

    fn pop(&mut self) -> Option<i32> {
        self.elems.pop()
    }

    fn peek(&self) -> Option<&i32> {
        self.elems.last()
    }
}

#[derive(Default)]
pub struct Queue {
    in_stack: Stack,
    out_stack: Stack,
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            in_stack: Stack::new(),
            out_stack: Stack::new(),
        }
    }
    
    pub fn enqueue(&mut self, val: i32) {
        self.in_stack.push(val);
    }

    pub fn fill_out_stack_if_empty(&mut self) {
        if self.out_stack.peek().is_none() {
            while let Some(out_value) = self.in_stack.pop() {
                self.out_stack.push(out_value);
            }
        }
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        self.fill_out_stack_if_empty();
        self.out_stack.pop()
    }

    pub fn peek(&mut self) -> Option<&i32> {
        self.fill_out_stack_if_empty();
        self.out_stack.peek()
    }

    pub fn print(&mut self) {
        self.fill_out_stack_if_empty();
        println!("{}", self.out_stack.peek().unwrap());
    }
}
