[data]

$number dw 1998
$newline dw 10 13 0

[text]

push [$number]
int 0 4
push $newline
int 0 3

term