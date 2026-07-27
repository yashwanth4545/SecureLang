#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    // Stack manipulation
    PushInt(i64),
    PushString(String),
    PushBool(bool),
    Pop,
    
    // Arithmetic
    Add, Sub, Mul, Div,
    
    // Comparison
    Eq, Neq, Lt, Gt, Lte, Gte,
    
    // Variables
    LoadGlobal(String),
    StoreGlobal(String),
    LoadLocal(usize),
    StoreLocal(usize),
    
    // Control Flow
    Jump(usize),
    JumpIfFalse(usize),
    CallNative(String, usize), // Function name, arg count
    Call(usize, usize),        // Address, arg count
    Return,
    
    // SecureLang specific
    AllocSecureString(String),
    PromptInput(String),
    Authenticate,
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<OpCode>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk { code: Vec::new() }
    }

    pub fn write(&mut self, op: OpCode) -> usize {
        self.code.push(op);
        self.code.len() - 1
    }
}
