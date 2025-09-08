@org 0

[data]

$name b "Pedram" 0
$year w 1998
$scores w 20 15 13 20 19 13 10
$data dw 0xaaaaaaaa 0xbbbbbbbb 0xcccccccc
$message b "Hello World! This is a test!" 0

[text]

move r1 0
push [$name + r1]
move r2 $name
move r3 &r2
push r3

term
