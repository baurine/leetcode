/**
 * https://leetcode.com/problems/min-stack/
 *
 * 常规解法，只用了一个栈，稍麻烦
 * 看了 Discuss 后发现使用两个栈来实现会简单很多，就是需要额外的空间
 */
class MinStack {
private:
    vector<int> data;
    int min_index;

public:
    void push(int x) {
        data.push_back(x);

        if (data.size() == 1) {
            min_index = 0;
        } else  if (x < data[min_index]) {
            min_index = data.size() - 1;
        }
    }

    void pop() {
        if (data.empty()) {
            return;
        }

        if (data.size() == 1) {
            min_index = -1;
        } else if (data.size() == min_index + 1) {
            int min_val = data[0];
            min_index = 0;
            for (int i=0; i<data.size()-1; i++) {
                if (data[i] < min_val) {
                    min_val = data[i];
                    min_index = i;
                }
            }
        }

        data.pop_back();
    }

    int top() {
        if (data.empty()) {
            return 0;
        } else {
            return data.back();
        }
    }

    int getMin() {
        if (min_index == -1) {
            return 0;
        } else {
            return data[min_index];
        }
    }
};