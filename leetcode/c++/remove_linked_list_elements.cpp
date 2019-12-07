/**
 * https://leetcode.com/problems/remove-linked-list-elements/
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
    ListNode* removeElements(ListNode* head, int val) {
        if (head == NULL) {
            return head;
        }

        // 先处理 head 节点之后的所有节点
        ListNode* p = head;
        ListNode* q = p->next;
        while (q != NULL) {
            if (q->val == val) {
                p->next = q->next;
                q->next = NULL;
                delete q;
                q = p->next;
            } else {
                p = q;
                q = q->next;
            }
        }

        // 最后再回来处理 head 节点
        p = head;
        if (p->val == val) {
            head = p->next;
            p->next = NULL;
            delete p;
            p = NULL;
        }

        return head;
    }
};