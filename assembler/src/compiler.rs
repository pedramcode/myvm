use machine::internal::opcode::{Opcode, OpcodeVariant};
use std::{collections::HashMap};

use crate::{parser::parse_program, tokens::DataType};

fn combine_hl(high: u32, low: u32) -> u32 {
    // mask to ensure only 16 bits are taken
    ((high & 0xFFFF) << 16) | (low & 0xFFFF)
}

#[derive(Debug)]
pub struct CompiledFrame {
    pub origin: u32,
    pub binary: Vec<u32>,
}

fn pack_u16_to_u32(v: Vec<u16>) -> Vec<u32> {
    let mut out = Vec::with_capacity((v.len() + 1) / 2);

    let mut iter = v.into_iter();
    while let Some(high) = iter.next() {
        if let Some(low) = iter.next() {
            // pack two u16 into u32
            let packed = ((high as u32) << 16) | (low as u32);
            out.push(packed);
        } else {
            // odd one out → shift into high 16 bits, low filled with zeros
            let packed = (high as u32) << 16;
            out.push(packed);
        }
    }

    out
}

fn pack_u8_to_u32(v: Vec<u8>) -> Vec<u32> {
    let mut out = Vec::with_capacity((v.len() + 3) / 4);

    let mut iter = v.into_iter();
    while let Some(b1) = iter.next() {
        if let Some(b2) = iter.next() {
            if let Some(b3) = iter.next() {
                if let Some(b4) = iter.next() {
                    // full 4 bytes
                    let packed = ((b1 as u32) << 24)
                               | ((b2 as u32) << 16)
                               | ((b3 as u32) << 8)
                               |  (b4 as u32);
                    out.push(packed);
                    continue;
                } else {
                    // 3 bytes
                    let packed = ((b1 as u32) << 24)
                               | ((b2 as u32) << 16)
                               | ((b3 as u32) << 8);
                    out.push(packed);
                    break;
                }
            } else {
                // 2 bytes
                let packed = ((b1 as u32) << 24)
                           | ((b2 as u32) << 16);
                out.push(packed);
                break;
            }
        } else {
            // only 1 byte
            let packed = (b1 as u32) << 24;
            out.push(packed);
            break;
        }
    }

    out
}


pub fn check_section(target: &str, current: &Option<&str>) {
    if let Some(s) = current {
        if target.to_lowercase() != (*s).to_lowercase() {
            panic!("invalid code for section '{}'", *s);
        }
    } else {
        panic!("invalid code for section");
    }
}

#[derive(Debug)]
struct DataLookup {
    pub address: u32,
}

