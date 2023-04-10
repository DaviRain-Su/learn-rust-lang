#include <iostream>
#include <stdio.h>

int main() {
  std::cout << "Hello from a C++ Buck2 program!" << std::endl;
  printf("hello, world! From C program\n");
  printf("(500*400)*(300*200) = %d\n", (500 * 400) * (300 * 200));
  printf("((500*400)*300)*200) = %d\n", ((500 * 400) * 300) * 200);
  printf("((200*500)*300)*400) = %d\n", ((200 * 500) * 300) * 400);
  printf("400*(200 *(300*500)) = %d\n", 400 * (200 * (300 * 500)));

  printf("(3.14+1e20) - 1e20 = %f\n", (3.14 + 1e20) - 1e20);
}
