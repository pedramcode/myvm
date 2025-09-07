#[cfg(test)]
pub mod tests {
    use machine::internal::machine::{Machine, MachineOptions};

    #[test]
    pub fn create() {
        Machine::new(MachineOptions{memory_cells: 10, memory_stack_size: 5}).unwrap();
    }

    #[test]
    pub fn load_data() {
        let data = [2,3,4,5];
        let mut machine = Machine::new(MachineOptions{memory_cells: 10, memory_stack_size: 5}).unwrap();
        machine.load_data(1, &data).unwrap();
    }

    #[test]
    pub fn execute() {
        let code = [
            0xf001a001, 5, // PUSH 5
            0xf001a001, 4, // PUSH 4
            0xf0050000, // SWAP
            0xf0030000, // ADD
            0xf002a004, 0, // POP r0
            0xf001a001, 10, // PUSH 10
            0xf001a002, 0, // PUSH r0
            0xf0040000, // SUB
            0xf002a004, 1, // POP r1
            0xf006a006, 4, 10, // MOVE r4 10
            0xf006a007, 5, 1, // MOVE r5 r1
            0xf001a002, 4, // PUSH r4
            0xf001a002, 5, // PUSH r5
            0xf0030000, // ADD
            0xf002a004, 7, // POP r7
            0xf007a00a, 50, 7, // STORE 50 r7
            0xf006a008, 0, 50, // MOVE r0 &50
            0xf0080000, 56, // JMP 56
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0xffff0000, // TERM
        ];
        let mut machine = Machine::new(MachineOptions{memory_cells: 1024, memory_stack_size: 256}).unwrap();
        machine.load_data(10, &code).unwrap();
        machine.set_origin(10);
        machine.execute().unwrap();
        assert_eq!(machine.read_register(0).unwrap(), 11);
    }

    #[test]
    pub fn execute_interrupt() {
        let code = [
            0xf001a001, 13, // PUSH 13
            0xf001a001, 79, // PUSH 79
            0xf001a001, 76, // PUSH 79
            0xf0110000, // DUP
            0xf001a001, 69, // PUSH 69
            0xf001a001, 72, // PUSH 72

            0xf0120000, 0, 0, // INT 0 0
            0xf0120000, 0, 0, // INT 0 0
            0xf0120000, 0, 0, // INT 0 0
            0xf0120000, 0, 0, // INT 0 0
            0xf0120000, 0, 0, // INT 0 0
            0xf0120000, 0, 0, // INT 0 0

            0xffff0000, // TERM
        ];
        let mut machine = Machine::new(MachineOptions{memory_cells: 1024, memory_stack_size: 256}).unwrap();
        machine.load_data(10, &code).unwrap();
        machine.set_origin(10);
        machine.execute().unwrap();
    }
}