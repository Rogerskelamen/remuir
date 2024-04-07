# remuir

Remuir, named from RISC-V Emulator In Rust, is a simple RISC-V emulator to run riscv binary code which is inspired from an educational emulator [nemu](https://github.com/NJU-ProjectN/nemu). It's may be a similar project to [Spike](https://github.com/riscv-software-src/riscv-isa-sim), but more too simple and dummy.

## development log

1. 从命令行参数中读入待载入的二进制文件路径

2. 为二进制文件分配内存空间，包括读取文件的字节大小

3. 将读到的二进制文件的内容给到mem，继续进行初始化

4. 调用`sdb_start()`，暂时直接调用`cpu_exec()`开始执行

5. `cpu_exec()`从`0x80000000`执行完整个程序

6. 对于每一个指令来说，从`PMEM`中读取到对应的指令数据返回给`s.inst`

7. 从`s.inst`这个32bit数据中解析出对应的指令，然后执行相应操作模拟处理器行为

8. 将PMEM的数据结构从`Vec<Byte>`转化为Hash，这样每次读写的时候就可以按需来创建新的内存数据，具体逻辑就是在写入时创建，读取未创建内存时读出的数据为0
