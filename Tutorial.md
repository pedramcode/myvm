# MyVM Assembly Language Tutorial

## Table of Contents
1. [Introduction](#introduction)
2. [Memory and Stack](#memory-and-stack)
3. [Registers](#registers)
4. [Flags](#flags)
5. [Opcodes and Variants](#opcodes-and-variants)
6. [Assembly Syntax](#assembly-syntax)
7. [Examples](#examples)

## Introduction

MyVM is a stack-based virtual machine with 32-bit memory cells and registers. Programs are executed by pushing values onto the stack, performing operations, and storing results. This tutorial covers the assembly language syntax and provides examples.

## Memory and Stack

- Memory consists of 32-bit (4-byte) cells
- Stack grows backwards from the end of memory
- Stack size is defined when VM starts
- Stack overflow occurs when pushing beyond stack limit
- Stack underflow occurs when popping from empty stack

## Registers

| Register | Address | Purpose           |
|----------|---------|-------------------|
| r0       | 0       | General purpose   |
| r1       | 1       | General purpose   |
| r2       | 2       | General purpose   |
| r3       | 3       | General purpose   |
| r4       | 4       | General purpose   |
| r5       | 5       | General purpose   |
| r6       | 6       | General purpose   |
| r7       | 7       | General purpose   |
| pc       | 100     | Program counter   |

## Flags

Flags are set based on operation results:
- `zero`: Result was zero
- `negative`: Result was negative
- `overflow`: Arithmetic overflow occurred
- `carry`: Arithmetic carry occurred

## Opcodes and Variants

### Main Opcodes
```plaintext
Push = 0xf001
Pop = 0xf002
Add = 0xf003
Sub = 0xf004
Swap = 0xf005
Move = 0xf006
Store = 0xf007
Jump = 0xf008
And = 0xf009
Or = 0xf00a
Xor = 0xf00b
Not = 0xf00c
SHR = 0xf00d
SHL = 0xf00e
Call = 0xf00f
Ret = 0xf010
Dup = 0xf011
Int = 0xf012
Drop = 0xf013
Mul = 0xf014
Div = 0xf015
Inc = 0xf016
Dec = 0xf017
SafeCall = 0xf018
Terminate = 0xffff
```

### Common Variants
```plaintext
Default = 0x0000
PushConst = 0xa001
PushReg = 0xa002
PushAddr = 0xa003
PopReg = 0xa004
PopAddr = 0xa005
MoveConst = 0xa006
MoveReg = 0xa007
MoveAddr = 0xa008
StoreConst = 0xa009
StoreReg = 0xa00a
JumpNotZero = 0xa00b
JumpZero = 0xa00c
JumpGreater = 0xa00d
JumpGreaterEqual = 0xa00e
JumpLesser = 0xa00f
JumpLesserEqual = 0xa010
```

## Assembly Syntax

### Comments
```asm
; This is a comment
```

### Numbers
```asm
123        ; Decimal
0xabc      ; Hexadecimal
0b010101   ; Binary
```

### Meta Commands
```asm
@ORIGIN 10  ; Code starts at address 10
```

### Sections
```asm
[text]     ; Code section
[data]     ; Data section
```

### Labels
```asm
.mylabel   ; Label definition
JMP .mylabel ; Jump to label
```

### Data Definitions
```asm
$name b "Pedram"         ; Byte array
$scores w 0x10 0x20     ; Word array
$data dw 0x11da00ae     ; Double word
```

## Commands

### PUSH
```asm
PUSH 10           ; Constant
PUSH r0           ; Register value
PUSH $name        ; Address of data
PUSH [$name]      ; Value at address
PUSH [$name + 1]  ; Value at address + offset
PUSH [$name + r0] ; Value at address + register offset
PUSH .mylabel     ; Label address
PUSH &0x1010      ; Memory value
```

### POP
```asm
POP r2        ; To register
POP &0x321    ; To memory
```

### Arithmetic Operations
```asm
PUSH 10
PUSH 20
ADD        ; Result: 30

PUSH 10
PUSH 20
SUB        ; Result: 10

PUSH 10
PUSH 20
MUL        ; Result: 200

PUSH 10
PUSH 3
DIV        ; Result: 3, remainder in r3
```

### Logical Operations
```asm
PUSH 0b1010
PUSH 0b1100
AND        ; Result: 0b1000

PUSH 0b1010
PUSH 0b1100
OR         ; Result: 0b1110

PUSH 0b1010
PUSH 0b1100
XOR        ; Result: 0b0110

PUSH 0b1010
NOT        ; Result: 0xfffffff5
```

### Flow Control
```asm
JMP .label    ; Unconditional jump
JNZ .label    ; Jump if not zero
JZ .label     ; Jump if zero
JG .label     ; Jump if greater
JGE .label    ; Jump if greater or equal
JL .label     ; Jump if less
JLE .label    ; Jump if less or equal

CALL .func    ; Call function
SAFECALL .func ; Call with state preservation
RET           ; Return from call
```

### Other Operations
```asm
SWAP        ; Swap top two stack items
DROP        ; Remove top stack item
DUP         ; Duplicate top stack item
DUP 3       ; Duplicate top item 3 times
DUP r0      ; Duplicate top item r0 times

INC r0      ; Increment register
DEC r0      ; Decrement register

SHR 3       ; Shift right by 3
SHL r0      ; Shift left by r0 value

INT 0 0     ; Call interrupt module 0, function 0
TERM        ; Terminate program
```

## Examples

### Factorial Calculation
```asm
@org 0
[data]

$num dw 0 1 2 3 4 5 6 7
$title dw "Factorial result: " 13 10 0
$msg dw "! = " 0
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

### Number Reversal
```asm
@org 0
[text]

.start
    push 123456
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

### Interrupt Usage (IO Module)
The IO module (module 0) provides these functions:
- Function 0: Print ASCII character (pop value)
- Function 1: Print multiple characters (pop count, then pop characters)
- Function 2: Print until value (pop stop value, then pop characters until value)
- Function 3: Print null-terminated string (pop address, print until null)
- Function 4: Print number as digits (pop value)

```asm
; Print "Hello World"
push 'H'
int 0 0
push 'e'
int 0 0
push 'l'
int 0 0
push 'l'
int 0 0
push 'o'
int 0 0
push ' '
int 0 0
push 'W'
int 0 0
push 'o'
int 0 0
push 'r'
int 0 0
push 'l'
int 0 0
push 'd'
int 0 0
push 10    ; Newline
int 0 0
push 13    ; Carriage return
int 0 0

; Print number 123
push 123
int 0 4
```

## Best Practices

1. Always include `TERM` at the end of your program
2. Use `SAFECALL` when you need to preserve register state
3. Manage stack carefully to avoid overflow/underflow
4. Use comments to document complex operations
5. Test interrupt calls with simple values first

This tutorial covers the basics of MyVM assembly programming. Experiment with these examples and explore the various opcodes and addressing modes to become proficient with this virtual machine architecture.