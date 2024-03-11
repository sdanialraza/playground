#include <math.h>
#include <stdio.h>

main() {
  int a, b, c;
  printf("This program will tell if the entered number is positive or negative\n");
  printf("Enter a real number\n");
  scanf("%d", &a);
  b = 0;
  c = 0;
  while (a != 0) {
    if (a < 0) {
      printf("The number is negative\n");
      b = b + 1;
    } else {
      printf("The number is positive\n");
      c = c + 1;
    }
    scanf("%d", &a);
  }
  printf("Entered number is zero,program stopped.\n");
  printf("You entered %d positive numbers.\n", b);
  printf("You entered %d negative numbers.\n", c);
}
