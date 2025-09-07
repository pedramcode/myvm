use crate::errors::VMError;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// # Opcode
/// 
/// VM commands or `Opcode`s are set of operations to interact with VM 
pub enum Opcode {
    Push = 0xf001,
    Pop = 0xf002,
    Add = 0xf003,
    Sub = 0xf004,
    Swap = 0xf005,
    Move = 0xf006,
    Store = 0xf007,
    Jump = 0xf008,
    And = 0xf009,
    Or = 0xf00a,
    Xor = 0xf00b,
    Not = 0xf00c,
    SHR = 0xf00d,
    SHL = 0xf00e,
    Call = 0xf00f,
    Ret = 0xf010,
    Dup = 0xf011,
    Int = 0xf012,
    Drop = 0xf013,
    Mul = 0xf014,
    Div = 0xf015,
    Inc = 0xf016,
    Dec = 0xf017,
    Terminate = 0xffff,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// # Opcode variant
/// 
/// Each `Opcode` may have it's own variant and do things in different ways
pub enum OpcodeVariant {
    /// default variant
    Default = 0x0000,
    /// push a constant into stack
    PushConst = 0xa001,
    /// push value of register into stack
    PushReg = 0xa002,
    /// push value in an address into stack
    PushAddr = 0xa003,
    /// pop stack into a register
    PopReg = 0xa004,
    /// pop stack into an address
    PopAddr = 0xa005,
    /// move constant into register
    MoveConst = 0xa006,
    /// move register value into other register
    MoveReg = 0xa007,
    /// move address value into register
    MoveAddr = 0xa008,
    /// store a constant value into memory
    StoreConst = 0xa009,
    /// store a register value into memory
    StoreReg = 0xa00a,
    /// jump if not zero
    JumpNotZero = 0xa00b,
    /// jump if zero
    JumpZero = 0xa00c,
    /// jump if greater
    JumpGreater = 0xa00d,
    /// jump if greater or equal
    JumpGreaterEqual = 0xa00e,
    /// jump if lesser
    JumpLesser = 0xa00f,
    /// jump if lesser or equal
    JumpLesserEqual = 0xa010,
    /// shift right by constant
    SHRConst = 0xa011,
    /// shift right by register value
    SHRReg = 0xa012,
    /// shift left by constant
    SHLConst = 0xa013,
    /// shift left by register value
    SHLReg = 0xa014,
    /// call constant address
    CallConst = 0xa015,
    /// call register value address
    CallReg = 0xa016,
    /// call address value address
    CallAddr = 0xa017,
    /// duplicate constant
    DupConst = 0xa018,
    /// duplicate register
    DupReg = 0xa019,
}

impl Opcode {
    pub fn from_num(value: u32) -> Result<Opcode, VMError> {
        match value {
            x if x == Self::Push as u32 => Ok(Self::Push),
            x if x == Self::Pop as u32 => Ok(Self::Pop),
            x if x == Self::Add as u32 => Ok(Self::Add),
            x if x == Self::Sub as u32 => Ok(Self::Sub),
            x if x == Self::Swap as u32 => Ok(Self::Swap),
            x if x == Self::Terminate as u32 => Ok(Self::Terminate),
            x if x == Self::Move as u32 => Ok(Self::Move),
            x if x == Self::Jump as u32 => Ok(Self::Jump),
            x if x == Self::Store as u32 => Ok(Self::Store),
            x if x == Self::And as u32 => Ok(Self::And),
            x if x == Self::Or as u32 => Ok(Self::Or),
            x if x == Self::Xor as u32 => Ok(Self::Xor),
            x if x == Self::Not as u32 => Ok(Self::Not),
            x if x == Self::SHR as u32 => Ok(Self::SHR),
            x if x == Self::SHL as u32 => Ok(Self::SHL),
            x if x == Self::Call as u32 => Ok(Self::Call),
            x if x == Self::Ret as u32 => Ok(Self::Ret),
            x if x == Self::Dup as u32 => Ok(Self::Dup),
            x if x == Self::Int as u32 => Ok(Self::Int),
            x if x == Self::Drop as u32 => Ok(Self::Drop),
            x if x == Self::Mul as u32 => Ok(Self::Mul),
            x if x == Self::Div as u32 => Ok(Self::Div),
            x if x == Self::Inc as u32 => Ok(Self::Inc),
            x if x == Self::Dec as u32 => Ok(Self::Dec),
            _ => {
                return Err(VMError::InvalidOpcode);
            }
        }
    }

    pub fn extract(value: u32) -> Result<(Opcode, OpcodeVariant), VMError> {
        let high = value >> 16;
        let low = value & 0xffff;
        let opcode = Self::from_num(high)?;
        let variant = OpcodeVariant::from_num(low)?;
        Ok((opcode, variant))
    }
}

impl OpcodeVariant {
    pub fn from_num(value: u32) -> Result<OpcodeVariant, VMError> {
        match value {
            x if x == Self::Default as u32 => Ok(Self::Default),
            x if x == Self::PushConst as u32 => Ok(Self::PushConst),
            x if x == Self::PushReg as u32 => Ok(Self::PushReg),
            x if x == Self::PushAddr as u32 => Ok(Self::PushAddr),
            x if x == Self::PopReg as u32 => Ok(Self::PopReg),
            x if x == Self::PopAddr as u32 => Ok(Self::PopAddr),
            x if x == Self::MoveAddr as u32 => Ok(Self::MoveAddr),
            x if x == Self::MoveConst as u32 => Ok(Self::MoveConst),
            x if x == Self::MoveReg as u32 => Ok(Self::MoveReg),
            x if x == Self::StoreConst as u32 => Ok(Self::StoreConst),
            x if x == Self::StoreReg as u32 => Ok(Self::StoreReg),
            x if x == Self::JumpNotZero as u32 => Ok(Self::JumpNotZero),
            x if x == Self::JumpZero as u32 => Ok(Self::JumpZero),
            x if x == Self::JumpGreater as u32 => Ok(Self::JumpGreater),
            x if x == Self::JumpGreaterEqual as u32 => Ok(Self::JumpGreaterEqual),
            x if x == Self::JumpLesser as u32 => Ok(Self::JumpLesser),
            x if x == Self::JumpLesserEqual as u32 => Ok(Self::JumpLesserEqual),
            x if x == Self::SHRConst as u32 => Ok(Self::SHRConst),
            x if x == Self::SHRReg as u32 => Ok(Self::SHRReg),
            x if x == Self::SHLConst as u32 => Ok(Self::SHLConst),
            x if x == Self::SHLReg as u32 => Ok(Self::SHLReg),
            x if x == Self::CallConst as u32 => Ok(Self::CallConst),
            x if x == Self::CallReg as u32 => Ok(Self::CallReg),
            x if x == Self::CallAddr as u32 => Ok(Self::CallAddr),
            x if x == Self::DupConst as u32 => Ok(Self::DupConst),
            x if x == Self::DupReg as u32 => Ok(Self::DupReg),
            _ => Err(VMError::InvalidOpcode)
        }
    }
}