#include <device.h>
#include <stdint.h>

int main(const char *args);

static const char mainargs[] = MAINARGS;

void putch(char ch) {
  // TODO
  *(uint8_t *)SERIAL_PORT = ch;
}

void halt(int code) {
  asm volatile("mv a0, %0; ebreak" : : "r"(code));

  // never reached here
  while (1);
}

void _metal_init() {
  int ret = main(mainargs);
  halt(ret);
}
