# Intro

This directory is a utils box which aims to generate clean riscv binary codes for tests in x86 environment. It provides bare metal environment so that you can just write some C codes and `make` them into simple riscv binary file without local host environment affecting.

## Requirements

You need [riscv64-linux-gnu-toolchain](https://github.com/riscv-collab/riscv-gnu-toolchain) to cross-compile the C codes into riscv elf file. This usually could be installed through some package manager. For instance, on Ubuntu you could install by `sudo apt install gcc-riscv64-linux-gnu`.

### Tiny fix

Though you install `riscv64-linux-gnu-toolchain`, you may still encounter a compile error when running test program.

If compiler yield that `/usr/riscv64-linux-gnu/include/gnu/stubs.h:8:11: fatal error: gnu/stubs-ilp32.h: No such file or directory`, you need to fix it by modifying the following file with root permission:

```diff
--- /usr/riscv64-linux-gnu/include/gnu/stubs.h
+++ /usr/riscv64-linux-gnu/include/gnu/stubs.h
@@ -5,5 +5,5 @@
 #include <bits/wordsize.h>

 #if __WORDSIZE == 32 && defined __riscv_float_abi_soft
-# include <gnu/stubs-ilp32.h>
+//# include <gnu/stubs-ilp32.h>
 #endif
```

## LD

This generator use a customized link script `linker.ld` to help keeping environment clean. If you're curious about this file, just read it and check `ld --verbose` for a default link script.

## Todo

For now, the generator only provides single C code file(*in `test`*) for each test case project. Thus, you can't count it to generate one binary code for a bunch of code files when you want to run a relatively large graph render app. In the future, this feature will be implemented.
