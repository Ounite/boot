bits 16

extern main

section .text

global _start
_start:
    cli

    mov ax, 0   ; reset segment registers
    mov ds, ax
    mov es, ax

    mov cx, 0x0100  ; self relocate from 0x7c00 to 0x0900
    mov si, 0x7c00
    mov di, 0x0900
    rep movsw

    mov ss, ax       ; reset stack
    mov esp, 0x0900
    mov ebp, esp

    sti

    mov [BOOT_DISK_NUMBER], dl  ; save current disk number

    jmp 0:main  ; jump into rust code

global start_envz
start_envz:
    mov esp, 0x7c00
    mov ebp, esp
    push WORD [BOOT_DISK_NUMBER]
    jmp 0x7c00

global abort
abort:
    cli
    hlt
    jmp abort

section .data

global BOOT_DISK_NUMBER
BOOT_DISK_NUMBER db 0
