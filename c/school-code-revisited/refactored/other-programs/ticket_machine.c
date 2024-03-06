#include <stdio.h>
#include <stdlib.h>

const int TICKET_COST = 6;
const int LINE_SIZE = 256;

int main() {
  char line[LINE_SIZE];

  int total_money = 0;

  printf("A ticket costs $%d. Purchase a ticket to travel\n", TICKET_COST);

  while (total_money < TICKET_COST) {
    int inserted_money = 0;

    printf("Insert money:\n");

    fgets(line, LINE_SIZE, stdin);

    inserted_money = atoi(line);

    total_money += inserted_money;

    printf("Total money inserted: $%d\n", total_money);
  }

  int change = total_money - TICKET_COST;

  printf("Please collect your ticket. Your change is $%d.\n", change);

  return 0;
}
