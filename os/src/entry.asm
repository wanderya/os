    .section .text.entry
    .global _entry
_entry:
    la sp, boot_satck_top
    call start

    .section .bss.stack
    .global boot_stack
boot_stack:
    .space 4096 * 16
    .global boot_satck_top
boot_satck_top:
