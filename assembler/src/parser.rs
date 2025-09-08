use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_till, take_until, take_while1},
    character::{complete::{alphanumeric1, char, digit1, line_ending, multispace1, space0}, multispace0},
    combinator::{map, map_res, opt, value},
    error::{Error, ErrorKind},
    multi::separated_list0,
    sequence::{delimited, pair, preceded, terminated},
    Err, IResult, Parser,
};

use crate::tokens::{Cmd, ConstValue, DataAddressOffset, DataType, DataValue, MetaType, Token};

// ----------------- Basic parsers -----------------

pub fn parse_comment(input: &str) -> IResult<&str, ()> {
    // A comment starts with `;` and goes until newline or EOF
    let (rem, _) = preceded(
        char(';'),
        take_till(|c| c == '\n')
    ).parse(input)?;
    Ok((rem, ()))
}

pub fn parse_number(input: &str) -> IResult<&str, u32> {
    alt((
        map_res(preceded(tag_no_case("0x"), take_while1(|c: char| c.is_ascii_hexdigit())), |s: &str| u32::from_str_radix(s, 16)),
        map_res(preceded(tag_no_case("0b"), take_while1(|c| c == '0' || c == '1')), |s: &str| u32::from_str_radix(s, 2)),
        map_res(digit1, str::parse::<u32>),
    ))
    .parse(input)
}

pub fn parse_str(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), take_until("\""), char('"')).parse(input)
}

pub fn parse_label(input: &str) -> IResult<&str, &str> {
    preceded(tag("."), alphanumeric1).parse(input)
}

pub fn parse_address(input: &str) -> IResult<&str, u32> {
    preceded(tag("&"), parse_number).parse(input)
}

pub fn parse_const_value(input: &str) -> IResult<&str, ConstValue<'_>> {
    alt((
        map(parse_number, ConstValue::Number),
        map(parse_label, ConstValue::Label),
    ))
    .parse(input)
}

pub fn parse_meta(input: &str) -> IResult<&str, MetaType<'_>> {
    let (rem, keyword) = preceded(
        tag_no_case("@"),
        alt((tag_no_case("org"), tag_no_case("include")))
    ).parse(input)?;

    if keyword.eq_ignore_ascii_case("org") {
        let (rem, number) = preceded(multispace1, parse_number).parse(rem)?;
        Ok((rem, MetaType::Org(number)))
    } else if keyword.eq_ignore_ascii_case("include") {
        let (rem, path) = preceded(multispace1, parse_str).parse(rem)?;
        Ok((rem, MetaType::Include(path)))
    } else {
        Err(Err::Error(Error::new(input, ErrorKind::Tag)))
    }
}

pub fn parse_section(input: &str) -> IResult<&str, &str> {
    delimited(tag("["), alphanumeric1, tag("]")).parse(input)
}



// -----------------      Data       -----------------


pub fn parse_data_type(input: &str) -> IResult<&str, DataType> {
    alt((
        value(DataType::Byte, tag_no_case("b")),
        value(DataType::Word, tag_no_case("w")),
        value(DataType::DoubleWord, tag_no_case("dw")),
    )).parse(input)
}

pub fn parse_data_values(input: &'_ str) -> IResult<&'_ str, Vec<DataValue<'_>>> {
    separated_list0(multispace1, alt((
        map(parse_number, DataValue::Number),
        map(parse_str, DataValue::String),
    ))).parse(input)
}

pub fn parse_identifier(input: &str) -> IResult<&str, &str> {
    preceded(tag("$"), alphanumeric1).parse(input)
}

pub fn parse_data_def(input: &'_ str) -> IResult<&'_ str, Token<'_>> {
    let (rem, id) = preceded(multispace0(), parse_identifier).parse(input)?;
    let (rem, typ) = preceded(multispace1, parse_data_type).parse(rem)?;
    let (rem, values) = preceded(multispace1, parse_data_values).parse(rem)?;
    Ok((rem, Token::DataDef(id, typ, values)))
}

