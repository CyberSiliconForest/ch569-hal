
/**
 * CH569 memory map
 *
 * Note this is using 96K RAMX and 32K zero wait state flash configuration
 * This configuration must be set on the device first
 */

MEMORY
{
    FLASH : ORIGIN = 0x00000000, LENGTH = 448K
    RAMS : ORIGIN = 0x20000000, LENGTH = 16K
    RAMX : ORIGIN = 0x20020000, LENGTH = 96K
}

ENTRY(_start_hal)

SECTIONS
{
    .text :
    {
        _stext = .;
        KEEP(*(.init));
        KEEP(*(.init.rust));
        . = ALIGN(4);
        KEEP(*(.trap));
        KEEP(*(.trap.rust));
        *(.text .text.*);
        _etext = .;
    } > FLASH

    .bss (NOLOAD) :
    {
        . = ALIGN(8);
        _sbss = .;
        *(.dynsbss)
        *(.sbss)
        *(.sbss.*)
        *(.gnu.linkonce.sb.*)
        *(.scommon)
        *(.sbss2)
        *(.sbss2.*)
        *(.gnu.linkonce.sb2.*)
        *(.dynbss)
        *(.sbss .sbss.* .bss .bss.*);
        *(.share.mem)
        *(.gnu.linkonce.b.*)
        *(COMMON)

        . = ALIGN(8);
        _ebss = .;
    } > RAMS
}