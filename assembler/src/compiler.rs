use machine::internal::opcode::{Opcode, OpcodeVariant};
use std::{collections::HashMap};

use crate::parser::parse_program;

fn combine_hl(high: u32, low: u32) -> u32 {
    // mask to ensure only 16 bits are taken
    ((high & 0xFFFF) << 16) | (low & 0xFFFF)
}

#[derive(Debug)]
pub struct CompiledFrame {
    pub origin: u32,
    pub binary: Vec<u32>,
}

pub fn compile(code: String) -> CompiledFrame {
    let mut result: Vec<u32> = Vec::new();
    let mut origin: u32 = 0;
    let content = code.as_str();
    let mut label_usage = HashMap::<usize, &str>::new();
    let mut labels = HashMap::<&str, usize>::new();
    let (_, tokens) = parse_program(content).unwrap();
    for token in tokens {
        match token {
            crate::tokens::Token::Meta(meta_type) => {
                match meta_type {
                    crate::tokens::MetaType::Org(n) => {
                        origin = n;
                    },
                    crate::tokens::MetaType::Include(_) => {},
                }
            },
            crate::tokens::Token::Command(cmd) => {
                match cmd {
                    crate::tokens::Cmd::PushConst(const_value) => {
                        match const_value {
                            crate::tokens::ConstValue::Number(val) => {
                                result.push(combine_hl(Opcode::Push as u32, OpcodeVariant::PushConst as u32));
                                result.push(val);
                            },
                            crate::tokens::ConstValue::Label(val) => {
                                result.push(combine_hl(Opcode::Push as u32, OpcodeVariant::PushConst as u32));
                                label_usage.insert(result.len(), val);
                                result.push(0);
                            },
                        }
                    },
                    crate::tokens::Cmd::Mul => {
                        result.push(combine_hl(Opcode::Mul as u32, OpcodeVariant::Default as u32));
                    },
                    crate::tokens::Cmd::Div => {
                        result.push(combine_hl(Opcode::Div as u32, OpcodeVariant::Default as u32));
                    },
                    crate::tokens::Cmd::Drop => {
                        result.push(combine_hl(Opcode::Drop as u32, OpcodeVariant::Default as u32));
                    },
                    crate::tokens::Cmd::PushReg(val) => {
                        result.push(combine_hl(Opcode::Push as u32, OpcodeVariant::PushReg as u32));
                        result.push(val);
                    },
                    crate::tokens::Cmd::PushAddr(val) => {
                        result.push(combine_hl(Opcode::Push as u32, OpcodeVariant::PushAddr as u32));
                        result.push(val);
                    },
                    crate::tokens::Cmd::PopReg(val) => {
                        result.push(combine_hl(Opcode::Pop as u32, OpcodeVariant::PopReg as u32));
                        result.push(val);
                    },
                    crate::tokens::Cmd::PopAddr(val) => {
                        result.push(combine_hl(Opcode::Pop as u32, OpcodeVariant::PopAddr as u32));
                        result.push(val);
                    },
                    crate::tokens::Cmd::Add => {
                        result.push(combine_hl(Opcode::Add as u32, OpcodeVariant::Default as u32));
                    },
                    crate::tokens::Cmd::Sub => {
                        result.push(combine_hl(Opcode::Sub as u32, OpcodeVariant::Default as u32));
                    },
                    crate::tokens::Cmd::Swap => {
                        result.push(combine_hl(Opcode::Swap as u32, OpcodeVariant::Default as u32));
                    },
                    crate::tokens::Cmd::MoveConst(val, const_value) => {
                        match const_value {
                            crate::tokens::ConstValue::Number(n) => {
                                result.push(combine_hl(Opcode::Move as u32, OpcodeVariant::MoveConst as u32));
                                result.push(val);
                                result.push(n);
                            },
                            crate::tokens::ConstValue::Label(label) => {
                                result.push(combine_hl(Opcode::Move as u32, OpcodeVariant::MoveConst as u32));
                                result.push(val);
                                label_usage.insert(result.len(), label);
                                result.push(0);
                            },
                        }
                    },
                    crate::tokens::Cmd::MoveReg(val, reg) => {
                        result.push(combine_hl(Opcode::Move as u32, OpcodeVariant::MoveReg as u32));
                        result.push(val);
                        result.push(reg);
                    },
                    crate::tokens::Cmd::MoveAddr(val, addr) => {
                        result.push(combine_hl(Opcode::Move as u32, OpcodeVariant::MoveAddr as u32));
                        result.push(val);
                        result.push(addr);
                    },
                    crate::tokens::Cmd::StoreConst(val, const_value) => {
                        match const_value {
                            crate::tokens::ConstValue::Number(n) => {
                                result.push(combine_hl(Opcode::Store as u32, OpcodeVariant::StoreConst as u32));
                                result.push(val);
                                result.push(n);
                            },
                            crate::tokens::ConstValue::Label(label) => {
                                result.push(combine_hl(Opcode::Store as u32, OpcodeVariant::StoreConst as u32));
                                result.push(val);
                                label_usage.insert(result.len(), label);
                                result.push(0);
                            },
                        }
                    },
                    crate::tokens::Cmd::StoreReg(val, reg) => {
                        result.push(combine_hl(Opcode::Store as u32, OpcodeVariant::StoreReg as u32));
                        result.push(val);
                        result.push(reg);
                    },
                    crate::tokens::Cmd::Jmp(label) => {
                        result.push(combine_hl(Opcode::Jump as u32, OpcodeVariant::Default as u32));
                        label_usage.insert(result.len(), label);
                        result.push(0);
                    },
                    crate::tokens::Cmd::Jnz(label) => {
                        result.push(combine_hl(Opcode::Jump as u32, OpcodeVariant::JumpNotZero as u32));
                        label_usage.insert(result.len(), label);
                        result.push(0);
                    },
                    crate::tokens::Cmd::Jz(label) => {
                        result.push(combine_hl(Opcode::Jump as u32, OpcodeVariant::JumpZero as u32));
                        label_usage.insert(result.len(), label);
                        result.push(0);
                    },
                    crate::tokens::Cmd::Jg(label) => {
                        result.push(combine_hl(Opcode::Jump as u32, OpcodeVariant::JumpGreater as u32));
                        label_usage.insert(result.len(), label);
                        result.push(0);
                    },
                    crate::tokens::Cmd::Jge(label) => {
                        result.push(combine_hl(Opcode::Jump as u32, OpcodeVariant::JumpGreaterEqual as u32));
                        label_usage.insert(result.len(), label);
                        result.push(0);
                    },
                    crate::tokens::Cmd::Jl(label) => {
                        result.push(combine_hl(Opcode::Jump as u32, OpcodeVariant::JumpLesser as u32));
                        label_usage.insert(result.len(), label);
                        result.push(0);
                    },
                    crate::tokens::Cmd::Jle(label) => {
                        result.push(combine_hl(Opcode::Jump as u32, OpcodeVariant::JumpLesserEqual as u32));
                        label_usage.insert(result.len(), label);
                        result.push(0);
                    },
                    crate::tokens::Cmd::And => {
                        result.push(combine_hl(Opcode::And as u32, OpcodeVariant::Default as u32));
                    },
                    crate::tokens::Cmd::Or => {
                        result.push(combine_hl(Opcode::Or as u32, OpcodeVariant::Default as u32));
                    },
                    crate::tokens::Cmd::Xor => {
                        result.push(combine_hl(Opcode::Xor as u32, OpcodeVariant::Default as u32));
                    },
                    crate::tokens::Cmd::Not => {
                        result.push(combine_hl(Opcode::Not as u32, OpcodeVariant::Default as u32));
                    },
                    crate::tokens::Cmd::ShrConst(val) => {
                        result.push(combine_hl(Opcode::SHR as u32, OpcodeVariant::SHRConst as u32));
                        result.push(val);
                    },
                    crate::tokens::Cmd::ShrReg(val) => {
                        result.push(combine_hl(Opcode::SHR as u32, OpcodeVariant::SHRReg as u32));
                        result.push(val);
                    },
                    crate::tokens::Cmd::ShlConst(val) => {
                        result.push(combine_hl(Opcode::SHL as u32, OpcodeVariant::SHLConst as u32));
                        result.push(val);
                    },
                    crate::tokens::Cmd::ShlReg(reg) => {
                        result.push(combine_hl(Opcode::SHL as u32, OpcodeVariant::SHLReg as u32));
                        result.push(reg);
                    },
                    crate::tokens::Cmd::CallConst(const_value) => {
                        match const_value {
                            crate::tokens::ConstValue::Number(val) => {
                                result.push(combine_hl(Opcode::Call as u32, OpcodeVariant::CallConst as u32));
                                result.push(val);
                            },
                            crate::tokens::ConstValue::Label(label) => {
                                result.push(combine_hl(Opcode::Call as u32, OpcodeVariant::CallConst as u32));
                                label_usage.insert(result.len(), label);
                                result.push(0);
                            },
                        }
                    },
                    crate::tokens::Cmd::CallReg(reg) => {
                        result.push(combine_hl(Opcode::Call as u32, OpcodeVariant::CallReg as u32));
                        result.push(reg);
                    },
                    crate::tokens::Cmd::CallAddr(addr) => {
                        result.push(combine_hl(Opcode::Call as u32, OpcodeVariant::CallAddr as u32));
                        result.push(addr);
                    },
                    crate::tokens::Cmd::Ret => {
                        result.push(combine_hl(Opcode::Ret as u32, OpcodeVariant::Default as u32));
                    },
                    crate::tokens::Cmd::Dup => {
                        result.push(combine_hl(Opcode::Dup as u32, OpcodeVariant::Default as u32));
                    },
                    crate::tokens::Cmd::DupConst(val) => {
                        result.push(combine_hl(Opcode::Dup as u32, OpcodeVariant::DupConst as u32));
                        result.push(val);
                    },
                    crate::tokens::Cmd::DupReg(reg) => {
                        result.push(combine_hl(Opcode::Dup as u32, OpcodeVariant::DupReg as u32));
                        result.push(reg);
                    },
                    crate::tokens::Cmd::Int(module, function) => {
                        result.push(combine_hl(Opcode::Int as u32, OpcodeVariant::Default as u32));
                        result.push(module);
                        result.push(function);
                    },
                    crate::tokens::Cmd::Term => {
                        result.push(combine_hl(Opcode::Terminate as u32, OpcodeVariant::Default as u32));
                    },
                }
            },
            crate::tokens::Token::Label(label) => {
                labels.insert(label, result.len());
            },
        }
    }
    for (k, v) in label_usage {
        result[k] = labels[v] as u32 + origin;
    }
    CompiledFrame{
        binary: result,
        origin: origin,
    }
}