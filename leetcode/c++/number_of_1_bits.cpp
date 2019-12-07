// https://leetcode.com/problems/number-of-1-bits/
class Solution {
public:
    int hammingWeight(uint32_t n) {
        int cnt = 0;
        while (n > 0) {
            cnt += (n & 0x1);
            n = n >> 1;
        }
        return cnt;
    }
};