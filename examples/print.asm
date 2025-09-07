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
