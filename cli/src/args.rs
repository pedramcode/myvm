use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// compile code and generate binary file
    Compile {
        /// path of source file
        #[arg(short, long)]
        path: String,
        /// path of output file
        #[arg(short, long)]
        output: String,
    },
    /// execute binary code
    Exec {
        /// path of binary file
        #[arg(short, long)]
        path: String,
        /// memory cells
        #[arg(short, long, default_value_t = 2048)]
        cells: u32,
        /// stack cells
        #[arg(short, long, default_value_t = 256)]
        stack: u32,
        /// dump memory to stdout after execution
        #[arg(short, long)]
        dump: bool
    },
}