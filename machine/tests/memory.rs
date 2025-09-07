#[cfg(test)]
pub mod tests {
    use machine::internal::memory::Memory;

    #[test]
    pub fn create() {
        Memory::new(10, 5).unwrap();
    }

    #[test]
    #[should_panic]
    pub fn create_invalid() {
        Memory::new(5, 10).unwrap();
    }

    #[test]
    pub fn push() {
        let mut mem = Memory::new(10, 5).unwrap();
        mem.push(1).unwrap();
        mem.push(2).unwrap();
        mem.push(3).unwrap();
        mem.push(4).unwrap();
        mem.push(5).unwrap();
    }

    #[test]
    #[should_panic]
    pub fn stackoverflow() {
        let mut mem = Memory::new(10, 5).unwrap();
        mem.push(1).unwrap();
        mem.push(2).unwrap();
        mem.push(3).unwrap();
        mem.push(4).unwrap();
        mem.push(5).unwrap();
        mem.push(6).unwrap();
    }

    #[test]
    pub fn pop() {
        let mut mem = Memory::new(10, 5).unwrap();
        mem.push(1).unwrap();
        mem.push(2).unwrap();
        mem.push(3).unwrap();
        mem.push(4).unwrap();
        mem.push(5).unwrap();
        assert_eq!(mem.pop().unwrap(), 5);
        assert_eq!(mem.pop().unwrap(), 4);
        assert_eq!(mem.pop().unwrap(), 3);
        assert_eq!(mem.pop().unwrap(), 2);
        assert_eq!(mem.pop().unwrap(), 1);
    }

    #[test]
    #[should_panic]
    pub fn pop_empry(){
        let mut mem = Memory::new(10, 5).unwrap();
        mem.pop().unwrap();
    }

    #[test]
    pub fn write() {
        let data = [1,2,3,4,5];
        let mut mem = Memory::new(10, 5).unwrap();
        mem.write(0, &data).unwrap();
    }

    #[test]
    #[should_panic]
    pub fn write_overlap() {
        let data = [1,2,3,4,5,6];
        let mut mem = Memory::new(10, 5).unwrap();
        mem.write(0, &data).unwrap();
    }

    #[test]
    pub fn read() {
        let data = [1,2,3,4,5];
        let mut mem = Memory::new(10, 5).unwrap();
        mem.write(0, &data).unwrap();
        assert_eq!(mem.read(0).unwrap(), 1);
        assert_eq!(mem.read(4).unwrap(), 5);
    }

    #[test]
    #[should_panic]
    pub fn read_invalid() {
        let mem = Memory::new(10, 5).unwrap();
        mem.read(20).unwrap();
    }
}