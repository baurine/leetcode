/**
 * https://leetcode.com/problems/string-to-integer-atoi/
 *
 * 这道看似简单的题却花费了我最多的时间，主要耗费在如何判断溢出的情况。
 */

#define INT_MAX 2147483647
#define INT_MIN -2147483648

class Solution {
public:
    int myAtoi(string str) {
        int sign = 1;
        int result = 0;
        int startPos = 0;

        if (str.empty()) {
            return 0;
        }

        // 跳过空格
        while (str[startPos] == ' ') {
            startPos++;
        }

        // 判断符号
        if (str[startPos] == '+') {
            sign = 1;
            startPos++;
        } else if (str[startPos] == '-') {
            sign = -1;
            startPos++;
        }

        // 正式开始转换
        for (int i=startPos; i<str.size(); i++) {
            if (str[i] >= '0' && str[i] <= '9') {
                int newVal = result * 10 + (str[i] - '0');
                // 一旦溢出就停止
                if (newVal < 0) {
                    result = -1;
                    break;
                } else if (newVal/10 < result) {
                    result = -1;
                    break;
                } else {
                    result = newVal;
                }
            } else {
                break;
            }
        }

        // 处理溢出
        if (result < 0) {
            result = sign > 0 ? INT_MAX : INT_MIN;
        } else {
            result = sign*result;
        }

        return result;
    }
};