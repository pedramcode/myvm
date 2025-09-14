use std::io::Write;
use std::io::{Read};
use std::mem;

use assembler::compiler::compile;
use clap::Parser;
use machine::internal::machine::{Machine, MachineOptions};

use crate::args::{Args, Commands};

pub mod args;

fn main() {
    let cli = Args::parse();

    match &cli.command {
        Some(Commands::Compile { path, output }) => {
            let code = std::fs::read_to_string(path.as_str()).expect("unable to open source file");
            let mut output = std::fs::File::create(output.as_str()).expect("unable to create output file");
            
            let result = compile(code);
            output.write_all(&result.header.origin.to_le_bytes()).expect("unable to write in output file");
            output.write_all(&result.header.start.to_le_bytes()).expect("unable to write in output file");
            for &num in &result.binary {
                output.write_all(&num.to_le_bytes()).expect("unable to write in output file");
            }
        },
        Some(Commands::Exec { path, cells, stack, dump }) => {
            let mut file = std::fs::File::open(path).expect("unable to open binary file");
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).expect("unable to read binary content");
            if buffer.len() < mem::size_of::<u32>() {
                panic!("file is too short to contain header");
            }
            let origin = u32::from_le_bytes(buffer[..4].try_into().unwrap());
            let start = u32::from_le_bytes(buffer[4..8].try_into().unwrap());
            let data: Vec<u32> = buffer[8..]
                .chunks_exact(4)
                .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
                .collect();
            
            let mut machine = Machine::new(MachineOptions{
                memory_cells: *cells,
                memory_stack_size: *stack,
            }).unwrap();
            machine.load_data(origin, &data).unwrap();
            machine.set_start(start);
            machine.execute().unwrap();
            if *dump {
                println!("{}", machine.memory);
            }
        },
        None => {},
    }
}
