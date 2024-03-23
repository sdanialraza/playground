#include <stdio.h>
#include <stdlib.h>

const int LINE_SIZE = 256;

int main() {
  char line[LINE_SIZE];

  printf("This program will calculate the average of the marks you entered\n");
  printf("Please enter the total number of subjects:\n");

  fgets(line, LINE_SIZE, stdin);

  int subjects = atoi(line);

  printf("Please enter the marks for each subject:\n");

  float total = 0;

  for (int index = 0; index < subjects; index++) {
    fgets(line, LINE_SIZE, stdin);

    float marks = (float)atoi(line);

    total += marks;
  }

  float average = total / (float)subjects;

  printf("The average marks are %.2f\n", average);

  return 0;
}
