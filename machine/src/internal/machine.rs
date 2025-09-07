use crate::{errors::VMError, internal::{flag::Flag, interrupts::handler::interrupt_handler, memory::Memory, opcode::{Opcode, OpcodeVariant}, register::Register}};

#[derive(Debug)]
/// Machine initialization options
pub struct MachineOptions {
    pub memory_cells: u32,
    pub memory_stack_size: u32,
}

#[derive(Debug)]
/// # Machine
/// 
/// Main VM struct which is containing all of required components for VM
pub struct Machine {
    pub memory: Memory,
    pub register: Register,
    pub flag: Flag,
    call_stack: Vec<u32>,
}

impl Machine {
    /// creates new virtual machine
    pub fn new(options: MachineOptions) -> Result<Self, VMError> {
        let memory = Memory::new(options.memory_cells, options.memory_stack_size)?;
        return Ok(Self{
            memory: memory,
            register: Register::new(),
            flag: Flag::new(),
            call_stack: Vec::new(),
        });
    }

    /// load data into memory
    pub fn load_data(&mut self, address: u32, data: &[u32]) -> Result<(), VMError> {
        self.memory.write(address, data)?;
        Ok(())
    }

    /// set origin of machine in memory (where to start running code)
    pub fn set_origin(&mut self, address: u32) {
        self.register.pc = address;
    }

    /// read register value
    pub fn read_register(&self, reg_num: u32) -> Result<u32, VMError> {
        Ok(self.register.get(reg_num)?)
    }

