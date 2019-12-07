// https://leetcode.com/problems/add-digits/
// 重点是总结出数学规律
class Solution {
public:
    int addDigits(int num) {
        return num < 10 ? num : (num - 10) % 9 + 1;
    }
};
