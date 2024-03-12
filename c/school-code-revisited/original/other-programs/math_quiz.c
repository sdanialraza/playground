#include <stdio.h>
#include <time.h>

void main()
{
    int answer, start_time, end_time, duration;
    printf("Answer this question as fast as you can!\n");
    printf("What is 23x45\n");
    start_time = time(NULL);
    scanf("%d", &answer);
    if (answer == 1035)
    {
        end_time = time(NULL);
        duration = end_time - start_time;
        printf("Your answer is correct!\n");
        printf("It took you %d seconds to answer\n", duration);
    }
    else
    {
        printf("Your answer is wrong!");
    }
}
