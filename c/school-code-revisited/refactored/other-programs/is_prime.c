#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

const int LINE_SIZE = 256;

bool is_prime(int number) {
  if (number <= 1) {
    return false;
  }

  for (int index = 2; index * index <= number; index++) {
    if (number % index == 0) {
      return false;
    }
  }

  return true;
}

int main() {
  char line[LINE_SIZE];

  printf("This program will tell if the given number is prime or not\n");
  printf("Enter a number:\n");

  fgets(line, LINE_SIZE, stdin);

  int given_number = atoi(line);

  printf("%d is %s prime number.\n", given_number, is_prime(given_number) ? "a" : "not a");

  return 0;
}
