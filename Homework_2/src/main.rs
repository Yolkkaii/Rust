#[derive(Debug)]
enum Node {
    Cons(i32, Box<Node>),
    Nil,
}

#[derive(Debug)]
struct BoxedStack {
    top: Box<Node>
}

impl BoxedStack {
    pub fn new() -> BoxedStack {
        BoxedStack { top: Box::new(Node::Nil) }
    }

    pub fn push(&mut self, value: i32) {
        let old_top = std::mem::replace(&mut self.top, Box::new(Node::Nil));
        let new_top = Box::new(Node::Cons(value, old_top));
        self.top = new_top;
    }

    pub fn pop(&mut self) -> Option<i32> {
        let old_top = std::mem::replace(&mut self.top, Box::new(Node::Nil));
        match *old_top {
            Node::Cons(value, next_node) => {
                self.top = next_node;
                Some(value)
            }
            Node::Nil => None,
        }
    }

    pub fn peek(&self) -> Option<&i32> {
        match *self.top {
            Node::Cons(ref value, _) => Some(value),
            Node::Nil => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        match *self.top {
            Node::Nil => true,
            _ => false,
        }
    }

    pub fn print_stack(&self) {
        println!("Stack contents:");
        Self::print(&self.top);
    }

    fn print(node: &Box<Node>) {
        match **node {
            Node::Cons(value, ref next_node) => {
                print!("{} -> ", value);
                Self::print(next_node);
            }
            Node::Nil => {
                println!("Nil");
            }
        }
    }
}

fn main() {
    let mut stack = BoxedStack::new();
    let mut val = 10;
    for i in 0..3 {
        println!("Pushing {} onto the stack.", val);
        stack.push(val);
        val += 10;
    }
    stack.print_stack();

    println!("\nTop of the stack: {}", stack.peek().unwrap());

    println!("\nPopped {} from the stack.", stack.pop().unwrap());
    stack.print_stack();

    println!("\nPopped {} from the stack.", stack.pop().unwrap());
    stack.print_stack();

    println!("\nPopped {} from the stack.", stack.pop().unwrap());
    stack.print_stack();

    println!("\nIs the stack empty? {}", stack.is_empty());
}
