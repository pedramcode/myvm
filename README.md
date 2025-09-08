# Rust Virtual Machine / Assembler and Compiler

## Table of Contents
- [🏗 Architecture](#-architecture)
  - [What is a Stack-Based VM?](#what-is-a-stack-based-vm)
  - [Instruction Set: RISC Design](#instruction-set-risc-design)
  - [Main VM Components](#main-vm-components)
    - [1. Memory](#1-memory)
    - [2. Registers](#2-registers)
    - [3. Flags](#3-flags)
- [📝 VM Assembly Syntax and Commands](#-vm-assembly-syntax-and-commands)
  - [Sections](#sections)
  - [Data Definitions](#data-definitions)
  - [Numbers](#numbers)
  - [Memory Addresses](#memory-addresses)
  - [Metas](#metas)
  - [Comments](#comments)
  - [Registers](#registers)
  - [Opcodes (commands)](#opcodes-commands)
  - [Labels](#labels)
  - [Interrupts](#interrupts)
    - [Syntax](#syntax)
    - [Example: Module 0 = IO](#example-module-0--io)
    - [Example Usage](#example-usage)
  - [Hello World!](#hello-world)
- [💻 Command-Line Interface (CLI)](#-command-line-interface-cli)
  - [Installation](#installation)
  - [Commands](#commands)
    - [1. Compile](#1-compile)
    - [2. Exec](#2-exec)
- [🛠️ Developer TODO / Roadmap](#️-developer-todo--roadmap)

---

## 🏗 Architecture

This project is a **stack-based virtual machine (VM)** with a **separate call stack** for managing function calls.

### What is a Stack-Based VM?

A stack-based VM executes instructions primarily by manipulating a **stack data structure**.  
Instead of operating directly on registers, most instructions **push values onto the stack** and **pop them when needed**.  

For example, consider the following program:

```assembly
PUSH 10
PUSH 20
ADD
```

- `PUSH 10` pushes the value `10` onto the stack.  
- `PUSH 20` pushes the value `20`.  
- `ADD` pops the top two values (`20` and `10`), adds them, and pushes the result (`30`) back onto the stack.  

This simple model makes it easier to implement compilers and interpreters since there is no need to manage complex register allocations.

---

### Instruction Set: RISC Design

The VM uses a **Reduced Instruction Set Computing (RISC)** design.  
RISC focuses on having a **small, well-defined set of simple instructions**, each performing a single operation efficiently.  

Each instruction in the VM is represented as an **Opcode**.

#### What is an Opcode?

An **Opcode (Operation Code)** is the numeric or symbolic representation of an instruction that tells the VM what operation to perform.  
Opcodes may also take parameters (operands).  

For example:

```assembly
PUSH 42 ; Opcode: PUSH, Operand: 42
PUSH 32 ; Opcode: PUSH, Operand: 32
ADD ; Opcode: ADD (no operands)
```

Some Opcodes have **variants**.  
For example, the **Jump instruction** has multiple variants that depend on conditions:  
- `jmp` → Unconditional jump  
- `jnz` → Jump if not zero  
- `jz`  → Jump if zero  
- `jg`  → Jump if greater  
- `jl`  → Jump if less  
- `jge` → Jump if greater or equal  
- `jle` → Jump if less or equal  

---

### Main VM Components

The VM core is composed of three primary components:

#### 1. Memory
- The **main storage** that contains:
  - Uploaded program code  
  - Variables and data  
  - Execution stack  
- The stack is located at the **end of memory** and grows **backwards** to avoid collisions with program data.  
- If the stack grows beyond its capacity, a **stack overflow** occurs.  
- Each memory cell is **32-bit wide**.

#### 2. Registers
- **Registers** are small, fast storage locations inside the VM that hold data during execution.  
- This VM has:
  - **8 general-purpose registers**: `r0` ... `r7`  
  - **Program Counter (PC)** register: stores the address of the **next instruction** to execute  

Registers allow the VM to perform operations more quickly than relying solely on memory.

#### 3. Flags
- **Flags** are single-bit indicators that store the outcome of operations.  
- They are used by conditional instructions (like jumps) to determine execution flow.  
- This VM defines the following flags:
  - **Zero (Z)** → Set when the result of an operation is `0`  
  - **Overflow (O)** → Set when an arithmetic operation produces a value outside the representable range  
  - **Negative (N)** → Set when the result of an operation is negative  
  - **Carry (C)** → Set when an arithmetic operation generates a carry/borrow bit beyond the register size  

---

## 📝 VM Assembly Syntax and Commands

The VM uses a **custom assembly language** for programming, supporting constants, memory addresses, registers, labels, macros, and interrupts.

---

### Sections

Programs must be organized into sections using `[section_name]` tags. There are two mandatory sections:

- `[text]` - Contains executable code
- `[data]` - Contains data definitions and constants

Writing code in the `[data]` section or defining data in the `[text]` section will result in an error.

Example:
```asm
[data]
$name b "Pedram"
$scores w 0x2301 0x1212 19 20

[text]
PUSH $name
INT 0 0
TERM
```

### Data Definitions

Data can be defined in the `[data]` section using the following syntax:

```
$[identifier] [data type] [... data separated with space ...]
```

Supported data types:
- `b` - Byte: stores data in 1-byte arrays
- `w` - Word: stores data in 2-byte arrays  
- `dw` - Double Word: stores data in 4-byte arrays

Examples:
```asm
[data]
$name b "Pedram"
$scores w 0x2301 0x1212 19 20
$data dw 0xaaaaaaaa 0xbbbbbbbb
```

**Note on memory storage**: Each memory cell is 32-bit (4 bytes). When using `b` or `w` types, data is packed into memory cells at the bit level. For example:
- `b 0xaa 0xbb 0xcc` stores as: `0xaabbcc00`
- `w 0xaabb 0xcc` stores as: `0xaabbcc00`

### Enhanced Memory Access Syntax

The following syntax is supported for accessing data:

```asm
; Push operations
push $name        ; pushes $name address to stack
push [$name]      ; pushes $name value to stack  
push [$name + 4]  ; pushes $name value with offset to stack
push [$name + r0] ; pushes $name value with offset stored in register to stack

; Move operations
move r0 $name        ; move address of $name into register r0
move r0 [$name]      ; move value of $name into register r0
move r0 [$name + 2]  ; move value of $name with offset into register r0
move r0 [$name + r1] ; move value of $name with offset stored in r1 into register r0
move r0 &r1          ; move value of data that its address stored in register r1 to r0
```

### Numbers
- **Decimal:** `42`  
- **Hexadecimal:** `0x2A`  
- **Binary:** `0b101010`  

### Memory Addresses
- Use `&[number]` to reference memory addresses:  
  - `&0x1010`, `&321`, `&0b101010`  
- They point to the value stored at the specified memory address.

### Metas
Metas are special commands that control how the VM executes or prepares code.  
They **start with `@`** and may have parameters:

- `@ORG x` → Sets the origin address in memory for the following code.  
- `@INCLUDE "./file.asm"` → Includes another assembly file into the current file.  

### Comments
- Start with `;`  
```asm
; This is a comment
```

### Registers

The VM supports registers:

* General-purpose: r0, r1, r2, r3, r4, r5, r6, r7

* Special-purpose: pc (Program Counter)

### Opcodes (commands)

| Command         | Parameters         | Description                                               |
| --------------- | ------------------ | --------------------------------------------------------- |
| `PUSH 10`       | number             | Pushes a constant number to the stack                     |
| `PUSH r0`       | register           | Pushes value of a register onto the stack                 |
| `PUSH &10`      | address            | Pushes value from memory address onto the stack           |
| `PUSH $name`    | data label         | Pushes address of data label to stack                     |
| `PUSH [$name]`  | data label         | Pushes value of data label to stack                       |
| `POP r1`        | register           | Pops value from stack into a register                     |
| `POP &32`       | address            | Pops value from stack into a memory address               |
| `ADD`           | -                  | Pops two values, adds them, pushes result                 |
| `SUB`           | -                  | Pops two values, subtracts, pushes result                 |
| `MUL`           | -                  | Pops two values, multiplies, pushes result                |
| `DIV`           | -                  | Pops two values, divides, pushes result, puts reminder in R3.                   |
| `DROP`          | -                  | Drops the top item of the stack                           |
| `SWAP`          | -                  | Swaps top two items on stack                              |
| `INC r0 `       | register           | increase register by 1                                    |
| `DEC r0 `       | register           | decrease register by 1                                    |
| `MOVE r0 10`    | register, value    | Moves constant into register                              |
| `MOVE r0 r1`    | register, register | Moves value from one register to another                  |
| `MOVE r0 &12`   | register, address  | Moves value from memory address to register               |
| `MOVE r0 $name` | register, data     | Moves address of data label to register                   |
| `MOVE r0 [$name]` | register, data   | Moves value of data label to register                     |
| `MOVE r0 [$name + 2]` | register, data | Moves value of data label with offset to register       |
| `MOVE r0 [$name + r1]` | register, data | Moves value of data label with register offset to register |
| `MOVE r0 &r1`   | register, register | Moves value from address in register to register          |
| `STORE 1010 32` | address, value     | Stores constant into memory                               |
| `STORE 1010 r3` | address, register  | Stores register value into memory                         |
| `JMP .label`    | label              | Unconditional jump                                        |
| `JNZ .label`    | label              | Jump if not zero                                          |
| `JZ .label`     | label              | Jump if zero                                              |
| `JG .label`     | label              | Jump if greater                                           |
| `JGE .label`    | label              | Jump if greater or equal                                  |
| `JL .label`     | label              | Jump if less                                              |
| `JLE .label`    | label              | Jump if less or equal                                     |
| `AND`           | -                  | Pops two values, bitwise AND, pushes result               |
| `OR`            | -                  | Pops two values, bitwise OR, pushes result                |
| `XOR`           | -                  | Pops two values, bitwise XOR, pushes result                |
| `NOT`           | -                  | Pops one value, bitwise NOT, pushes result                |
| `SHR 10`        | value              | Pops value, shifts right by constant, pushes result       |
| `SHR r3`        | value              | Pops value, shifts right by register value, pushes result |
| `SHL 10`        | value              | Pops value, shifts left by constant, pushes result        |
| `SHL r3`        | value              | Pops value, shifts left by register value, pushes result  |
| `CALL 32`       | address            | Calls procedure at address                                |
| `CALL .label`   | label              | Calls procedure by label                                  |
| `CALL r0`       | register           | Calls procedure at address in register                    |
| `CALL &323`     | address            | Calls procedure at memory address                         |
| `RET`           | -                  | Returns from procedure                                    |
| `DUP`           | -                  | Duplicates top stack item                                 |
| `DUP 10`        | number             | Duplicates top stack item `n` times                       |
| `DUP r3`        | register           | Duplicates top stack item `r3` times                      |
| `INT 0 2`       | module, function   | Calls an interrupt (see below)                            |
| `TERM`          | -                  | Terminates code execution                                 |

### Labels

* Labels mark code positions and procedures for jumps and calls.

* Start with . and contain only alphanumeric characters (no spaces):

```asm
.sayhello
    ; code here
RET
```

### Interrupts

An interrupt is a pre-defined function in a VM module that performs operations outside normal instructions.
Interrupts allow modular, system-level functionality like I/O.

#### Syntax

```asm
INT module_number function_number
```

#### Example: Module 0 = IO

| Function | Description                                                                    |
| -------- | ------------------------------------------------------------------------------ |
| 0        | Pops top of stack and prints it                                                |
| 1        | Pops number `n` from stack, then pops `n` items and prints them                |
| 2        | Pops a stop value, then continuously pops and prints until reaching stop value |

Interrupts provide a bridge between VM code and system-level functions without complicating the instruction set.

---

#### Example Usage

```asm
@ORG 0x100

CALL .f
TERM

.f
PUSH 13
INT 0 0
RET
```

## Hello World! (The hard way!)

```asm
@org 10

PUSH 10
PUSH 13
PUSH 33
PUSH 100
PUSH 108
PUSH 114
PUSH 111
PUSH 87
PUSH 32
PUSH 111
PUSH 108
PUSH 114
PUSH 111
PUSH 87
PUSH 32
PUSH 111
PUSH 108
PUSH 108
PUSH 101
PUSH 72
PUSH 10

INT 0 2

TERM
```

## Hello World! (Using data section)

```asm
[data]
$hello b "Hello World!" 0  ; Null-terminated string
$newline b 10 13           ; Newline characters

[text]
@org 0x100

PUSH $hello
INT 0 1        ; Print string

PUSH 2
PUSH $newline  
INT 0 1        ; Print newline

TERM
```

## Hexdump

Hexdump of above example after execution:

```
00000000: 00000000 00000000 00000000 00000000 |................|
00000010: 00000000 00000000 00000000 00000000 |................|
00000020: 00000000 00000000 F001A001 0000000A |................|
00000030: F001A001 0000000D F001A001 00000021 |...............!|
00000040: F001A001 00000064 F001A001 0000006C |.......d.......l|
00000050: F001A001 00000072 F001A001 0000006F |.......r.......o|
00000060: F001A001 00000057 F001A001 00000020 |.......W....... |
00000070: F001A001 0000006F F001A001 0000006C |.......o.......l|
00000080: F001A001 00000072 F001A001 0000006F |.......r.......o|
00000090: F001A001 00000057 F001A001 00000020 |.......W....... |
000000A0: F001A001 0000006F F001A001 0000006C |.......o.......l|
000000B0: F001A001 0000006C F001A001 00000065 |.......l.......e|
000000C0: F001A001 00000048 F001A001 0000000A |.......H........|
000000D0: F0120000 00000000 00000002 FFFF0000 |................|
000000E0: 00000000 00000000 00000000 00000000 |................|
000000F0: 00000000 00000000 00000000 00000000 |................|
00000100: 00000000 00000000 00000000 00000000 |................|
00000110: 00000000 00000000 00000000 00000000 |................|
00000120: 00000000 00000000 00000000 00000000 |................|
00000130: 00000000 00000000 00000000 00000000 |................|
00000140: 00000000 00000000 00000000 00000000 |................|
00000150: 00000000 00000000 00000000 00000000 |................|
00000160: 00000000 00000000 00000000 00000000 |................|
00000170: 00000000 00000000 00000000 00000000 |................|
00000180: 00000000 00000000 00000000 00000000 |................|
00000190: 00000000 00000000 00000000 00000000 |................|
000001A0: 00000000 00000000 00000000 00000000 |................|
000001B0: 00000000 00000000 00000000 00000000 |................|
000001C0: 00000000 00000000 00000000 00000000 |................|
000001D0: 00000000 00000000 00000000 00000000 |................|
000001E0: 00000000 00000000 00000000 00000000 |................|
000001F0: 00000000 00000000 00000000 00000000 |................|
```

## Reverse print number

```asm
@org 0

push 123
call .split
TERM

.split
    push 10
    swap
    div
    push r3
    call .printdigit
    dup
    push 10
    swap
    sub
    drop
    jge .split
    call .printdigit
    call .newline
    ret

; push digit as parameter
.printdigit
    push 48
    add
    int 0 0
    ret

.newline
    push 10
    push 13
    push 10
    int 0 2
    ret
```

### Hexdump

Hex dump of above example after running

```
00000000: F001A001 0001E240 F00FA015 00000005 |.......@........|
00000010: FFFF0000 F001A001 0000000A F0050000 |................|
00000020: F0150000 F001A002 00000003 F00FA015 |................|
00000030: 0000001A F0110000 F001A001 0000000A |................|
00000040: F0050000 F0040000 F0130000 F008A00E |................|
00000050: 00000005 F00FA015 0000001A F00FA015 |................|
00000060: 00000021 F0100000 F001A001 00000030 |...!...........0|
00000070: F0030000 F0120000 00000000 00000000 |................|
00000080: F0100000 F001A001 0000000A F001A001 |................|
00000090: 0000000D F001A001 0000000A F0120000 |................|
000000A0: 00000000 00000002 F0100000 00000000 |................|
000000B0: 00000000 00000000 00000000 00000000 |................|
000000C0: 00000000 00000000 00000000 00000000 |................|
000000D0: 00000000 00000000 00000000 00000000 |................|
000000E0: 00000000 00000000 00000000 00000000 |................|
000000F0: 00000000 00000000 00000000 00000000 |................|
00000100: 00000000 00000000 00000000 00000000 |................|
00000110: 00000000 00000000 00000000 00000000 |................|
00000120: 00000000 00000000 00000000 00000000 |................|
00000130: 00000000 00000000 00000000 00000000 |................|
00000140: 00000000 00000000 00000000 00000000 |................|
00000150: 00000000 00000000 00000000 00000000 |................|
00000160: 00000000 00000000 00000000 00000000 |................|
00000170: 00000000 00000000 00000000 00000000 |................|
00000180: 00000000 00000000 00000000 00000000 |................|
00000190: 00000000 00000000 00000000 00000000 |................|
000001A0: 00000000 00000000 00000000 00000000 |................|
000001B0: 00000000 00000000 00000000 00000000 |................|
000001C0: 00000000 00000000 00000000 00000000 |................|
000001D0: 00000000 00000000 00000000 00000000 |................|
000001E0: 00000000 00000000 00000000 00000000 |................|
000001F0: 00000000 00000000 00000000 00000000 |................|
```

## 💻 Command-Line Interface (CLI)

This project includes a **CLI tool** built with [Rust Clap](https://crates.io/crates/clap) to **compile** assembly code into binary and **execute** binary files on the VM.

The CLI provides two main commands: `compile` and `exec`.

---

### Installation

After cloning the repository, build the CLI with Cargo:

```bash
cargo build --release
```

The compiled executable will be in target/release/. You can run it directly:

```bash
./myvm [COMMAND] [OPTIONS]
```

### Commands

#### 1. Compile

Compiles an assembly source file into a binary that can be executed on the VM.

Usage:

```bash
./myvm compile -p source.asm -o output.bin
```

##### Options

| Option         | Description                      |
| -------------- | -------------------------------- |
| `-p, --path`   | Path to the source assembly file |
| `-o, --output` | Path to the output binary file   |

How it works:

* Reads the assembly file at the given path
* Compiles it using the VM compiler
* Writes a binary file with:
  * Header: origin address (u32)
  * Body: compiled bytecode (u32 per instruction)

#### 2. Exec

Executes a compiled binary file on the VM.

Usage:

```bash
./myvm exec -p output.bin --cells 2048 --stack 256
```

Options:

| Option        | Description                             | Default |
| ------------- | --------------------------------------- | ------- |
| `-p, --path`  | Path to the binary file                 | —       |
| `-c, --cells` | Number of memory cells in the VM        | 2048    |
| `-s, --stack` | Number of cells allocated for the stack | 256     |

How it works:

* Reads the binary file
* Parses the origin address from the header
* Loads the bytecode into VM memory
* Configures the VM memory and stack size
* Sets the program counter to the origin
* Executes instructions sequentially until `TERM` or an error occurs

## 🛠️ Developer TODO / Roadmap

This project is a hobby but fully open for contributions. Here are some key areas to work on:

- **Error Handling**  
  - Detect and handle stack overflows, invalid memory access, and illegal opcodes  
  - Provide descriptive runtime error messages

- **Heap and Memory Management**  
  - Implement dynamic memory allocation  
  - Add garbage collection or memory reuse strategies

- **IO Interrupt Module**  
  - Expand module 0 functionality  
  - Support reading input, printing formatted output, and file operations

- **Network Interrupt Module**  
  - Add network communication interrupts for sending/receiving data  
  - Enable TCP/UDP support for simple network programs

- **More unit tests**
  - Write unit test for all modules and functions

- **Code docs**
  - Write better code docs

- **Create a REPL**
  - REPL in CLI