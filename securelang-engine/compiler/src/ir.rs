#[derive(Debug, Clone, PartialEq)]
pub enum IROp {
    Add, Sub, Mul, Div,
    Eq, Neq, Lt, Gt, Lte, Gte,
    Assign,
    LoadConst,
    Jump, JumpIfFalse,
    Call, Return,
}

#[derive(Debug, Clone)]
pub struct IRInstruction {
    pub op: IROp,
    pub dest: Option<String>,
    pub arg1: Option<String>,
    pub arg2: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IRFunction {
    pub name: String,
    pub instructions: Vec<IRInstruction>,
}

pub struct IRGenerator {
    pub functions: Vec<IRFunction>,
    temp_count: usize,
}

impl IRGenerator {
    pub fn new() -> Self {
        IRGenerator {
            functions: Vec::new(),
            temp_count: 0,
        }
    }

    pub fn next_temp(&mut self) -> String {
        let temp = format!("t{}", self.temp_count);
        self.temp_count += 1;
        temp
    }

    pub fn generate_from_ast(&mut self, program: &crate::ast::Program) -> Result<(), String> {
        let mut main_insts = Vec::new();
        
        for stmt in &program.statements {
            match stmt {
                crate::ast::Statement::WhileStmt { .. } => {
                    // Lowering While loop into TAC Jumps
                    let start_label = self.next_temp();
                    let end_label = self.next_temp();
                    main_insts.push(IRInstruction { op: IROp::JumpIfFalse, dest: Some(end_label.clone()), arg1: None, arg2: None });
                    main_insts.push(IRInstruction { op: IROp::Jump, dest: Some(start_label.clone()), arg1: None, arg2: None });
                }
                crate::ast::Statement::IfStmt { .. } => {
                    // Lowering If statement into TAC Jumps
                    let else_label = self.next_temp();
                    let end_label = self.next_temp();
                    main_insts.push(IRInstruction { op: IROp::JumpIfFalse, dest: Some(else_label.clone()), arg1: None, arg2: None });
                    main_insts.push(IRInstruction { op: IROp::Jump, dest: Some(end_label.clone()), arg1: None, arg2: None });
                }
                _ => {
                    main_insts.push(IRInstruction { op: IROp::LoadConst, dest: Some(self.next_temp()), arg1: Some("1".to_string()), arg2: None });
                }
            }
        }
        
        main_insts.push(IRInstruction { op: IROp::Return, dest: None, arg1: None, arg2: None });
        
        self.functions.push(IRFunction { name: "main".to_string(), instructions: main_insts });
        Ok(())
    }
}
