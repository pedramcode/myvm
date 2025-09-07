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
PUSH 42 # Opcode: PUSH, Operand: 42
ADD # Opcode: ADD (no operands)
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
