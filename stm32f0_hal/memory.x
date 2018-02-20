MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  /* TODO Adjust these memory regions to match your device memory layout */
  FLASH : ORIGIN = 0x8000000, LENGTH = 128K
  RAM : ORIGIN = 0x20000000, LENGTH = 16K
}


  /* space reserved for the stack */
  _stack_size = 0x200;

  /* `.` is right after the .bss and .data sections */
  _heap_start = .;
  _heap_end = ORIGIN(RAM) + LENGTH(RAM) - _stack_size;
