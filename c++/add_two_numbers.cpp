/**
 * https://leetcode.com/problems/add-two-numbers/
 *
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        ListNode* sum = new ListNode(0);
        ListNode* p = sum;
        int carry = 0;
        int val = 0;
        
        while (l1 != NULL && l2 != NULL) {
            val = l1->val + l2->val + carry;
            carry = val > 9 ? 1 : 0;
            val = val > 9 ? val - 10 : val;
            
            ListNode* newNode = new ListNode(val);
            p->next = newNode;
            p = newNode;
            
            l1 = l1->next;
            l2 = l2->next;
        }
        
        while (l1 != NULL) {
            val = l1->val + carry;
            carry = val > 9 ? 1 : 0;
            val = val > 9 ? val - 10 : val;
            
            ListNode* newNode = new ListNode(val);
            p->next = newNode;
            p = newNode;
            
            l1 = l1->next;
        }
        
        while (l2 != NULL) {
            val = l2->val + carry;
            carry = val > 9 ? 1 : 0;
            val = val > 9 ? val - 10 : val;
            
            ListNode* newNode = new ListNode(val);
            p->next = newNode;
            p = newNode;
            
            l2 = l2->next;
        }
        
        if (carry == 1) {
            ListNode* newNode = new ListNode(1);
            p->next = newNode;
            p = newNode;
        }
        
        return sum->next;
    }
};
