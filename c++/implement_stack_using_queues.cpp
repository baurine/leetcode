// https://leetcode.com/problems/implement-stack-using-queues/

class Stack {
private:
    list<int> queue_list;

public:
    // Push element x onto stack.
    void push(int x) {
        int size = queue_list.size();
        queue_list.push_back(x);
        while (size-- > 0) {
            queue_list.push_back(queue_list.front());
            queue_list.pop_front();
        }
    }

    // Removes the element on top of the stack.
    void pop() {
        queue_list.pop_front();
    }

    // Get the top element.
    int top() {
        return queue_list.front();
    }

    // Return whether the stack is empty.
    bool empty() {
        return queue_list.empty();
    }
};