    /// execute next command
    /// 
    /// # Return
    /// 
    /// it will return true on execution done (reached the terminate opcode)
    fn execute_next(&mut self) -> Result<bool, VMError> {
        let (opcode, opcode_var) = Opcode::extract(self.memory.read(self.register.pc)?)?;
        let mut jumped = false;
        match (opcode, opcode_var) {
            (Opcode::Push, OpcodeVariant::PushConst) => {
                self.register.pc += 1;
                let next = self.memory.read(self.register.pc)?;
                self.memory.push(next)?;
            },
            (Opcode::Push, OpcodeVariant::PushReg) => {
                self.register.pc += 1;
                let next = self.memory.read(self.register.pc)?;
                let value = self.register.get(next)?;
                self.memory.push(value)?;
            },
            (Opcode::Push, OpcodeVariant::PushAddr) => {
                self.register.pc += 1;
                let next = self.memory.read(self.register.pc)?;
                let value = self.memory.read(next)?;
                self.memory.push(value)?;
            },
            (Opcode::Pop, OpcodeVariant::PopReg) => {
                self.register.pc += 1;
                let next = self.memory.read(self.register.pc)?;
                let value = self.memory.pop()?;
                self.flag.zero = value == 0;
                self.flag.negative = (value as i32) < 0;
                self.register.set(next, value)?;
            },
            (Opcode::Pop, OpcodeVariant::PopAddr) => {
                self.register.pc += 1;
                let next = self.memory.read(self.register.pc)?;
                let value = self.memory.pop()?;
                self.flag.zero = value == 0;
                self.flag.negative = (value as i32) < 0;
                self.memory.write(next, &[value])?;
            },
            (Opcode::Drop, OpcodeVariant::Default) => {
                let value = self.memory.pop()?;
                self.flag.zero = value == 0;
                self.flag.negative = (value as i32) < 0;
            },
            (Opcode::Terminate, OpcodeVariant::Default) => {
                return Ok(true);
            },
            (Opcode::Add, OpcodeVariant::Default) => {
                let b = self.memory.pop()? as i32;
                let a = self.memory.pop()? as i32;
                let result = b.wrapping_add(a);
                self.flag.zero = result == 0;
                self.flag.negative = result < 0;
                self.flag.overflow = (b > 0 && a > 0 && result < 0) || (b < 0 && a < 0 && result > 0);
                let carry = (b as u32).overflowing_add(a as u32).1;
                self.flag.carry = carry;
                self.memory.push(result as u32)?;
            },
            (Opcode::Sub, OpcodeVariant::Default) => {
                let b = self.memory.pop()? as i32;
                let a = self.memory.pop()? as i32;
                let result = b.wrapping_sub(a);
                self.flag.zero = result == 0;
                self.flag.negative = result < 0;
                self.flag.overflow = (b > 0 && a < 0 && result < 0) || (b < 0 && a > 0 && result > 0);
                let (_res, borrow) = (b as u32).overflowing_sub(a as u32);
                self.flag.carry = !borrow;
                self.memory.push(result as u32)?;
            },
            (Opcode::Swap, OpcodeVariant::Default) => {
                let a = self.memory.pop()?;
                let b = self.memory.pop()?;
                self.memory.push(a)?;
                self.memory.push(b)?;
            },
            (Opcode::Move, OpcodeVariant::MoveConst) => {
                self.register.pc += 1;
                let reg = self.memory.read(self.register.pc)?;
                self.register.pc += 1;
                let value = self.memory.read(self.register.pc)?;
                self.register.set(reg, value)?;
            },
            (Opcode::Move, OpcodeVariant::MoveReg) => {
                self.register.pc += 1;
                let reg = self.memory.read(self.register.pc)?;
                self.register.pc += 1;
                let value = self.memory.read(self.register.pc)?;
                self.register.set(reg, self.register.get(value)?)?;
            },
            (Opcode::Move, OpcodeVariant::MoveAddr) => {
                self.register.pc += 1;
                let reg = self.memory.read(self.register.pc)?;
                self.register.pc += 1;
                let value = self.memory.read(self.register.pc)?;
                self.register.set(reg, self.memory.read(value)?)?;
            },
            (Opcode::Store, OpcodeVariant::StoreConst) => {
                self.register.pc += 1;
                let addr = self.memory.read(self.register.pc)?;
                self.register.pc += 1;
                let value = self.memory.read(self.register.pc)?;
                self.memory.write(addr, &[value])?;
            },
            (Opcode::Store, OpcodeVariant::StoreReg) => {
                self.register.pc += 1;
                let addr = self.memory.read(self.register.pc)?;
                self.register.pc += 1;
                let value = self.memory.read(self.register.pc)?;
                self.memory.write(addr, &[self.register.get(value)?])?;
            },
            (Opcode::Jump, OpcodeVariant::JumpNotZero) => {
                self.register.pc += 1;
                let addr = self.memory.read(self.register.pc)?;
                if !self.flag.zero {
                    self.register.pc = addr;
                    jumped = true;
                }
            },
            (Opcode::Jump, OpcodeVariant::Default) => {
                self.register.pc += 1;
                let addr = self.memory.read(self.register.pc)?;
                self.register.pc = addr;
                jumped = true;
            },
            (Opcode::Mul, OpcodeVariant::Default) => {
                let b = self.memory.pop()? as i32;
                let a = self.memory.pop()? as i32;
                let result = b.wrapping_mul(a);
                self.flag.zero = result == 0;
                self.flag.negative = result < 0;
                // Overflow detection (signed multiplication)
                self.flag.overflow = a != 0 && result / a != b;
                // Carry flag not meaningful here, but you may set it if you want
                self.flag.carry = false;
                self.memory.push(result as u32)?;
            },
            (Opcode::Div, OpcodeVariant::Default) => {
                let a = self.memory.pop()?; // u32
                let b = self.memory.pop()?; // u32
                if b == 0 {
                    return Err(VMError::DivisionByZero);
                }
                let quotient = a / b;
                let remainder = a % b;
                self.register.r3 = remainder;
                self.memory.push(quotient)?; // always u32
                self.flag.zero = quotient == 0;
                self.flag.negative = false; // treat as unsigned
                self.flag.overflow = false;
                self.flag.carry = false;
            },

            (Opcode::Jump, OpcodeVariant::JumpZero) => {
                self.register.pc += 1;
                let addr = self.memory.read(self.register.pc)?;
                if self.flag.zero {
                    self.register.pc = addr;
                    jumped = true;
                }
            },
            (Opcode::Jump, OpcodeVariant::JumpGreater) => {
                self.register.pc += 1;
                let addr = self.memory.read(self.register.pc)?;
                if !self.flag.zero && (self.flag.negative == self.flag.overflow) {
                    self.register.pc = addr;
                    jumped = true;
                }
            },
            (Opcode::Jump, OpcodeVariant::JumpGreaterEqual) => {
                self.register.pc += 1;
                let addr = self.memory.read(self.register.pc)?;
                if self.flag.negative == self.flag.overflow {
                    self.register.pc = addr;
                    jumped = true;
                }
            },
            (Opcode::Jump, OpcodeVariant::JumpLesser) => {
                self.register.pc += 1;
                let addr = self.memory.read(self.register.pc)?;
                if self.flag.negative != self.flag.overflow {
                    self.register.pc = addr;
                    jumped = true;
                }
            },
            (Opcode::Jump, OpcodeVariant::JumpLesserEqual) => {
                self.register.pc += 1;
                let addr = self.memory.read(self.register.pc)?;
                if self.flag.zero || (self.flag.negative != self.flag.overflow) {
                    self.register.pc = addr;
                    jumped = true;
                }
            },
            (Opcode::And, OpcodeVariant::Default) => {
                let a = self.memory.pop()?;
                let b = self.memory.pop()?;
                self.memory.push(a & b)?;
            },
            (Opcode::Or, OpcodeVariant::Default) => {
                let a = self.memory.pop()?;
                let b = self.memory.pop()?;
                self.memory.push(a | b)?;
            },
            (Opcode::Xor, OpcodeVariant::Default) => {
                let a = self.memory.pop()?;
                let b = self.memory.pop()?;
                self.memory.push(a ^ b)?;
            },
            (Opcode::Not, OpcodeVariant::Default) => {
                let a = self.memory.pop()?;
                self.memory.push(!a)?;
            },
            (Opcode::SHR, OpcodeVariant::SHRConst) => {
                self.register.pc += 1;
                let amount = self.memory.read(self.register.pc)?;
                let value = self.memory.pop()?;
                self.memory.push(value >> amount)?;
            },
            (Opcode::SHL, OpcodeVariant::SHLConst) => {
                self.register.pc += 1;
                let amount = self.memory.read(self.register.pc)?;
                let value = self.memory.pop()?;
                self.memory.push(value << amount)?;
            },
            (Opcode::SHR, OpcodeVariant::SHRReg) => {
                self.register.pc += 1;
                let amount = self.register.get(self.memory.read(self.register.pc)?)?;
                let value = self.memory.pop()?;
                self.memory.push(value >> amount)?;
            },
            (Opcode::SHL, OpcodeVariant::SHLReg) => {
                self.register.pc += 1;
                let amount = self.register.get(self.memory.read(self.register.pc)?)?;
                let value = self.memory.pop()?;
                self.memory.push(value << amount)?;
            },
            (Opcode::Call, OpcodeVariant::CallConst) => {
                self.register.pc += 1;
                let addr = self.memory.read(self.register.pc)?;
                self.call_stack.push(self.register.pc);
                self.register.pc = addr;
                jumped = true;
            },
            (Opcode::Call, OpcodeVariant::CallReg) => {
                self.register.pc += 1;
                let addr = self.register.get(self.memory.read(self.register.pc)?)?;
                self.call_stack.push(self.register.pc);
                self.register.pc = addr;
                jumped = true;
            },
            (Opcode::Call, OpcodeVariant::CallAddr) => {
                self.register.pc += 1;
                let addr = self.memory.read(self.memory.read(self.register.pc)?)?;
                self.call_stack.push(self.register.pc);
                self.register.pc = addr;
                jumped = true;
            },
            (Opcode::Ret, OpcodeVariant::Default) => {
                let addr = self.call_stack.pop();
                if addr.is_none() {
                    return Err(VMError::InvalidReturn);
                }
                self.register.pc = addr.unwrap() + 1;
                jumped = true;
            },
            (Opcode::Dup, OpcodeVariant::Default) => {
                let a = self.memory.pop()?;
                self.flag.zero = a == 0;
                self.flag.negative = (a as i32) < 0;
                self.memory.push(a)?;
                self.memory.push(a)?;
            },
            (Opcode::Dup, OpcodeVariant::DupConst) => {
                self.register.pc += 1;
                let amount = self.memory.read(self.register.pc)?;
                for _ in 0..amount {
                    let a = self.memory.pop()?;
                    self.flag.zero = a == 0;
                    self.flag.negative = (a as i32) < 0;
                    self.memory.push(a)?;
                    self.memory.push(a)?;
                }
            },
            (Opcode::Inc, OpcodeVariant::Default) => {
                self.register.pc += 1;
                let reg = self.memory.read(self.register.pc)?;
                self.register.set(reg, self.register.get(reg)? + 1)?;
            },
            (Opcode::Dec, OpcodeVariant::Default) => {
                self.register.pc += 1;
                let reg = self.memory.read(self.register.pc)?;
                self.register.set(reg, self.register.get(reg)? - 1)?;
            },
            (Opcode::Dup, OpcodeVariant::DupReg) => {
                self.register.pc += 1;
                let amount = self.register.get(self.memory.read(self.register.pc)?)?;
                for _ in 0..amount {
                    let a = self.memory.pop()?;
                    self.flag.zero = a == 0;
                    self.flag.negative = (a as i32) < 0;
                    self.memory.push(a)?;
                    self.memory.push(a)?;
                }
            },
            (Opcode::Int, OpcodeVariant::Default) => {
                self.register.pc += 1;
                let module = self.memory.read(self.register.pc)?;
                self.register.pc += 1;
                let function = self.memory.read(self.register.pc)?;
                interrupt_handler(self, module, function)?;
            },
            _ => {
                return Err(VMError::InvalidOpcode);
            },
        }
        if !jumped{
            self.register.pc += 1;
        }
        Ok(false)
    }

    /// Execute code that loaded into memory. start from PC register address.
    pub fn execute(&mut self) -> Result<(), VMError> {
        loop {
            let done = self.execute_next()?;
            if done {
                break;
            }
        }
        Ok(())
    }
}