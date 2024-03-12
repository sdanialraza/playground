#include <stdio.h>
#include <stdlib.h>
#include <time.h>

const int LINE_SIZE = 256;

const char QUESTION[18] = "What is 23 x 45?\n";
const int CORRECT_ANSWER = 1035;

int main() {
  char line[LINE_SIZE];

  printf("This program will test your multiplication skills\n");
  printf(QUESTION);

  long start_time = time(NULL);

  fgets(line, LINE_SIZE, stdin);

  int given_number = atoi(line);

  while (given_number != CORRECT_ANSWER) {
    printf("No, that's not correct. Try again:\n");

    fgets(line, LINE_SIZE, stdin);

    given_number = atoi(line);
  }

  long end_time = time(NULL);

  printf("Congratulations! You got it right in %ld seconds.\n", end_time - start_time);

  return 0;
}
