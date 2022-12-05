%include "src/01/solve.asm"

global _main

section .text
_main:
    push rbp

    call solve1

    pop rbp
    xor rax, rax
    ret