pub fn parse_id_address(input: &'_ str) -> IResult<&'_ str, Cmd<'_>> {
    map(delimited(tag("["), parse_identifier, tag("]")), Cmd::PushIdAddress).parse(input)
}

pub fn parse_id_offset_const(input: &'_ str) -> IResult<&'_ str, DataAddressOffset<'_>> {
    let (rem , id) = preceded(multispace0(),parse_identifier).parse(input)?;
    let (rem, _) = preceded(multispace0(), tag("+")).parse(rem)?;
    let (rem, number) = preceded(multispace0(), parse_number).parse(rem)?;
    Ok((rem, DataAddressOffset::Const(id, number)))
}

pub fn parse_id_offset_zero(input: &'_ str) -> IResult<&'_ str, DataAddressOffset<'_>> {
    let (rem , id) = preceded(multispace0(),parse_identifier).parse(input)?;
    Ok((rem, DataAddressOffset::Zero(id)))
}

pub fn parse_id_offset_reg(input: &'_ str) -> IResult<&'_ str, DataAddressOffset<'_>> {
    let (rem , id) = preceded(multispace0(),parse_identifier).parse(input)?;
    let (rem, _) = preceded(multispace0(), tag("+")).parse(rem)?;
    let (rem, number) = terminated(preceded(multispace0(), parse_reg), multispace0()).parse(rem)?;
    Ok((rem, DataAddressOffset::Reg(id, number)))
}

pub fn parse_id_address_with_offset(input: &'_ str) -> IResult<&'_ str, DataAddressOffset<'_>> {
    delimited(
        tag("["),
        alt((parse_id_offset_reg, parse_id_offset_const, parse_id_offset_zero)),
        tag("]")
    ).parse(input)
}

pub fn parse_push_id_address(input: &'_ str) -> IResult<&'_ str, Cmd<'_>> {
    let (rem, _) = tag_no_case("push").parse(input)?;
    let (rem, id) = preceded(multispace1, parse_identifier).parse(rem)?;
    Ok((rem, Cmd::PushIdAddress(id)))
}

pub fn parse_push_id_value(input: &'_ str) -> IResult<&'_ str, Cmd<'_>> {
    let (rem, _) = tag_no_case("push").parse(input)?;
    let (rem, val) = preceded(multispace1, parse_id_address_with_offset).parse(rem)?;
    match val {
        DataAddressOffset::Zero(id) => Ok((rem, Cmd::PushIdValueConst(id, 0))),
        DataAddressOffset::Const(id, n) => Ok((rem, Cmd::PushIdValueConst(id, n))),
        DataAddressOffset::Reg(id, r) => Ok((rem, Cmd::PushIdValueReg(id, r))),
    }
}


// ----------------- Register parser -----------------

pub fn parse_reg(input: &str) -> IResult<&str, u32> {
    let (rem, reg) = alt((
        tag_no_case("r0"), tag_no_case("r1"), tag_no_case("r2"), tag_no_case("r3"),
        tag_no_case("r4"), tag_no_case("r5"), tag_no_case("r6"), tag_no_case("r7"),
        tag_no_case("pc"),
    ))
    .parse(input)?;

    let code = match reg.to_ascii_lowercase().as_str() {
        "r0" => 0, "r1" => 1, "r2" => 2, "r3" => 3,
        "r4" => 4, "r5" => 5, "r6" => 6, "r7" => 7,
        "pc" => 100,
        _ => return Err(Err::Error(Error::new(input, ErrorKind::Char))),
    };

    Ok((rem, code))
}

// ----------------- Helper parsers -----------------

pub fn parse_number_or_const(input: &str) -> IResult<&str, ConstValue<'_>> {
    alt((
        map(parse_reg, ConstValue::Number),
        map(parse_address, ConstValue::Number),  // Add this line to handle addresses
        parse_const_value,
    ))
    .parse(input)
}

macro_rules! unary_cmd {
    ($kw:expr, $parser:expr, $variant:ident) => {
        |input| {
            let (rem, val) = preceded(pair(tag_no_case($kw), multispace1), $parser).parse(input)?;
            Ok((rem, Cmd::$variant(val)))
        }
    };
}

