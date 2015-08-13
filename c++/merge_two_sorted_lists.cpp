/**
 * https://leetcode.com/problems/merge-two-sorted-lists/
 * 虽然题目中没有明说，经过实践，sorted list 排序是由小到大，
 * listNode 没有头结点
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
    ListNode* mergeTwoLists(ListNode* l1, ListNode* l2) {
        if (l1 == NULL && l2 == NULL) {
            return NULL;
        }
        if (l1 == NULL) {
            return l2;
        }
        if (l2 == NULL) {
            return l1;
        }
        
        ListNode *bigHead, *smallHead, *tmpHead, *retHead;
        if (l1->val > l2->val) {
            bigHead = l1;
            smallHead = l2;
        } else {
            bigHead = l2;
            smallHead = l1;
        }
        retHead = smallHead;
        
        while (smallHead != NULL && bigHead != NULL) {
            if (smallHead->next == NULL) {
                smallHead->next = bigHead;
                break;
            }
            if (smallHead->next->val < bigHead->val) {
                smallHead = smallHead->next;
            } else {
                tmpHead = bigHead;
                bigHead = bigHead->next;
                tmpHead->next = smallHead->next;
                smallHead->next = tmpHead;
                // smallHead = tmpHead->next; // wrong!
                smallHead = tmpHead;
            }
        }
        
        return retHead;
    }
};

