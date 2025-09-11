[data]

$number w 1998
$newline dw 10 13 0

[text]

.start

push [$number]
SHR 16
int 0 4
push $newline
int 0 3

term