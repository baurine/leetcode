// https://leetcode.com/problems/reverse-bits/
class Solution {
public:
    uint32_t reverseBits(uint32_t n) {
        int retVal = 0;
        int bitCnt = 0;

        while (n > 0) {
            retVal = (retVal << 1) | (n & 0x1);
            n = n >> 1;
            bitCnt++;
        }

        retVal = retVal << (32-bitCnt);
        return retVal;
    }
};