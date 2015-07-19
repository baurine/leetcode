// 思路：位操作，右移
class Solution {
public:
    bool isPowerOfTwo(int n) {
        if (n < 1) {
            return false;
        }

        while ((n & 0x1) == 0) {
            n = n >> 1;
        }
        return n == 1;
    }
};