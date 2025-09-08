#[derive(Debug)]
pub enum MetaType<'a> {
    Org(u32),
    Include(&'a str),
}

#[derive(Debug, Clone)]
pub enum ConstValue<'a> {
    Number(u32),
    Label(&'a str)
}

#[derive(Debug, Clone)]
pub enum DataType {
    Byte,
    Word,
    DoubleWord,
}

#[derive(Debug, Clone)]
pub enum DataValue<'a> {
    Number(u32),
    String(&'a str),
}

#[derive(Debug)]
pub enum DataAddressOffset<'a> {
    Zero(&'a str),
    Const(&'a str, u32),
    Reg(&'a str, u32),
}

#[derive(Debug, Clone)]
pub enum Cmd<'a> {
    PushConst(ConstValue<'a>),
    PushReg(u32),
    PushAddr(u32),
    PushIdAddress(&'a str),
    PushIdValueConst(&'a str, u32),
    PushIdValueReg(&'a str, u32),
    PopReg(u32),
    PopAddr(u32),
    Add,
    Drop,
    Sub,
    Swap,
    MoveConst(u32, ConstValue<'a>),
    MoveReg(u32, u32),
    MoveAddr(u32, u32),
    MoveAddrReg(u32, u32),
    MoveIdAddress(u32, &'a str),
    MoveIdValueConst(u32, &'a str, u32),
    MoveIdValueReg(u32, &'a str, u32),
    StoreConst(u32, ConstValue<'a>),
    StoreReg(u32, u32),
    Jmp(&'a str),
    Jnz(&'a str),
    Jz(&'a str),
    Jg(&'a str),
    Jge(&'a str),
    Jl(&'a str),
    Jle(&'a str),
    And,
    Or,
    Xor,
    Not,
    Mul,
    Div,
    Inc(u32),
    Dec(u32),
    ShrConst(u32),
    ShrReg(u32),
    ShlConst(u32),
    ShlReg(u32),
    CallConst(ConstValue<'a>),
    CallReg(u32),
    CallAddr(u32),
    SafeCallConst(ConstValue<'a>),
    SafeCallReg(u32),
    SafeCallAddr(u32),
    Ret,
    Dup,
    DupConst(u32),
    DupReg(u32),
    Int(u32, u32),
    Term,
}

#[derive(Debug)]
pub enum Token<'a> {
    Meta(MetaType<'a>),
    Command(Cmd<'a>),
    Label(&'a str),
    Section(&'a str),
    DataDef(&'a str, DataType, Vec<DataValue<'a>>),
}
