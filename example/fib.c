#include <stdio.h>

void fib(int n) {
  int x = 0;
  int y = 1;
  int z;

  for (int i = 1; i <= n; i++) {
    printf("n%d: %d\n", i, x);

    z = x + y;
    x = y;
    y = z;
  }
}

int main() { fib(20); }
