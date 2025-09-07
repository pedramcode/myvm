#[cfg(test)]
pub mod tests {
    use assembler::compiler::compile;
    use machine::internal::machine::Machine;

    #[test]
    pub fn compile_code() {
        let code = r#"
        @ORG 32
        
        CALL .print
        CALL .print
        CALL .print
        CALL .print
        CALL .print
        CALL .print
        PUSH 10
        PUSH 3
        MUL
        POP r0
        TERM

        ; Print function
        .print
            PUSH 10
            PUSH 13
            PUSH 69
            DUP
            PUSH 71
            PUSH 72
            PUSH 10

            INT 0 2
            RET
        "#;
        let res = compile(code.to_string());
        let mut machine = Machine::new(machine::internal::machine::MachineOptions { memory_cells: 2048, memory_stack_size: 1024 }).unwrap();
        machine.load_data(res.origin, &res.binary).unwrap();
        machine.set_origin(res.origin);
        machine.execute().unwrap();
        assert_eq!(machine.read_register(0).unwrap(), 30);
    }
}
