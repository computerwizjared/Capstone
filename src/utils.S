.globl put32
put32:
    STR     W1, [X0]
    RET

.globl get32
get32:
    LDR     W0, [X0]
    RET

.globl delay
delay:
    SUBS    X0, X0, #1
    BNE     delay
    RET