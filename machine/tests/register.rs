#[cfg(test)]
pub mod tests {
    use machine::internal::register::Register;


    #[test]
    pub fn set() {
        let mut reg = Register::new();
        reg.set(0, 100).unwrap();
    }

    #[test]
    #[should_panic]
    pub fn set_invalid() {
        let mut reg = Register::new();
        reg.set(1990, 1).unwrap();
    }

    #[test]
    pub fn get() {
        let reg = Register::new();
        reg.get(1).unwrap();
    }

    #[test]
    #[should_panic]
    pub fn get_invalid() {
        let reg = Register::new();
        reg.get(1990).unwrap();
    }
}