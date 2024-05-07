#include <am.h>

int main() {
  char *hello = "hello, world\n";

  for (int i = 0; i < 1000; i++) {
    putstr(hello);
  }
}
