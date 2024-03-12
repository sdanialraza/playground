#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

const int LINE_SIZE = 256;

bool is_positive(int number) { return number > 0; }

int main() {
  char line[LINE_SIZE];

  printf("This program will count how many positive and negative numbers you entered\n");
  printf("Enter a number or zero to finish:\n");

  int positive_numbers = 0;
  int negative_numbers = 0;

  fgets(line, LINE_SIZE, stdin);

  int given_number = atoi(line);

  while (given_number != 0) {
    if (is_positive(given_number)) {
      positive_numbers++;
    } else {
      negative_numbers++;
    }

    printf("Enter another number:\n");

    fgets(line, LINE_SIZE, stdin);

    given_number = atoi(line);
  }

  printf("You entered %d positive numbers and %d negative numbers.\n", positive_numbers, negative_numbers);
}
