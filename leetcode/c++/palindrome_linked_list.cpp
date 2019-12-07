// https://leetcode.com/problems/palindrome-linked-list/

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
public:
    // 思路：先遍历一遍 list，得到中间结点
    // 将后 1/2 的 list 进行逆序
    // 然后再对比前 1/2 与后 1/2 的 list 是否相等
    // 这里没有说明 head 是头结点还是头指针，按头指针处理
    bool isPalindrome(ListNode* head) {
        // 处理特殊情况
        if (head == NULL || head->next == NULL) {
            return true;
        }
        
        int len = 0;
        ListNode* p = head;
        // 计算长度
        while (p != NULL) {
            len++;
            p = p->next;
        }
        // 定位到中间结点
        len = len / 2 + len % 2;
        int i = 1;
        p = head;
        while (i < len) {
            p = p->next;
            i++;
        }
        reverseList(p);
        return isEqualOrPrefix(head, p->next);
    }
    
    // 逆序
    // 这里 head 是头结点，而非头指针
    void reverseList(ListNode* head) {
        if (head == NULL || 
            head->next == NULL ||
            head->next->next == NULL) {
            return;
        }
        
        ListNode *p, *q;
        p = head->next;
        q = p->next;
        while (p->next != NULL) {
            p->next = q->next;
            q->next = head->next;
            head->next = q;
            q = p->next;
        }
    }
    
    // 判断两个 list 是否相等，
    // 或者一个 list 是另一个 list 的前缀
    // 这里 head 是头指针
    bool isEqualOrPrefix(ListNode* headFirst, ListNode* headSecond) {
        if (headFirst == NULL || headSecond == NULL) {
            return false;
        }
        while (headFirst != NULL && headSecond != NULL) {
            if (headFirst->val != headSecond->val) {
                return false;
            }
            headFirst = headFirst->next;
            headSecond = headSecond->next;
        }
        return true;
    }
};