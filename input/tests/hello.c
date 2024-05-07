#include <am.h>

int main() {
  char *hello = "hello, world";
  for (int i = 0; i < 12; i++) {
    putch(hello[i]);
  }
}
