#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

const int LINE_SIZE = 256;

bool is_positive(int number) { return number > 0; };

int main() {
  char line[LINE_SIZE];

  printf("This program will tell if the given number is positive or negative\n");
  printf("Enter a number:\n");

  fgets(line, LINE_SIZE, stdin);

  int given_number = atoi(line);

  while (given_number == 0) {
    printf("Please enter a non-zero number:\n");

    fgets(line, LINE_SIZE, stdin);

    given_number = atoi(line);
  }

  printf("%d is a %s number.\n", given_number, is_positive(given_number) ? "positive" : "negative");

  return 0;
}
