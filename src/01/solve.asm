%include "./src/util.asm"

;; Compile-Time Constants
%define STACK_OFFSET          64

;; Stack Pointer Labeling
%define FILE_CONTENT          [rbp -  8]
%define RESULT_NUMBER         [rbp - 12]
%define TOTAL_NUMBER          [rbp - 16]
%define READ_NUMBER           [rbp - 20]
%define POINTER_STORE1        [rbp - 32]

section .text

solve1:
    push rbp

    lea rdi, [rel solve1_header_fmt]
    call _printf

    call solve1_part1

    pop rbp
    ret

solve1_part1:
    push rbp
    mov rbp, rsp
    sub rsp, STACK_OFFSET

    lea rdi, [rel filename]
    xor rax, rax
    call _read_file

    cmp rax, 0
    je end_solve1

    mov QWORD FILE_CONTENT, rax
    mov r8, QWORD FILE_CONTENT
    dec r8
    mov QWORD [rel end_strtoll_ptr], r8

    mov DWORD RESULT_NUMBER, 0
    mov DWORD TOTAL_NUMBER, 0

loop1:
    inc r8
    cmp BYTE [r8], NEWLINE_CHAR
    jne string_to_number

    mov QWORD POINTER_STORE1, r8

    mov edi, DWORD TOTAL_NUMBER
    mov esi, DWORD RESULT_NUMBER
    xor rax, rax
    call maximum

    mov DWORD RESULT_NUMBER, eax
    mov DWORD TOTAL_NUMBER, 0
    mov r8, QWORD POINTER_STORE1
    inc r8

string_to_number:
    mov rdi, r8
    lea rsi, [rel end_strtoll_ptr]
    xor rdx, rdx
    xor rax, rax
    call _strtol

    mov DWORD READ_NUMBER, eax

    mov edx, DWORD READ_NUMBER
    mov eax, DWORD TOTAL_NUMBER
    add eax, edx
    mov DWORD TOTAL_NUMBER, eax

    mov r8, [rel end_strtoll_ptr]
    cmp BYTE [r8], NULL_CHAR
    jne loop1
end_loop1:

    mov edi, DWORD TOTAL_NUMBER
    mov esi, DWORD RESULT_NUMBER
    xor rax, rax
    call maximum

    lea rdi, [rel part1_fmt]
    mov esi, DWORD RESULT_NUMBER
    call _printf

    mov rdi, QWORD FILE_CONTENT
    xor rax, rax
    call _free_file_contents

end_solve1:
    add rsp, STACK_OFFSET
    pop rbp
    xor rax, rax
    ret

section .data
solve1_header_fmt:
    db "<< Problem 1 >>", NEWLINE_CHAR, 0

print_num_fmt:
    db "The number is: %lld", NEWLINE_CHAR, 0

part1_fmt:
    db "part1: %ld", NEWLINE_CHAR, 0

filename:
    db "./src/01/input.txt", 0

section .bss
end_strtoll_ptr: resq 1
