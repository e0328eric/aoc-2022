;; Functions from C
extern _printf
extern _scanf
extern _strtol

;; Functions from Rust
extern _read_file
extern _free_file_contents

;; Compile-Time Constants
%define STACK_OFFSET        32
%define NEWLINE_CHAR        0x0A
%define NULL_CHAR           0x00

;; Stack Pointer Labeling
%define FILE_CONTENT        [rbp - 8]
%define TOTAL_NUMBER        [rbp - 12]
%define READ_NUMBER         [rbp - 16]
%define POINTER_STORE1      [rbp - 24]

section .text
solve1:
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

    mov DWORD TOTAL_NUMBER, 0

loop1:
    inc r8
    cmp BYTE [r8], NEWLINE_CHAR
    jne string_to_number

    mov QWORD POINTER_STORE1, r8

    lea rdi, [rel print_total_num_fmt]
    mov esi, DWORD TOTAL_NUMBER
    xor rax, rax
    call _printf

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

    lea rdi, [rel print_num_fmt]
    mov esi, eax
    xor rax, rax
    call _printf

    xor rdi, rdi
    mov edx, DWORD READ_NUMBER
    mov eax, DWORD TOTAL_NUMBER
    add eax, edx
    mov DWORD TOTAL_NUMBER, eax


    mov r8, [rel end_strtoll_ptr]
    cmp BYTE [r8], NULL_CHAR
    jne loop1
end_loop1:

    lea rdi, [rel print_total_num_fmt]
    mov esi, DWORD TOTAL_NUMBER
    xor rax, rax
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
file_contents:
    db "%s", NEWLINE_CHAR, 0

print_num_fmt:
    db "The number is: %lld", NEWLINE_CHAR, 0

print_total_num_fmt:
    db "The total number is: %lld", NEWLINE_CHAR, 0

filename:
    db "./src/01/example.txt", 0

section .bss
end_strtoll_ptr: resq 1
