
section .text
extern snek_error
extern snek_print
my_error:
and rsp, -16
mov rdi, rsi
call snek_error
global our_code_starts_here
  our_code_starts_here:
push rbp
mov rbp, rsp
sub rsp, 32
mov rax, 10
mov [rbp - 8], rax
mov rax, 8
mov [rbp - 8], rax
mov rax, [rbp - 8]
add rax, [rbp - 8]
mov rsi, 2
jo my_error
mov [rbp - 16], rax
mov rax, [rbp - 8]
mov [rbp - 8], rax
mov rax, 4
sar rax, 1
imul rax, [rbp - 8]
mov rsi, 2
jo my_error
mov [rbp - 24], rax
mov rax, [rbp - 16]
mov [rbp - 32], rax
mov rax, [rbp - 24]
sub rax, [rbp - 32]
mov rsi, 2
jo my_error
leave
ret

