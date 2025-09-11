[data]

$hello dw "Hello World!" 10 13 0

[text]

.start
push $hello
int 0 3
term
