# MyVM Tutorial

*A Stack-Based Virtual Machine and Assembly Language Guide*

---

## 1. Introduction

**MyVM** is a custom **stack-based virtual machine** designed with simplicity and flexibility in mind.
It operates on a **32-bit architecture**, where each memory cell and register is `u32` (4 bytes).

Key features include:

* Stack-based execution model (stack grows backwards in memory).
* 9 registers (`r0`–`r7` and `pc`).
* Flags for conditional execution.
* A separate call stack supporting both **regular calls** and **safe calls**.
* Rich instruction set with arithmetic, logic, control flow, and memory operations.
* Custom assembly language for programming MyVM.

This tutorial explains MyVM step by step with examples.

---

## 2. Architecture Overview

### 2.1 Memory

* Memory is an array of `u32` values.
* The **stack** is located at the end of memory and **grows backwards**.
* Stack operations must remain within the defined stack size:

  * **Stack Overflow**: pushing beyond limit.
  * **Stack Underflow**: popping from empty stack.

### 2.2 Registers

MyVM provides **9 registers**, each 32-bit wide:

| Register  | Purpose                                      | Address |
| --------- | -------------------------------------------- | ------- |
| `r0`      | General purpose                              | 0       |
| `r1`      | General purpose                              | 1       |
| `r2`      | General purpose                              | 2       |
| `r3`      | General purpose (used as remainder in `DIV`) | 3       |
| `r4`–`r7` | General purpose                              | 4–7     |
| `pc`      | Program counter                              | 100     |

### 2.3 Flags

Execution updates **flags** that affect conditional jumps:

* `zero` → result equals 0.
* `negative` → result is negative (for signed interpretation).
* `overflow` → arithmetic overflow occurred.
* `carry` → carry/borrow occurred in arithmetic.

### 2.4 Call Stack

MyVM maintains a separate **call stack** for function calls:

* **Regular Call**: pushes only `PC`.
* **Safe Call**: pushes `PC`, registers, and flags (restores state on return).

---

## 3. Instruction Format

Each instruction is **4 bytes**:

* **High 2 bytes** → Opcode
* **Low 2 bytes** → Variant

Example:

```
0xf018a01f  →  SafeCall with constant address
```

---

## 4. Opcodes

The instruction set supports **arithmetic, logic, memory, flow control, calls, and interrupts**.

### 4.1 Stack Operations

* `PUSH` (constant, register, address, offset).
* `POP` (into register or memory).
* `DROP` → discard top of stack.
* `DUP` → duplicate last item (supports constant/repetition).
* `SWAP` → swap top two items.

### Example

```asm
PUSH 10
PUSH 20
ADD     ; stack = [30]
```

---

### 4.2 Arithmetic

* `ADD` → `(a + b)`.
* `SUB` → `(b - a)` (remember pop order).
* `MUL` → multiplication.
* `DIV` → division (`result` on stack, `remainder` in `r3`).
* `INC rX` → increment register.
* `DEC rX` → decrement register.

---

### 4.3 Logic & Bitwise

* `AND`, `OR`, `XOR`, `NOT`.
* `SHR`/`SHL` (by const or reg).

---

### 4.4 Data Movement

* `MOVE` → move values into registers.
* `STORE` → write constants or registers into memory.

---

### 4.5 Control Flow

* `JMP` → unconditional.
* `JNZ`, `JZ`, `JG`, `JGE`, `JL`, `JLE`.
* `CALL`, `SAFECALL`, `RET`.

---

### 4.6 Interrupts

* `INT module function` → call host-provided system function.
* **Module 0 = I/O Module**, with 5 functions:

  1. Print character (`pop → ASCII`).
  2. Print character N times.
  3. Print until sentinel value.
  4. Print zero-terminated string.
  5. Print number (decimal digits).

---

## 5. Assembly Language

### 5.1 Basics

* Case-insensitive.
* Comments begin with `;`.
* Numbers: decimal (`10`), hex (`0xFF`), binary (`0b1010`).

---

### 5.2 Meta Directives

* `@ORIGIN n` → set code origin in memory.

---

### 5.3 Sections

* `[text]` → program instructions.
* `[data]` → constant/data definitions.

---

### 5.4 Labels

* Defined with `.labelname`.
* Used in jumps and calls.

---

### 5.5 Data Definitions

* `$name b/w/dw values`.
* Compact packing for `b` and `w`.

Examples:

```asm
$name b "ABC"
$scores w 0x10 0x20 0x30
```

---

## 6. Example Programs

### 6.1 Factorial

```asm
@ORIGIN 0
[data]
$num    dw 0 1 2 3 4 5 6 7
$title  dw "Factorial result: " 13 10 0
$msg    dw "! = " 0
$newline dw 10 13 0

[text]
.start
    push $title
    int 0 3

    move r0 7
.for
    push [$num + r0]
    safecall .factorial
    dec r0
    jnz .for
term

.factorial
    dup 2
    pop r5
    pop r1
    move r0 1 ; acc
.loop
    push r0
    mul
    pop r0

    dec r1
    push r1
    jg .loop

    push r5
    int 0 4
    push $msg
    int 0 3
    push r0
    int 0 4
    push $newline
    int 0 3
    ret
```

**Output**:

```
Factorial result:
7! = 5040
6! = 720
5! = 120
4! = 24
3! = 6
2! = 2
1! = 1
```

---

### 6.2 Reverse Number

```asm
@ORIGIN 0
[text]
.start
    push 123456
    call .split
    term

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

**Output**:

```
654321
```

---

## 7. Conclusion

MyVM is a flexible educational virtual machine with:

* **Simple stack-based execution model**.
* **Rich instruction set** with memory, arithmetic, and flow control.
* **Custom assembly language** supporting constants, labels, and structured programs.
* **Interrupt system** for I/O.

It can be extended with new modules, instructions, or compiler features to support more advanced applications.

---
