/**
 * https://leetcode.com/problems/min-stack/
 *
 * 使用双栈法的简易解法
 */
class MinStack {
private:
    vector<int> v1, v2;

public:
    void push(int x) {
        v1.push_back(x);

        if (v2.empty() || x <= v2.back()) {
            v2.push_back(x);
        }
    }

    void pop() {
        if (v2.back() == v1.back()) {
            v2.pop_back();
        }
        
        v1.pop_back();
    }

    int top() {
        if (v1.empty()) {
            return 0;
        } else {
            return v1.back();
        }
    }

    int getMin() {
        if (v2.empty()) {
            return 0;
        } else {
            return v2.back();
        }
    }
};