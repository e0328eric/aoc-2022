extern _printf
extern _scanf

extern _read_file
extern _free_file_contents

section .text
solve1:
    push rbp
    mov rbp, rsp
    sub rsp, 16

    lea rdi, [rel filename]
    xor rax, rax
    call _read_file

    cmp rax, 0
    je end_solve1

    mov QWORD [rbp-8], rax
    lea rdi, [rel file_contents]
    mov rsi, rax
    xor rax, rax
    call _printf

    mov rdi, QWORD [rbp-8]
    xor rax, rax
    call _free_file_contents

end_solve1:
    add rsp, 16
    pop rbp
    xor rax, rax
    ret

section .data
file_contents:
    db "%s", 10, 0

filename:
    db "./src/01/example.txt", 0
