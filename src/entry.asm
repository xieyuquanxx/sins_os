# 内核入口点，被放在0x8020000处
  .section .text.entry
  .global _start

_start:
  la sp, boot_stack_top
  call rust_main

  .section .bss.stack
  .global boot_stack
boot_stack:
  .space 4096 * 16 # 分配栈空间 64KB
  .global boot_stack_top
boot_stack_top: # 栈顶