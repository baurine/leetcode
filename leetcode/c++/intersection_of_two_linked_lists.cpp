/**
 * https://leetcode.com/problems/intersection-of-two-linked-lists/
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
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        // 处理特殊情况
        if (headA == NULL || headB == NULL) {
            return NULL;
        }
        
        // 计算两个链表的长度
        int lenA = 0;
        int lenB = 0;
        ListNode *tmpHeadA = headA;
        ListNode *tmpHeadB = headB;
        while (tmpHeadA != NULL) {
            lenA++;
            tmpHeadA = tmpHeadA->next;
        }
        tmpHeadA = headA;
        while (tmpHeadB != NULL) {
            lenB++;
            tmpHeadB = tmpHeadB->next;
        }
        tmpHeadB = headB;
        
        // 把长链表定位到与短链表长度一致的节点
        int delta = 0;
        if (lenA > lenB) {
            delta = lenA - lenB;
            while (delta > 0) {
                tmpHeadA = tmpHeadA->next;
                delta--;
            }
        } else if (lenA < lenB) {
            delta = lenB - lenA;
            while (delta > 0) {
                tmpHeadB = tmpHeadB->next;
                delta--;
            }
        }
        
        // 开始比较
        while (tmpHeadA != NULL) {
            if (tmpHeadA == tmpHeadB) {
                break;
            }
            tmpHeadA = tmpHeadA->next;
            tmpHeadB = tmpHeadB->next;
        }
        
        return tmpHeadA;
    }
};