macro_rules! keyword_cmd {
    ($kw:expr, $variant:ident) => {
        map(tag_no_case($kw), |_| Cmd::$variant)
    };
}

// ----------------- Unary commands -----------------

fn parse_push_const(input: &str) -> IResult<&str, Cmd<'_>> { unary_cmd!("push", parse_const_value, PushConst)(input) }
fn parse_push_reg(input: &str) -> IResult<&str, Cmd<'_>> { unary_cmd!("push", parse_reg, PushReg)(input) }
fn parse_inc(input: &str) -> IResult<&str, Cmd<'_>> { unary_cmd!("inc", parse_reg, Inc)(input) }
fn parse_dec(input: &str) -> IResult<&str, Cmd<'_>> { unary_cmd!("dec", parse_reg, Dec)(input) }
fn parse_push_address(input: &str) -> IResult<&str, Cmd<'_>> { unary_cmd!("push", parse_address, PushAddr)(input) }
fn parse_pop_reg(input: &str) -> IResult<&str, Cmd<'_>> { unary_cmd!("pop", parse_reg, PopReg)(input) }
fn parse_pop_address(input: &str) -> IResult<&str, Cmd<'_>> { unary_cmd!("pop", parse_address, PopAddr)(input) }
fn parse_dup_const(input: &str) -> IResult<&str, Cmd<'_>> { unary_cmd!("dup", parse_number, DupConst)(input) }
fn parse_dup_reg(input: &str) -> IResult<&str, Cmd<'_>> { unary_cmd!("dup", parse_reg, DupReg)(input) }
fn parse_dup(input: &str) -> IResult<&str, Cmd<'_>> { map(tag_no_case("dup"), |_| Cmd::Dup).parse(input) }
fn parse_shr_const(input: &str) -> IResult<&str, Cmd<'_>> { unary_cmd!("shr", parse_number, ShrConst)(input) }
fn parse_shr_reg(input: &str) -> IResult<&str, Cmd<'_>> { unary_cmd!("shr", parse_reg, ShrReg)(input) }
fn parse_shl_const(input: &str) -> IResult<&str, Cmd<'_>> { unary_cmd!("shl", parse_number, ShlConst)(input) }
fn parse_shl_reg(input: &str) -> IResult<&str, Cmd<'_>> { unary_cmd!("shl", parse_reg, ShlReg)(input) }
fn parse_call_const(input: &str) -> IResult<&str, Cmd<'_>> { unary_cmd!("call", parse_const_value, CallConst)(input) }
fn parse_call_reg(input: &str) -> IResult<&str, Cmd<'_>> { unary_cmd!("call", parse_reg, CallReg)(input) }
fn parse_call_address(input: &str) -> IResult<&str, Cmd<'_>> { unary_cmd!("call", parse_address, CallAddr)(input) }

// ----------------- Keyword commands -----------------

fn parse_add(input: &str) -> IResult<&str, Cmd<'_>> { keyword_cmd!("add", Add).parse(input) }
fn parse_drop(input: &str) -> IResult<&str, Cmd<'_>> { keyword_cmd!("drop", Drop).parse(input) }
fn parse_sub(input: &str) -> IResult<&str, Cmd<'_>> { keyword_cmd!("sub", Sub).parse(input) }
fn parse_swap(input: &str) -> IResult<&str, Cmd<'_>> { keyword_cmd!("swap", Swap).parse(input) }
fn parse_and(input: &str) -> IResult<&str, Cmd<'_>> { keyword_cmd!("and", And).parse(input) }
fn parse_or(input: &str) -> IResult<&str, Cmd<'_>> { keyword_cmd!("or", Or).parse(input) }
fn parse_xor(input: &str) -> IResult<&str, Cmd<'_>> { keyword_cmd!("xor", Xor).parse(input) }
fn parse_not(input: &str) -> IResult<&str, Cmd<'_>> { keyword_cmd!("not", Not).parse(input) }
fn parse_ret(input: &str) -> IResult<&str, Cmd<'_>> { keyword_cmd!("ret", Ret).parse(input) }
fn parse_term(input: &str) -> IResult<&str, Cmd<'_>> { keyword_cmd!("term", Term).parse(input) }
fn parse_mul(input: &str) -> IResult<&str, Cmd<'_>> { keyword_cmd!("mul", Mul).parse(input) }
fn parse_div(input: &str) -> IResult<&str, Cmd<'_>> { keyword_cmd!("div", Div).parse(input) }

