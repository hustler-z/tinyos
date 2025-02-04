// @Hustler's Project

#include <asm/entry.h>

OUTPUT_FORMAT("elf64-littleaarch64")
OUTPUT_ARCH("aarch64")
ENTRY(__head__)

SECTIONS {
    . = __TINY_START__;

    __tiny_start = .;

    .text : {
        __text_start = .;
        *(.text.head)
        *(.text.vecs)
        *(.text.gic)
        *(.text.timer)
        *(.text.init)
        *(.text .text.*)
        . = ALIGN(8);
        __text_end = .;
    }

    .data : {
        __data_start = .;
        *(.data .data.*)
        . = ALIGN(8);
        __data_end = .;
    }

    .rodata : {
        __rodata_start = .;
        *(.rodata.debug)
        *(.rodata .rodata.*)
        . = ALIGN(8);
        __rodata_end = .;
    }

    .console : {
        __console_start = .;
        /* console commands */
        __cmd_list__ = .;
        KEEP(*(.cmd_list))
        __cmd_auto__ = .;
        KEEP(*(.cmd_auto))
        KEEP(*(.cmd_end))
        . = ALIGN(8);
        __console_end = .;
    }

    . = ALIGN(4096);

    .bss : {
        __bss_start = .;
        *(.bss.percpu)
        *(.bss.percpu.ro)
        *(.bss .bss.*)
        . = ALIGN(4096);
        __bss_end = .;
    }

    .stack : {
        __stack_start = .;
        *(.stack .stack.*)
        . = ALIGN(4096);
        __stack_end = .;
    }

    .heap : {
        __heap_start = .;
        *(.heap .heap.*)
        . = ALIGN(4096);
        __heap_end = .;
    }

    __tiny_end = .;
}
