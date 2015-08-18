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
        
        // 简单这样处理有内存泄露啊
        head->next = NULL;
        
        // 下面的代码运行时出错，没有调试，暂时不知道错在哪里
        // 有可能是因为 ListNode* 是 malloc 出来的，所以不能用 delete?
        // first = head->next;
        // ListNode* delNode;
        // while (first != NULL) {
        //    delNode = first;
        //    first = first->next;
        //    delete delNode;
        // }
        
        return retHead;
    }
};
