.globl memzero
memzero:
    STR     XZR, [X0], #8     // store 0 in
    SUBS    X1, X1, #8
    B.GT    memzero
    RET