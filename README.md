# Shy is Not Shy OS
使用Rust编写的RiscV的操作系统内核。
参考资料
- [rcore](https://rcore-os.github.io/rCore-Tutorial-Book-v3/)

## todolist
- [x] chapter 1
- [x] chapter 2
- [ ] chapter 3
- [ ] chapter 4
- [ ] chapter 5
- [ ] chapter 6
- [ ] chapter 7
- [ ] chapter 8
- [ ] chapter 9

## features
- [x] 彩色LOG打印 使用crate `log`
- [x] 批处理
  - 自动运行应用程序，一个程序结束后自动加载下一个应用程序到内存中并执行
  - 实现特权级的隔离
  - 用户栈/内核栈的切换