#include <stdio.h>
#include <math.h>

main()
{
    int a, b, c;
    printf("A ticket costs 6$, Purchase  a ticket to travel\n");
    printf("Insert the money\n");
    b = 0;
    while (b < 6)
    {
        scanf("%d", &a);
        printf("Insufficient money, put more money\n");
        b = b + a;
    }
    c = b - 6;
    printf("Please collect your ticket and the change is %d$\n", c);
}
