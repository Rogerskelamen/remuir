# remuir

Everyone loves RISC-V!

Everyone loves Rust!

So why not combine them together?

Here comes remuir, named from RISC-V Emulator In Rust, is a simple RISC-V emulator to run riscv binary code which is inspired from an educational emulator [nemu](https://github.com/NJU-ProjectN/nemu). Its goal is to become as usable as [Spike](https://github.com/riscv-software-src/riscv-isa-sim), but it may take a long way to make.

## Spec

For now, remuir supports only RV32I isa to execute code.

Virtual machine in it:

1. pc

2. 32 gpr

3. no csr regs (***TODO***)

## Dependency

1. structopt: for cli options handling

2. rustyline: provide a shell prompt for sdb

3. capstone: a disassembly engine (**need rust version greater than 1.60.0**)

4. libloading: load C dynamic link library

5. lazy_static: lazy load global static variable, this is for some global variable being initialized when program run after some time point

## Problem

Rust suggests that user should reconsider it when trying to use global mutable variables. However, I use global mutable variables(*in `static mut` style*) almost everywhere in my project. It's easy to make such design because the state of CPU and Memory is accessible by modules for most of time.

If not using global mutable variables, another way is to implement CPU and Memory as trait object, every state access can only caused by call function of those objects. To reach the goal, I have to restruct the project, mainly the `cpu`, `isa`, `memory` module will be rewritten. (*A great work to go*)

I decide to open another branch to do it in the future.

## FAQ

If you turn on the difftest feature in `config.mk`, you may encounter trouble when compiling [spike](https://github.com/NJU-ProjectN/riscv-isa-sim) reference, which tells you `boost_regex` could not be linked. This is because an open issue for some OS, you could find more [here](https://github.com/riscv-software-src/riscv-isa-sim/issues/834). To solve the trouble, just modify the following file:

```diff
--- a/tools/spike-diff/Makefile
+++ b/tools/spike-diff/Makefile
@@ -7,7 +7,7 @@ REPO_BUILD_PATH = $(REPO_PATH)/build
 REPO_MAKEFILE = $(REPO_BUILD_PATH)/Makefile
 $(REPO_MAKEFILE):
 	@mkdir -p $(@D)
-	cd $(@D) && $(abspath $(REPO_PATH))/configure
+	cd $(@D) && $(abspath $(REPO_PATH))/configure --without-boost --without-boost-asio --without-boost-regex
 	sed -i -e 's/-g -O2/-O2/' $@

 SPIKE = $(REPO_BUILD_PATH)/spike
```

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

10. 考虑使用llvm的反汇编接口实现二进制指令到指令字符串的转化(这里使用的是capstone)

11. 考虑如何将difftest引入到remuir中，应该可以用rust调用c函数

12. 利用实现好的difftest工具，完成剩余RISC-V指令的实现
