use crate::ir::{IRFunction, IRInstruction, IROp};
use crate::bytecode::{Chunk, OpCode};

pub struct CodeGenerator {
    chunk: Chunk,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            chunk: Chunk::new(),
        }
    }

    pub fn generate_from_ir(&mut self, function: &IRFunction) -> Chunk {
        for instruction in &function.instructions {
            self.generate_instruction(instruction);
        }
        self.chunk.clone()
    }

    fn generate_instruction(&mut self, inst: &IRInstruction) {
        match inst.op {
            IROp::LoadConst => {
                if let Some(arg1) = &inst.arg1 {
                    if let Ok(val) = arg1.parse::<i64>() {
                        self.chunk.write(OpCode::PushInt(val));
                    } else if arg1 == "true" {
                        self.chunk.write(OpCode::PushBool(true));
                    } else if arg1 == "false" {
                        self.chunk.write(OpCode::PushBool(false));
                    } else {
                        self.chunk.write(OpCode::PushString(arg1.clone()));
                    }
                    if let Some(dest) = &inst.dest {
                        self.chunk.write(OpCode::StoreGlobal(dest.clone()));
                    }
                }
            }
            IROp::Assign => {
                if let Some(arg1) = &inst.arg1 {
                    self.chunk.write(OpCode::LoadGlobal(arg1.clone()));
                    if let Some(dest) = &inst.dest {
                        self.chunk.write(OpCode::StoreGlobal(dest.clone()));
                    }
                }
            }
            IROp::Add => {
                if let (Some(a), Some(b)) = (&inst.arg1, &inst.arg2) {
                    self.chunk.write(OpCode::LoadGlobal(a.clone()));
                    self.chunk.write(OpCode::LoadGlobal(b.clone()));
                    self.chunk.write(OpCode::Add);
                    if let Some(dest) = &inst.dest {
                        self.chunk.write(OpCode::StoreGlobal(dest.clone()));
                    }
                }
            }
            IROp::Sub => {
                if let (Some(a), Some(b)) = (&inst.arg1, &inst.arg2) {
                    self.chunk.write(OpCode::LoadGlobal(a.clone()));
                    self.chunk.write(OpCode::LoadGlobal(b.clone()));
                    self.chunk.write(OpCode::Sub);
                    if let Some(dest) = &inst.dest {
                        self.chunk.write(OpCode::StoreGlobal(dest.clone()));
                    }
                }
            }
            IROp::Mul => {
                if let (Some(a), Some(b)) = (&inst.arg1, &inst.arg2) {
                    self.chunk.write(OpCode::LoadGlobal(a.clone()));
                    self.chunk.write(OpCode::LoadGlobal(b.clone()));
                    self.chunk.write(OpCode::Mul);
                    if let Some(dest) = &inst.dest {
                        self.chunk.write(OpCode::StoreGlobal(dest.clone()));
                    }
                }
            }
            IROp::Div => {
                if let (Some(a), Some(b)) = (&inst.arg1, &inst.arg2) {
                    self.chunk.write(OpCode::LoadGlobal(a.clone()));
                    self.chunk.write(OpCode::LoadGlobal(b.clone()));
                    self.chunk.write(OpCode::Div);
                    if let Some(dest) = &inst.dest {
                        self.chunk.write(OpCode::StoreGlobal(dest.clone()));
                    }
                }
            }
            IROp::Eq => {
                if let (Some(a), Some(b)) = (&inst.arg1, &inst.arg2) {
                    self.chunk.write(OpCode::LoadGlobal(a.clone()));
                    self.chunk.write(OpCode::LoadGlobal(b.clone()));
                    self.chunk.write(OpCode::Eq);
                    if let Some(dest) = &inst.dest {
                        self.chunk.write(OpCode::StoreGlobal(dest.clone()));
                    }
                }
            }
            IROp::Neq => {
                if let (Some(a), Some(b)) = (&inst.arg1, &inst.arg2) {
                    self.chunk.write(OpCode::LoadGlobal(a.clone()));
                    self.chunk.write(OpCode::LoadGlobal(b.clone()));
                    self.chunk.write(OpCode::Neq);
                    if let Some(dest) = &inst.dest {
                        self.chunk.write(OpCode::StoreGlobal(dest.clone()));
                    }
                }
            }
            IROp::Lt => {
                if let (Some(a), Some(b)) = (&inst.arg1, &inst.arg2) {
                    self.chunk.write(OpCode::LoadGlobal(a.clone()));
                    self.chunk.write(OpCode::LoadGlobal(b.clone()));
                    self.chunk.write(OpCode::Lt);
                    if let Some(dest) = &inst.dest {
                        self.chunk.write(OpCode::StoreGlobal(dest.clone()));
                    }
                }
            }
            IROp::Gt => {
                if let (Some(a), Some(b)) = (&inst.arg1, &inst.arg2) {
                    self.chunk.write(OpCode::LoadGlobal(a.clone()));
                    self.chunk.write(OpCode::LoadGlobal(b.clone()));
                    self.chunk.write(OpCode::Gt);
                    if let Some(dest) = &inst.dest {
                        self.chunk.write(OpCode::StoreGlobal(dest.clone()));
                    }
                }
            }
            IROp::Lte => {
                if let (Some(a), Some(b)) = (&inst.arg1, &inst.arg2) {
                    self.chunk.write(OpCode::LoadGlobal(a.clone()));
                    self.chunk.write(OpCode::LoadGlobal(b.clone()));
                    self.chunk.write(OpCode::Lte);
                    if let Some(dest) = &inst.dest {
                        self.chunk.write(OpCode::StoreGlobal(dest.clone()));
                    }
                }
            }
            IROp::Gte => {
                if let (Some(a), Some(b)) = (&inst.arg1, &inst.arg2) {
                    self.chunk.write(OpCode::LoadGlobal(a.clone()));
                    self.chunk.write(OpCode::LoadGlobal(b.clone()));
                    self.chunk.write(OpCode::Gte);
                    if let Some(dest) = &inst.dest {
                        self.chunk.write(OpCode::StoreGlobal(dest.clone()));
                    }
                }
            }
            IROp::Jump => {
                // Placeholder Jump, label resolution requires two passes in a real compiler.
                self.chunk.write(OpCode::Jump(0));
            }
            IROp::JumpIfFalse => {
                self.chunk.write(OpCode::JumpIfFalse(0));
            }
            IROp::Call => {
                if let Some(name) = &inst.arg1 {
                    self.chunk.write(OpCode::CallNative(name.clone(), 0));
                }
            }
            IROp::Return => {
                if let Some(arg) = &inst.arg1 {
                    self.chunk.write(OpCode::LoadGlobal(arg.clone()));
                }
                self.chunk.write(OpCode::Return);
            }
        }
    }
}
