// https://leetcode.com/problems/contains-duplicate/
// 看了 Discuss 后学习到的 unique() 算法
class Solution {
public:
    bool containsDuplicate(vector<int>& nums) {
        sort(nums.begin(), nums.end());
        return unique(nums.begin(), nums.end()) != nums.end();
    }
};