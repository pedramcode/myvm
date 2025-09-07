# Rust virtual machine / assembler and compiler

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
| `POP r1`        | register           | Pops value from stack into a register                     |
| `POP &32`       | address            | Pops value from stack into a memory address               |
| `ADD`           | -                  | Pops two values, adds them, pushes result                 |
| `SUB`           | -                  | Pops two values, subtracts, pushes result                 |
| `MUL`           | -                  | Pops two values, multiplies, pushes result                |
| `DIV`           | -                  | Pops two values, divides, pushes result                   |
| `DROP`          | -                  | Drops the top item of the stack                           |
| `SWAP`          | -                  | Swaps top two items on stack                              |
| `MOVE r0 10`    | register, value    | Moves constant into register                              |
| `MOVE r0 r1`    | register, register | Moves value from one register to another                  |
| `MOVE r1 &12`   | register, address  | Moves value from memory address to register               |
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
| `XOR`           | -                  | Pops two values, bitwise XOR, pushes result               |
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
