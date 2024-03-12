#include <stdio.h>
#include <math.h>

main()
{
    int x, numpos, numneg;
    printf("Aquest programa compta quants nombres enters positius i negatius hem entrat\n");
    printf("Entra un nombre enter\n");
    scanf("%d", &x);
    numpos = 0;
    numneg = 0;
    while (x != 0)
    {
        if (x > 0)
        {
            numpos = numpos + 1;
        }
        if (x < 0)
        {
            numneg = numneg + 1;
        }
        printf("Entra un altre nombre enter\n");
        scanf("%d", &x);
    }
    printf("Has entrat un zero. Porgrama acabat\n");
    printf("Has entrat %d nombres posituius i %d nombres negatius\n", numpos, numneg);
}
