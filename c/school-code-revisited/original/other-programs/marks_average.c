#include <stdio.h>

int main() 
{
  int Subjects, i;
  float Marks, Total, Average;
  printf("Please enter the Total Number of Subjects\n");
  scanf("%d", &Subjects);
  printf("Please Enter the Marks for each Subject\n");
  for (i = 0; i < Subjects; i++) {
    scanf("%f", &Marks);
    Total = Total + Marks;
  }
  Average = Total / Subjects;
  printf("Average Marks = %.2f\n", Average);
}