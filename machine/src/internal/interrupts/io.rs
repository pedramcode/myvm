use crate::{errors::VMError, internal::machine::Machine};

pub fn print_function(machine: &mut Machine) -> Result<(), VMError> {
    let code = machine.memory.pop()?;
    print!("{}", char::from_u32(code).unwrap_or('☐'));
    Ok(())
}

pub fn print_counted_function(machine: &mut Machine) -> Result<(), VMError> {
    let number = machine.memory.pop()?;
    for _ in 0..number{
        let code = machine.memory.pop()?;
        print!("{}", char::from_u32(code).unwrap_or('☐'));
    }
    Ok(())
}

pub fn print_until_function(machine: &mut Machine) -> Result<(), VMError> {
    let chr = machine.memory.pop()?;
    loop{
        let code = machine.memory.pop()?;
        print!("{}", char::from_u32(code).unwrap_or('☐'));
        if code == chr {
            break;
        }
    }
    Ok(())
}