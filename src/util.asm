;; Functions from C
extern _printf
extern _scanf
extern _strtol

;; Functions from Rust
extern _read_file
extern _free_file_contents

%define NEWLINE_CHAR        0x0A
%define NULL_CHAR           0x00

;; rdi : first number
;; rsi : second number
maximum:
    push rbp

    cmp rdi, rsi
    jle maximum_snd

    mov rax, rdi
    jmp end_maximum

maximum_snd:
    mov rax, rsi

end_maximum:
    pop rbp
    ret

minimum:
    push rbp

    cmp rdi, rsi
    jge minimum_snd

    mov rax, rdi
    jmp end_minimum

minimum_snd:
    mov rax, rsi

end_minimum:
    pop rbp
    ret
