ENTRY(start)

MEMORY
{
    ram (!rx) : org = 0, len = 128M
}

SECTIONS {
	. = 1M;

	.boot :
	{
		/* ensure that the multiboot header is at the beginning */
		*(.multiboot_header)
	} > ram

	.text :
	{
	    *(.text)
	} > ram

    stack_end = ORIGIN(ram) + LENGTH(ram) - 4;

}
