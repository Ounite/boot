OUTPUT_FORMAT(binary)
ENTRY(_start)
STARTUP(boot.o)
SECTIONS
{
    . = 0x0900;
    .text : { *(.text) }
    .rodata : { *(.rodata) }
    .data : { *(.data) }
    . = 0x0ab8; .mbrdiskid : { BYTE(0x4f); BYTE(0x53); BYTE(0x55); BYTE(0x49); }
    . = 0x0abc; .mbrflag : { BYTE(0x5A); BYTE(0x5A); }
    . = 0x0afe; .mbrbootsig : { BYTE(0x55); BYTE(0xaa); }
    .bss : { *(.bss) }
    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.rel.eh_frame)
        *(.note*)
    }
}
