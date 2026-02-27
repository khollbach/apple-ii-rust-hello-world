/* Apple II linker script, based on LLVM-MOS's `sim` linker script.
 */

/* Provide imaginary (zero page) registers. */
__rc0 = 0x00;
INCLUDE imag-regs.ld
ASSERT(__rc31 == 0x001f, "Inconsistent zero page map.")

MEMORY {
    zp : ORIGIN = __rc31 + 1, LENGTH = 0x100 - (__rc31 + 1)
    ram (rw) : ORIGIN = 0x6000, LENGTH = 0x6000
}

REGION_ALIAS("c_readonly", ram)
REGION_ALIAS("c_writeable", ram)

SECTIONS { INCLUDE c.ld }

/* Set initial soft stack address to just above last memory address. (It grows down.) */
/* NOTE: it seems we need to set this by hand; not sure why! See below */
__stack = 0xc000;

OUTPUT_FORMAT {
    /* Set up stack pointer at $00 */
    /* (note that these are little endian) */
    SHORT(0x00a9) /* lda #$00 */
    SHORT(0x0085) /* sta $00 */
    SHORT(0xc0a9) /* lda #$c0 */
    SHORT(0x0185) /* sta $01 */
    TRIM(ram)
}
