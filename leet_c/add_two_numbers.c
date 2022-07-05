#include <stdio.h>
#include <stdlib.h>

/*
 * You are given two non-empty linked lists representing two non-negative 
 * integers. The digits are stored in reverse order, and each of their nodes 
 * contains a single digit. Add the two numbers and return the sum as a linked 
 * list. You may assume the two numbers do not contain any leading zero, 
 * except the number 0 itself.
 */
struct ListNode {
  int val;
  struct ListNode* next;
};

struct ListNode* addTwoNumbers(struct ListNode* l1, struct ListNode* l2) {
  struct ListNode* temp = malloc(sizeof(struct ListNode));
  temp->val = 0; temp->next = NULL;

  struct ListNode* curr = temp;

  int carry = 0;
  int sum   = 0;

  while (l1 != NULL || l2 != NULL || carry != 0) {
    int x = l1 != NULL ? l1->val : 0;
    int y = l2 != NULL ? l2->val : 0;

    sum = x + y + carry;
    carry = sum / 10;
    sum %= 10;

    curr->next = malloc(sizeof(struct ListNode));
    curr->next->next = NULL;
    curr->next->val  = sum;
    curr = curr->next;

    l1 = l1 == 0 ? 0 : l1->next;
    l2 = l2 == 0 ? 0 : l2->next;
  }
  return temp->next;
}

