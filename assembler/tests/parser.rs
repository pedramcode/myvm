#[cfg(test)]
pub mod tests {
    use assembler::{parser::{parse_const_value, parse_meta, parse_number, parse_program, parse_str}};

    #[test]
    pub fn number_hex(){
        let data = "0xa";
        let res = parse_number(data).unwrap();
        assert_eq!(res.1, 10);
    }

    #[test]
    pub fn number_bin(){
        let data = "0b1010";
        let res = parse_number(data).unwrap();
        assert_eq!(res.1, 10);
    }

    #[test]
    pub fn number_dec(){
        let data = "10";
        let res = parse_number(data).unwrap();
        assert_eq!(res.1, 10);
    }

    #[test]
    pub fn parse_string(){
        let data = r#""Hello World\n""#;
        let res = parse_str(data).unwrap();
        assert_eq!(res.1.to_string(), "Hello World\\n".to_string());
    }

    #[test]
    pub fn parse_meta_object() {
        let data = "@org 0xa";
        let (_, org) = parse_meta(data).unwrap();
        match org {
            assembler::tokens::MetaType::Org(val) => assert_eq!(val, 10),
            _ => panic!()
        }

        let data = r#"@include "Hello World""#;
        let (_, org) = parse_meta(data).unwrap();
        match org {
            assembler::tokens::MetaType::Include(val) => assert_eq!(val, "Hello World"),
            _ => panic!()
        }
    }

    #[test]
    pub fn parse_const_value_object() {
        let data = "123";
        let (_, res) = parse_const_value(data).unwrap();
        match res {
            assembler::tokens::ConstValue::Number(n) => assert_eq!(n, 123),
            _ => panic!(),
        }

        let data = ".label";
        let (_, res) = parse_const_value(data).unwrap();
        match res {
            assembler::tokens::ConstValue::Label(n) => assert_eq!(n, "label"),
            _ => panic!(),
        }
    }

    #[test]
    pub fn parse_program_object() {
        let code = r#"
        @org    0x10 
        PUSH 10
        PUSH 32
        PUSH 0x23
        POP     r0
        ADD
        INT 32 0b10101
        TERM
        "#;
        let _res = parse_program(code).unwrap();
    }
}