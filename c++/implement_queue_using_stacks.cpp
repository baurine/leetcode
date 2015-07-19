/**
 * https://leetcode.com/problems/implement-queue-using-stacks/
 *
 * 解题思路：用两个栈来实现
 */

class Queue {
private:
    std::list<int> stack, stack_tmp;
public:
    // Push element x to the back of queue.
    void push(int x) {
        while (!stack.empty()) {
            stack_tmp.push_front(stack.front());
            stack.pop_front();
        }
        stack.push_front(x);
        while (!stack_tmp.empty()) {
            stack.push_front(stack_tmp.front());
            stack_tmp.pop_front();
        }
    }

    // Removes the element from in front of queue.
    void pop(void) {
        stack.pop_front();
    }

    // Get the front element.
    int peek(void) {
        return stack.front();
    }

    // Return whether the queue is empty.
    bool empty(void) {
        return stack.empty();
    }
};