// ----------------- Jump commands (parse label as target) -----------------

fn parse_jmp(input: &str) -> IResult<&str, Cmd<'_>> {
    let (rem, _) = tag_no_case("jmp")(input)?;
    let (rem, target) = preceded(multispace1, parse_label).parse(rem)?;
    Ok((rem, Cmd::Jmp(target)))
}

fn parse_jnz(input: &str) -> IResult<&str, Cmd<'_>> {
    let (rem, _) = tag_no_case("jnz")(input)?;
    let (rem, target) = preceded(multispace1, parse_label).parse(rem)?;
    Ok((rem, Cmd::Jnz(target)))
}

fn parse_jz(input: &str) -> IResult<&str, Cmd<'_>> {
    let (rem, _) = tag_no_case("jz")(input)?;
    let (rem, target) = preceded(multispace1, parse_label).parse(rem)?;
    Ok((rem, Cmd::Jz(target)))
}

fn parse_jg(input: &str) -> IResult<&str, Cmd<'_>> {
    let (rem, _) = tag_no_case("jg")(input)?;
    let (rem, target) = preceded(multispace1, parse_label).parse(rem)?;
    Ok((rem, Cmd::Jg(target)))
}

fn parse_jge(input: &str) -> IResult<&str, Cmd<'_>> {
    let (rem, _) = tag_no_case("jge")(input)?;
    let (rem, target) = preceded(multispace1, parse_label).parse(rem)?;
    Ok((rem, Cmd::Jge(target)))
}

fn parse_jl(input: &str) -> IResult<&str, Cmd<'_>> {
    let (rem, _) = tag_no_case("jl")(input)?;
    let (rem, target) = preceded(multispace1, parse_label).parse(rem)?;
    Ok((rem, Cmd::Jl(target)))
}

fn parse_jle(input: &str) -> IResult<&str, Cmd<'_>> {
    let (rem, _) = tag_no_case("jle")(input)?;
    let (rem, target) = preceded(multispace1, parse_label).parse(rem)?;
    Ok((rem, Cmd::Jle(target)))
}



// ----------------- MOVE / STORE -----------------


fn parse_move(input: &str) -> IResult<&str, Cmd<'_>> {
    let (rem, _) = tag_no_case("move").parse(input)?;
    let (rem, dest) = delimited(multispace1, parse_reg, multispace1).parse(rem)?;  // Changed from parse_number to parse_reg
    let (rem, src) = parse_number_or_const(rem)?;
    Ok((rem, Cmd::MoveConst(dest, src)))
}

fn parse_move_addr_reg(input: &str) -> IResult<&str, Cmd<'_>> {
    let (rem, _) = tag_no_case("move").parse(input)?;
    let (rem, dest) = delimited(multispace1, parse_reg, multispace1).parse(rem)?;  // Changed from parse_number to parse_reg
    let (rem, src) = preceded(tag("&"), parse_reg).parse(rem)?;
    Ok((rem, Cmd::MoveAddrReg(dest, src)))
}

fn parse_store(input: &str) -> IResult<&str, Cmd<'_>> {
    let (rem, _) = tag_no_case("store").parse(input)?;
    let (rem, dest) = delimited(multispace1, parse_number, multispace1).parse(rem)?;
    let (rem, src) = parse_number_or_const(rem)?;
    Ok((rem, Cmd::StoreConst(dest, src)))
}

