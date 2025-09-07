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
    pub fn div() {
        let code = [
            0xf001a001, 2, // PUSH 2
            0xf001a001, 10, // PUSH 10
            0xf0150000, // DIV
            0xf002a004, 0, // POP r0
            0xf006a007, 1, 3, // MOVE r1 r3
            0xffff0000, // TERM
        ];
        let mut machine = Machine::new(MachineOptions{memory_cells: 1024, memory_stack_size: 256}).unwrap();
        machine.load_data(10, &code).unwrap();
        machine.set_origin(10);
        machine.execute().unwrap();
        assert_eq!(machine.read_register(0).unwrap(), 5);
        assert_eq!(machine.read_register(1).unwrap(), 0);
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