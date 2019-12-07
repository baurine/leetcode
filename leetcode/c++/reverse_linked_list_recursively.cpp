/**
 * https://leetcode.com/problems/reverse-linked-list/
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
    // 这里没有说明 head 是指头结点还是头指针，就按头指针处理吧 (实际还就是头指针)
    // 真心不能忍受指针的定义前后不一致
    // 前面定义时把 * 号靠近变量，这里定义函数时又把 * 号靠近类型
    // 使用了递归的方法，效率比迭代差远了 (wrong!)
    // 核心思想就是先把当前结点后面的 list 逆好序，然后再把这个结点放到逆好序的 list 的尾部。
    ListNode* reverseList(ListNode* head) {
        if (head == NULL || head->next == NULL) {
            return head;
        }

        ListNode* newHead = reverseList(head->next);

        /*
        ListNode* tmpNode = newHead;
        while (tmpNode->next != NULL) {
            tmpNode = tmpNode->next;
        }
        tmpNode->next = head;
        */
        // 上面的逻辑可以用下面这一句优化
        // 用这一句优化以后，可以达到和迭代法接近的效率
        head->next->next = head;
        head->next = NULL;

        return newHead;
    }
};