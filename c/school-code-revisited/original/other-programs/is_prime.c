#include <stdio.h>
#include <math.h>

main()
{
  int a, b, i;
  printf("This program will tell if the given number is prime or not\n");
  printf("Enter a number\n");
  scanf("%d", &a);
  b = 0;
  for (i = 2; i <= a - 1; i = i + 1)
  {
    if (a % i == 0)
    {
      b = 1;
    }
  }
  if (a == 1)
  {
    printf("1 is not prime nor composite.\n");
  }
  else
  {
    if (b == 1)
    {
      printf("%d is not a prime number.\n", a);
    }
    else
    {
      printf("%d is a prime number.\n", a);
    }
  }
}
