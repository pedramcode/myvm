[data]

$num dw 5
$title dw "Factorial result: " 13 10 0
$msg dw "! = " 0
$newline dw 10 13 0

[text]

push [$num]
call .factorial

push $title
int 0 3
push [$num]
int 0 4
push $msg
int 0 3
push r0
int 0 4
push $newline
int 0 3

term

.factorial
    dup
    pop r1
    move r0 1 ; acc
    .loop
        push r0
        mul
        pop r0

        dec r1
        push r1
        jg .loop

    ret