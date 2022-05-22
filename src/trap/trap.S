.altmacro

.macro SAVE_REG n
  sd x\n, \n*8(sp)
.endm

.macro LOAD_REG n
  ld x\n, \n*8(sp)
.endm

  .section .text
  .globl __alltraps
  .globl __restore
  .align 2
__alltraps:
  csrrw sp, sscratch, sp
  # sp->kernel stack, sscratch->user stack
  # allocate a TrapContext on kernel stack
  addi sp, sp, -34*8 # x0~x31 sstatus sepc 34个usize
  # 此时sp指向的是data[0]也就是x0
  sd x1, 1*8(sp)
  sd x3, 3*8(sp)
  .set n, 5
  .rept 27 # 保存x5~x31
    SAVE_REG %n
    .set n, n+1
  .endr
  # t0, t1 临时寄存器已经保存好了，所以可以使用
  csrr t0, sstatus
  csrr t1, sepc
  sd t0, 32*8(sp) # TrapContext 的sstatus字段
  sd t1, 33*8(sp) # TrapContext 的sepc字段
  # 保存用户栈的地址
  csrr t2, sscratch
  sd t2, 2*8(sp)
  # 设置函数调用参数，参数保存在a0中, sp指向的是TrapContext
  mv a0, sp
  call trap_handler

__restore:
  # 此时a0指向TrapContext
  mv sp, a0
  # restore sstatus/sepc
  ld t0, 32*8(sp)
  ld t1, 33*8(sp)
  ld t2, 2*8(sp) # 用户栈的地址
  csrw sstatus, t0
  csrw sepc, t1
  csrw sscratch, t2 # sscratch->user stack
  # 保存通用寄存器
  ld x1, 1*8(sp)
  ld x3, 3*8(sp)
  .set n, 5
  .rept 27
    LOAD_REG %n
    .set n, n+1
  .endr
  # release TrapContext on kernel stack
  addi sp, sp, 34*8
  csrrw sp, sscratch, sp
  # now sp->user stack ssctatch->kernel stack
  sret
