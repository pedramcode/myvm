use std::fmt::Display;

use crate::{errors::VMError};

pub fn hexdump_to_string(data: &[u32]) -> String {
    const BYTES_PER_LINE: usize = 16;
    let mut output = String::new();

    for (i, chunk) in data.chunks(BYTES_PER_LINE / 4).enumerate() {
        let offset = i * BYTES_PER_LINE;
        output.push_str(&format!("{:08X}: ", offset));

        // Hex values
        for val in chunk {
            output.push_str(&format!("{:08X} ", val));
        }

        // Pad if last line is shorter
        for _ in 0..(BYTES_PER_LINE / 4 - chunk.len()) {
            output.push_str("         ");
        }

        // ASCII representation
        output.push('|');
        for val in chunk {
            let bytes = val.to_be_bytes(); // Big endian for readable dump
            for &b in &bytes {
                let c = if b.is_ascii_graphic() || b == b' ' { b as char } else { '.' };
                output.push(c);
            }
        }
        output.push('|');
        output.push('\n');
    }

    output
}

#[derive(Debug)]
/// # Memory
/// Memory structure is main storage of VM that contains all required data for VM in order to work
pub struct Memory {
    /// main storage component
    memory: Vec<u32>,
    /// stack size
    ssize: u32,
    /// stack pointer
    sp: u32,
}

impl Memory {
    /// creates new `Memory`
    /// 
    /// # Params
    /// 
    /// * `cells`: Number of memory cells (each is `u32`)
    /// * `stack_size`: Number of stack memory cells allocated on main memory (each is `u32`)
    pub fn new(cells: u32, stack_size: u32) -> Result<Self, VMError> {
        if stack_size >= cells {
            return Err(VMError::InvalidSize("Stack size cannot be more than total memory cells".to_string()));
        }
        Ok(Self{
            memory: vec![0u32; cells as usize],
            ssize: stack_size,
            sp: 0,
        })
    }

    /// push data into stack
    pub fn push(&mut self, data: u32) -> Result<(), VMError> {
        let len = self.memory.len();
        if (len - self.sp as usize) < (len - self.ssize as usize + 1) {
            return Err(VMError::StackOverflow);
        }
        self.memory[len - self.sp as usize - 1] = data;
        self.sp += 1;
        Ok(())
    }

    /// pop data from stack
    pub fn pop(&mut self) -> Result<u32, VMError> {
        let len = self.memory.len();
        if self.sp == 0 {
            return Err(VMError::EmptyContainer("Cannot pop from empty stack".to_string()));
        }
        self.sp -= 1;
        let result = self.memory[len - self.sp as usize - 1];
        Ok(result)
    }

    /// write data into memory
    pub fn write(&mut self, address: u32, data: &[u32]) -> Result<(), VMError> {
        let len = self.memory.len();
        let last_address = len - self.ssize as usize - 1;
        if address as usize + data.len() - 1 > last_address {
            return Err(VMError::InvalidAddress);
        }
        for i in 0..data.len() {
            self.memory[address as usize + i] = data[i];
        }
        Ok(())
    }

    /// read data from memory
    pub fn read(&self, address: u32) -> Result<u32, VMError> {
        if address as usize > self.memory.len() - 1 {
            return Err(VMError::InvalidAddress);
        }
        Ok(self.memory[address as usize])
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hexdump_to_string(&self.memory))
    }
}