/*
 * Reverse Polish Notation: rpn.rs
 * See `rpn.md` for the overview.
 */

use std::io;
use rand::Rng;

// Stacks will work with Items, which either either integers or booleans
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Item {
    Int(i32),
    Bool(bool),
}

// List of possible errors
#[derive(Debug)]
pub enum Error {
    Empty,         // Tried to pop empty stack
    Extra,         // Stack ended with extra elements
    Type,          // Type mismatch
    Syntax,        // Syntax error, didn't recognize op
    IO(io::Error), // Some kind of IO error
    Quit,          // User quitting
}

// Base operations supported by calculator, see rpn.md
#[derive(Debug)]
pub enum Op {
    Add,
    Eq,
    Neg,
    Swap,
    Rand,
    Cond,
    Quit,
}

// We'll define a result type for our calculator: either a valid value, or a calculator Error
pub type Result<T> = std::result::Result<T, Error>;

// Define a type for Stacks
#[derive(Debug)]
pub struct Stack{
    stack: Vec<Item>
}

// Implement the following functions on Stacks
impl Stack {
    // Make a new Stack
    pub fn new() -> Self {
        Stack {stack: Vec::new()}
    }

    // Check if a Stack is empty
    pub fn empty(&self) -> bool {
        self.stack.is_empty()
    }

    // Push an item onto a stack (should never error)
    pub fn push(&mut self, item: Item) -> Result<()> {
        self.stack.push(item);
        Ok(())
    }

    // Pop an item off the Stack; may result in Empty error
    pub fn pop(&mut self) -> Result<Item> {
        if self.empty() {
            Err(Error::Empty)
        } else {
            Ok(self.stack.pop().unwrap())
        }
    }

    /*
     * Main evaluation function: apply an operation to a Stack
     *
     * Complete each of the cases. 
     *
     * Hint: You'll probably want to use the "question-mark" syntax quite a bit; see `rpn.md`.
     */
    pub fn eval(&mut self, op: Op) -> Result<()> {
        //&mut self will refer to a stack object
        //let stack refer to the self object
        let stack = self;
        match op {
            Op::Add => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                match (a, b) {
                    (Item::Int(a), Item::Int(b)) => {
                        let _ = stack.push(Item::Int(a + b));
                        Ok(())
                    }
                    _ => Err(Error::Type),
                }
            }
            Op::Eq => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                match (a, b) {
                    (Item::Int(a), Item::Int(b)) => {
                        let _ = stack.push(Item::Bool(a == b));
                        Ok(())
                    }
                    (Item::Bool(a), Item::Bool(b)) => {
                        let _ = stack.push(Item::Bool(a == b));
                        Ok(())
                    }
                    _ => Err(Error::Type),
                }
            }
            Op::Neg => {
                // if we have a boolean on the stack, negate it
                // else, return a type error
                let a = stack.pop()?;
                if let Item::Bool(a) = a {
                    let _ = stack.push(Item::Bool(!a));
                } else {
                    return Err(Error::Type);
                }
                Ok(())
            }
            Op::Swap => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                let _ = stack.push(a);
                let _ = stack.push(b);
                Ok(())

            }
            Op::Rand => {
                // looking at the top element of the stack
                // if its a bool, return a type error
                // if its empty, return an empty error
                // else, gen a rand number between 0 and the top element and push it
                let a = stack.pop()?;
                match a {
                    Item::Int(a) => {
                        //push the random number onto the stack using the rand crate
                        let num = rand::thread_rng().gen_range(0,a);
                        let _ = stack.push(Item::Int(num));
                        Ok(())
                    }
                    _ => Err(Error::Type),
                }
            }
            Op::Cond => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                let c = stack.pop()?;
                match c {
                    Item::Bool(c) => {
                        if c {
                            let _ = stack.push(b);
                        } else {
                            let _ = stack.push(a);
                        }
                        Ok(())
                    }
                    _ => Err(Error::Type),
                }
            }
            Op::Quit => {
                Err(Error::Quit)
            }
        }
    }
}
