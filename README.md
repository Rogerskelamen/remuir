# remuir

Everyone loves RISC-V!

Everyone loves Rust!

So why not combine them together?

Here comes remuir, named from RISC-V Emulator In Rust, is a simple RISC-V emulator to run riscv binary code which is inspired from an educational emulator [nemu](https://github.com/NJU-ProjectN/nemu). Its goal is to become as usable as [Spike](https://github.com/riscv-software-src/riscv-isa-sim), but it may take more time in the future.

## development log

1. 从命令行参数中读入待载入的二进制文件路径

2. 为二进制文件分配内存空间，包括读取文件的字节大小

3. 将读到的二进制文件的内容给到mem，继续进行初始化

4. 调用`sdb_start()`，暂时直接调用`cpu_exec()`开始执行

5. `cpu_exec()`从`0x80000000`执行完整个程序

6. 对于每一个指令来说，从`PMEM`中读取到对应的指令数据返回给`s.inst`

7. 从`s.inst`这个32bit数据中解析出对应的指令，然后执行相应操作模拟处理器行为

8. 将PMEM的数据结构从`Vec<Byte>`转化为简单的静态数组中的数据

9. 设置sdb的命令行工具

10. 考虑如何将difftest引入到remuir中
