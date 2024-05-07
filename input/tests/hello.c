#include <am.h>

int main() {
  char *hello = "hello, world\n";
  for (int i = 0; i < 13; i++) {
    putch(hello[i]);
  }
}
