use crate::errors::VMError;

#[derive(Debug)]
/// # Registers
/// 
/// required registers for VM
/// 
/// # Mapping
/// 
/// in order to access registers, there are some a mapping for each register:
/// 
/// * **0** = r0
/// * **1** = r1
/// * **2** = r2
/// * **3** = r3
/// * **4** = r4
/// * **5** = r5
/// * **6** = r6
/// * **7** = r7
/// * **8** = r8
/// * **100** = pc
pub struct Register {
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r4: u32,
    pub r5: u32,
    pub r6: u32,
    pub r7: u32,
    pub pc: u32,
}

impl Register {
    pub fn new() -> Self {
        Self {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            pc: 0,
        }
    }

    pub fn set(&mut self, reg_num: u32, value: u32) -> Result<(), VMError> {
        match reg_num {
            0 => self.r0 = value,
            1 => self.r1 = value,
            2 => self.r2 = value,
            3 => self.r3 = value,
            4 => self.r4 = value,
            5 => self.r5 = value,
            6 => self.r6 = value,
            7 => self.r7 = value,
            100 => self.pc = value,
            _ => {
                return Err(VMError::InvalidRegister)
            }
        }
        Ok(())
    }

    pub fn get(&self, reg_num: u32) -> Result<u32, VMError> {
        match reg_num {
            0 => Ok(self.r0),
            1 => Ok(self.r1),
            2 => Ok(self.r2),
            3 => Ok(self.r3),
            4 => Ok(self.r4),
            5 => Ok(self.r5),
            6 => Ok(self.r6),
            7 => Ok(self.r7),
            100 => Ok(self.pc),
            _ => Err(VMError::InvalidRegister),
        }
    }
}