pub fn compile(code: String) -> CompiledFrame {
    let mut result: Vec<u32> = Vec::new();
    let mut origin: u32 = 0;
    let content = code.as_str();
    let mut label_usage = HashMap::<usize, &str>::new();
    let mut labels = HashMap::<&str, usize>::new();
    let (_, tokens) = parse_program(content).unwrap();
    let mut current_section:Option<&str> = None;

    let mut data_list: Vec<(&str, DataType, Vec<u32>)> = Vec::new();
    let mut data_lookup: HashMap<&str, DataLookup> = HashMap::new();
    let mut data_usage: HashMap<usize, (&str, u32)> = HashMap::new();

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
            crate::tokens::Token::Section(sec) => {
                if sec.to_lowercase() == "text" || sec.to_lowercase() == "data" {
                    current_section = Some(sec);
                } else {
                    panic!("invalid section '{}'", sec);
                }
            },
            crate::tokens::Token::DataDef(id, typ, values) => {
                check_section("data", &current_section);
                let result = match typ {
                    crate::tokens::DataType::Byte => {
                        let mut acc: Vec<u8> = Vec::new();
                        for value in values {
                            match value {
                                crate::tokens::DataValue::Number(n) => {
                                    if n > u8::MAX as u32 {
                                        panic!("Byte value overflow");
                                    }
                                    acc.push(n as u8);
                                },
                                crate::tokens::DataValue::String(s) => {
                                    acc.append(&mut s.chars().map(|c| c as u8).collect());
                                }
                            }
                        }
                        pack_u8_to_u32(acc)
                    },
                    crate::tokens::DataType::Word => {
                        let mut acc: Vec<u16> = Vec::new();
                        for value in values {
                            match value {
                                crate::tokens::DataValue::Number(n) => {
                                    if n > u16::MAX as u32 {
                                        panic!("Word value overflow");
                                    }
                                    acc.push(n as u16);
                                },
                                crate::tokens::DataValue::String(s) => {
                                    acc.append(&mut s.chars().map(|c| c as u16).collect());
                                }
                            }
                        }
                        pack_u16_to_u32(acc)
                    },
                    crate::tokens::DataType::DoubleWord => {
                        let mut acc: Vec<u32> = Vec::new();
                        for value in values {
                            match value {
                                crate::tokens::DataValue::Number(n) => {
                                    acc.push(n);
                                },
                                crate::tokens::DataValue::String(s) => {
                                    acc.append(&mut s.chars().map(|c| c as u32).collect());
                                }
                            }
                        }
                        acc
                    },
                };
                data_list.push((id, typ, result));
            },
            crate::tokens::Token::Command(cmd) => {
                check_section("text", &current_section);
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
                    crate::tokens::Cmd::Inc(reg) => {
                        result.push(combine_hl(Opcode::Inc as u32, OpcodeVariant::Default as u32));
                        result.push(reg);
                    },
                    crate::tokens::Cmd::PushIdAddress(id) => {
                        result.push(combine_hl(Opcode::Push as u32, OpcodeVariant::PushConst as u32));
                        data_usage.insert(result.len(), (id, 0));
                        result.push(0);
                    },
                    crate::tokens::Cmd::PushIdValueConst(id, offset) => {
                        result.push(combine_hl(Opcode::Push as u32, OpcodeVariant::PushAddr as u32));
                        data_usage.insert(result.len(), (id, offset));
                        result.push(0);
                    },
                    crate::tokens::Cmd::PushIdValueReg(id, reg) => {
                        result.push(combine_hl(Opcode::Push as u32, OpcodeVariant::PushAddrOffsetReg as u32));
                        data_usage.insert(result.len(), (id, 0));
                        result.push(0);
                        result.push(reg);
                    },
                    crate::tokens::Cmd::MoveIdAddress(reg, id) => {
                        result.push(combine_hl(Opcode::Move as u32, OpcodeVariant::MoveConst as u32));
                        result.push(reg);
                        data_usage.insert(result.len(), (id, 0));
                        result.push(0);
                    },
                    crate::tokens::Cmd::MoveIdValueConst(reg, id, offset) => {
                        result.push(combine_hl(Opcode::Move as u32, OpcodeVariant::MoveAddr as u32));
                        result.push(reg);
                        data_usage.insert(result.len(), (id, offset));
                        result.push(0);
                    },
                    crate::tokens::Cmd::MoveIdValueReg(reg, id, reg_v) => {
                        result.push(combine_hl(Opcode::Move as u32, OpcodeVariant::MoveAddrOffsetReg as u32));
                        result.push(reg);
                        data_usage.insert(result.len(), (id, 0));
                        result.push(0);
                        result.push(reg_v);
                    },
                    crate::tokens::Cmd::Dec(reg) => {
                        result.push(combine_hl(Opcode::Dec as u32, OpcodeVariant::Default as u32));
                        result.push(reg);
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
                    crate::tokens::Cmd::MoveAddrReg(reg, reg_val) => {
                        result.push(combine_hl(Opcode::Move as u32, OpcodeVariant::MoveAddrReg as u32));
                        result.push(reg);
                        result.push(reg_val);
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
                    crate::tokens::Cmd::SafeCallConst(const_value) => {
                        match const_value {
                            crate::tokens::ConstValue::Number(val) => {
                                result.push(combine_hl(Opcode::SafeCall as u32, OpcodeVariant::SafeCallConst as u32));
                                result.push(val);
                            },
                            crate::tokens::ConstValue::Label(label) => {
                                result.push(combine_hl(Opcode::SafeCall as u32, OpcodeVariant::SafeCallConst as u32));
                                label_usage.insert(result.len(), label);
                                result.push(0);
                            },
                        }
                    },
                    crate::tokens::Cmd::SafeCallReg(reg) => {
                        result.push(combine_hl(Opcode::SafeCall as u32, OpcodeVariant::SafeCallReg as u32));
                        result.push(reg);
                    },
                    crate::tokens::Cmd::SafeCallAddr(addr) => {
                        result.push(combine_hl(Opcode::SafeCall as u32, OpcodeVariant::SafeCallAddr as u32));
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
                check_section("text", &current_section);
                labels.insert(label, result.len());
            },
        }
    }
    for (k, v) in label_usage {
        result[k] = labels[v] as u32 + origin;
    }
    for (name, _typ, cont) in data_list {
        let addr = result.len() as u32 + origin;
        let _len = cont.len();
        cont.iter().for_each(|v| result.push(*v));
        data_lookup.insert(name, DataLookup { address: addr as u32 });
    }
    for (k, (v, offset)) in data_usage {
        result[k] = data_lookup[v].address + offset;
    }
    CompiledFrame{
        binary: result,
        origin: origin,
    }
}