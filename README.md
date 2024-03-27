# remuir

Remuir, named from RISC-V Emulator In Rust, is a simple RISC-V emulator to run riscv binary code which is inspired from an educational emulator [nemu](https://github.com/NJU-ProjectN/nemu). It's may be a similar project to [Spike](https://github.com/riscv-software-src/riscv-isa-sim), but more too simple and dummy.

## development log

1. 从命令行参数中读入待载入的二进制文件路径

2. 为二进制文件分配内存空间，包括读取文件的字节大小
