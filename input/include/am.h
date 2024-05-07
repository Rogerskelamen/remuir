#define putstr(s) \
  ({ for (const char *p = s; *p; p++) putch(*p); })

void putch(char ch);
void halt(int code) __attribute__((__noreturn__));
