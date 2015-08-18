/**
 * https://leetcode.com/problems/remove-duplicates-from-sorted-list/
 * 刚好用上昨天学到的 c++ 的 unique() 算法
 * https://leetcode.com/problems/contains-duplicate/
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
    ListNode* deleteDuplicates(ListNode* head) {
        if (head == NULL || head->next == NULL) {
            return head;
        }
        
        ListNode* retHead = head;
        ListNode* first = head->next;
        while (first != NULL) {
            if (first->val != head->val) {
                head = head->next;
                head->val = first->val;
            }
            first = first->next;
        }
        

        // release nodes to avoid memory leak
        first = head->next;
        ListNode* delNode;
        while (first != NULL) {
           delNode = first;
           first = first->next;
           delete delNode;
        }

        head->next = NULL;
        
        return retHead;
    }
};
