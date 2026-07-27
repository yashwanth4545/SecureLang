use compiler::bytecode::{Chunk, OpCode};
use crate::memory::{Value, GCHeap};
use std::collections::HashMap;
use std::io::{self, Write};

pub struct CallFrame {
    pub ip: usize,
    pub stack_offset: usize,
    pub chunk: Chunk,
}

pub struct ExecutionEngine {
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
    heap: GCHeap,
    call_stack: Vec<CallFrame>,
}

impl ExecutionEngine {
    pub fn new() -> Self {
        ExecutionEngine {
            stack: Vec::new(),
            globals: HashMap::new(),
            heap: GCHeap::new(),
            call_stack: Vec::new(),
        }
    }

    pub fn execute(&mut self, chunk: Chunk) -> Result<(), String> {
        let frame = CallFrame {
            ip: 0,
            stack_offset: 0,
            chunk,
        };
        self.call_stack.push(frame);

        macro_rules! pop {
            () => {
                self.stack.pop().ok_or("Stack underflow")?
            };
        }

        while !self.call_stack.is_empty() {
            let (ip, code) = {
                let frame = self.call_stack.last().unwrap();
                (frame.ip, frame.chunk.code.clone())
            };
            if ip >= code.len() { break; }
            let instruction = &code[ip];
            match instruction {
                OpCode::PushInt(v) => self.stack.push(Value::Int(*v)),
                OpCode::PushString(v) => self.stack.push(Value::String(v.clone())),
                OpCode::PushBool(v) => self.stack.push(Value::Bool(*v)),
                OpCode::Pop => { pop!(); }
                OpCode::Add => {
                    let b = pop!();
                    let a = pop!();
                    if let (Value::Int(a_v), Value::Int(b_v)) = (a, b) {
                        self.stack.push(Value::Int(a_v + b_v));
                    } else {
                        return Err("Type mismatch on Add".into());
                    }
                }
                OpCode::Sub => {
                    let b = pop!();
                    let a = pop!();
                    if let (Value::Int(a_v), Value::Int(b_v)) = (a, b) {
                        self.stack.push(Value::Int(a_v - b_v));
                    }
                }
                OpCode::Mul => {
                    let b = pop!();
                    let a = pop!();
                    if let (Value::Int(a_v), Value::Int(b_v)) = (a, b) {
                        self.stack.push(Value::Int(a_v * b_v));
                    }
                }
                OpCode::Div => {
                    let b = pop!();
                    let a = pop!();
                    if let (Value::Int(a_v), Value::Int(b_v)) = (a, b) {
                        if b_v == 0 { return Err("Division by zero".into()); }
                        self.stack.push(Value::Int(a_v / b_v));
                    }
                }
                OpCode::Eq => {
                    let b = pop!();
                    let a = pop!();
                    self.stack.push(Value::Bool(a == b));
                }
                OpCode::Neq => {
                    let b = pop!();
                    let a = pop!();
                    self.stack.push(Value::Bool(a != b));
                }
                OpCode::Lt => {
                    let b = pop!();
                    let a = pop!();
                    if let (Value::Int(a_v), Value::Int(b_v)) = (a, b) {
                        self.stack.push(Value::Bool(a_v < b_v));
                    }
                }
                OpCode::Gt => {
                    let b = pop!();
                    let a = pop!();
                    if let (Value::Int(a_v), Value::Int(b_v)) = (a, b) {
                        self.stack.push(Value::Bool(a_v > b_v));
                    }
                }
                OpCode::Lte => {
                    let b = pop!();
                    let a = pop!();
                    if let (Value::Int(a_v), Value::Int(b_v)) = (a, b) {
                        self.stack.push(Value::Bool(a_v <= b_v));
                    }
                }
                OpCode::Gte => {
                    let b = pop!();
                    let a = pop!();
                    if let (Value::Int(a_v), Value::Int(b_v)) = (a, b) {
                        self.stack.push(Value::Bool(a_v >= b_v));
                    }
                }
                OpCode::LoadGlobal(name) => {
                    let val = self.globals.get(name).ok_or_else(|| format!("Undefined global: {}", name))?.clone();
                    self.stack.push(val);
                }
                OpCode::StoreGlobal(name) => {
                    let val = pop!();
                    self.globals.insert(name.clone(), val);
                }
                OpCode::LoadLocal(offset) => {
                    let frame = self.call_stack.last().unwrap();
                    let val = self.stack[frame.stack_offset + *offset].clone();
                    self.stack.push(val);
                }
                OpCode::StoreLocal(offset) => {
                    let val = pop!();
                    let frame = self.call_stack.last().unwrap();
                    let idx = frame.stack_offset + *offset;
                    if idx >= self.stack.len() {
                        self.stack.push(val);
                    } else {
                        self.stack[idx] = val;
                    }
                }
                OpCode::Jump(offset) => {
                    self.call_stack.last_mut().unwrap().ip = *offset;
                    continue;
                }
                OpCode::JumpIfFalse(offset) => {
                    let val = pop!();
                    if let Value::Bool(b) = val {
                        if !b {
                            self.call_stack.last_mut().unwrap().ip = *offset;
                            continue;
                        }
                    }
                }
                OpCode::Call(address, arg_count) => {
                    let frame = CallFrame {
                        ip: *address,
                        stack_offset: self.stack.len() - *arg_count,
                        chunk: self.call_stack.last().unwrap().chunk.clone(),
                    };
                    self.call_stack.push(frame);
                    continue;
                }
                OpCode::CallNative(name, _arg_count) => {
                    if name == "print" {
                        let val = pop!();
                        println!("{:?}", val);
                        self.stack.push(Value::Int(0));
                    }
                }
                OpCode::PromptInput(name) => {
                    print!("Input [{}]: ", name);
                    io::stdout().flush().map_err(|e| e.to_string())?;
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
                    self.globals.insert(name.clone(), Value::String(input.trim().to_string()));
                }
                OpCode::AllocSecureString(name) => {
                    print!("Secure Input [{}]: ", name);
                    io::stdout().flush().map_err(|e| e.to_string())?;
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
                    self.globals.insert(name.clone(), Value::SecureString(input.trim().to_string()));
                }
                OpCode::Authenticate => {
                    println!("[Runtime]: Authenticating against SecureLang Security Module...");
                    let user = self.globals.get("username");
                    let pass = self.globals.get("password");
                    match (user, pass) {
                        (Some(Value::String(u)), Some(Value::SecureString(p))) => {
                            if u == "admin" && p == "secure123" {
                                println!("[Runtime]: SUCCESS - Authenticated User.");
                            } else {
                                println!("[Runtime]: FAILURE - Invalid credentials.");
                            }
                        }
                        _ => println!("[Runtime]: Missing username or password variables."),
                    }
                }
                OpCode::Return => {
                    let return_val = pop!();
                    let popped_frame = self.call_stack.pop().unwrap();
                    self.stack.truncate(popped_frame.stack_offset); // clear locals
                    self.stack.push(return_val);
                    if self.call_stack.is_empty() {
                        return Ok(()); // Main function returned
                    }
                }
                _ => return Err("Unimplemented instruction".into()),
            }
            self.call_stack.last_mut().unwrap().ip += 1;
        }
        Ok(())
    }

    pub fn unwind_exception(&mut self, exception_msg: &str) -> Result<(), String> {
        // Advanced VM Exception Unwinding Mock
        println!("[VM Warning]: Exception thrown: {}. Unwinding stack...", exception_msg);
        // Normally this would pop stack frames until it finds a catch block
        Ok(())
    }
}
