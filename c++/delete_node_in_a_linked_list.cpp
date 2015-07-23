/**
 * https://leetcode.com/problems/delete-node-in-a-linked-list/
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
    // 思路：将 node 下一个节点的值赋给 node，然后删除 node 的下一个节点
    void deleteNode(ListNode* node) {
        if (node->next == NULL) {
            return;
        }
        
        ListNode* delNode = node->next;
        // 将下一个节点的值赋给当前 node
        node->val = delNode->val;
        // 删除下一个节点
        node->next = delNode->next;
        delNode->next = NULL;
        delete delNode;
    }
};