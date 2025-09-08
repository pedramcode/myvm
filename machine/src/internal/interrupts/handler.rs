use crate::{errors::VMError, internal::{interrupts::io::{print_counted_function, print_data_string_function, print_function, print_number_function, print_until_function}, machine::Machine}};

const IOMODULE: u32 = 0x0000_0000;

const PRINT_FUNC: u32 = 0x0000_0000;
const PRINT_COUNTED_FUNC: u32 = 0x0000_0001;
const PRINT_UNTIL_FUNC: u32 = 0x0000_0002;
const PRINT_DATA_STRING_FUNC: u32 = 0x0000_0003;
const PRINT_NUMBER_FUNC: u32 = 0x0000_0004;

pub fn interrupt_handler(machine: &mut Machine, module: u32, function: u32) -> Result<(), VMError> {
    match module {
        IOMODULE => {
            match function {
                PRINT_FUNC => {
                    print_function(machine)?;
                },
                PRINT_COUNTED_FUNC => {
                    print_counted_function(machine)?;
                },
                PRINT_UNTIL_FUNC => {
                    print_until_function(machine)?;
                },
                PRINT_DATA_STRING_FUNC => {
                    print_data_string_function(machine)?;
                },
                PRINT_NUMBER_FUNC => {
                    print_number_function(machine)?;
                },
                _ => {
                    return Err(VMError::InvalidFunction);
                }
            }
        },
        _ => {
            return Err(VMError::InvalidModule);
        }
    }
    Ok(())
}