pub fn parse_move_id_address(input: &'_ str) -> IResult<&'_ str, Cmd<'_>> {
    let (rem, _) = tag_no_case("move").parse(input)?;
    let (rem, reg) = preceded(multispace1, parse_reg).parse(rem)?;
    let (rem, id) = preceded(multispace1, parse_identifier).parse(rem)?;
    Ok((rem, Cmd::MoveIdAddress(reg, id)))
}

pub fn parse_move_id_value(input: &'_ str) -> IResult<&'_ str, Cmd<'_>> {
    let (rem, _) = tag_no_case("move").parse(input)?;
    let (rem, reg) = preceded(multispace1, parse_reg).parse(rem)?;
    let (rem, val) = preceded(multispace1, parse_id_address_with_offset).parse(rem)?;
    match val {
        DataAddressOffset::Zero(id) => Ok((rem, Cmd::MoveIdValueConst(reg, id, 0))),
        DataAddressOffset::Const(id, n) => Ok((rem, Cmd::MoveIdValueConst(reg, id, n))),
        DataAddressOffset::Reg(id, r) => Ok((rem, Cmd::MoveIdValueReg(reg, id, r))),
    }
}

// -----------------           INT            -----------------

fn parse_int(input: &str) -> IResult<&str, Cmd<'_>> {
    let (rem, _) = tag_no_case("int")(input)?;
    // Parse two numbers separated by at least one space
    let (rem, (module, function)) = preceded(
        multispace1,
        pair(
            parse_number,
            preceded(multispace1, parse_number)
        )
    ).parse(rem)?;
    Ok((rem, Cmd::Int(module, function)))
}

// ----------------- Top-level command parser -----------------

pub fn parse_command(input: &str) -> IResult<&str, Cmd<'_>> {
    alt((
        alt((
            parse_move,
            parse_store,
            parse_push_const,
            parse_push_reg,
            parse_push_address,
            parse_pop_reg,
            parse_pop_address,
            parse_dup_const,
            parse_dup_reg,
            parse_dup,
            parse_shr_const,
            parse_shr_reg,
            parse_shl_const,
            parse_shl_reg,
            parse_call_const,
            parse_call_reg,
            parse_call_address,
            parse_add,
            parse_sub,
            parse_swap,
            parse_and,
        )),
        alt((
            parse_push_id_address,
            parse_push_id_value,
            parse_move_id_address,
            parse_move_id_value,
            parse_inc,
            parse_dec,
            parse_int,
            parse_drop,
            parse_or,
            parse_xor,
            parse_not,
            parse_jmp,
            parse_jnz,
            parse_jz,
            parse_jg,
            parse_jge,
            parse_jl,
            parse_jle,
            parse_ret,
            parse_term,
            parse_mul,
        )),
        alt((
            parse_div,
            parse_move_addr_reg,
        )),
    ))
    .parse(input)
}


// ----------------- Line / Token / Program -----------------

pub fn parse_token(input: &str) -> IResult<&str, Token<'_>> {
    alt((
        map(parse_label, Token::Label),
        map(parse_meta, Token::Meta),
        map(parse_command, Token::Command),
        map(parse_section, Token::Section),
        parse_data_def,
    ))
    .parse(input)
}

pub fn parse_line(input: &str) -> IResult<&str, Option<Token<'_>>> {
    let (rem, _) = space0(input)?; // leading spaces
    
    // Try token first
    if let Ok((rem, token)) = parse_token(rem) {
        // Allow optional trailing comment
        let (rem, _) = opt(preceded(space0, parse_comment)).parse(rem)?;
        let (rem, _) = space0(rem)?;
        return Ok((rem, Some(token)));
    }

    // Otherwise, maybe it's just a comment
    if let Ok((rem, _)) = parse_comment(rem) {
        return Ok((rem, None));
    }

    // Or empty line
    Ok((rem, None))
}

pub fn parse_program(input: &str) -> IResult<&str, Vec<Token<'_>>> {
    let (rem, lines) = separated_list0(line_ending, parse_line).parse(input)?;
    let tokens = lines.into_iter().flatten().collect();
    Ok((rem, tokens))
}
