[data]

$num dw 0 1 2 3 4 5 6 7
$title dw "Factorial result: " 13 10 0
$msg dw "! = " 0
$newline dw 10 13 0

[text]

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