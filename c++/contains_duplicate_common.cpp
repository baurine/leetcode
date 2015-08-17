// https://leetcode.com/problems/contains-duplicate/
// 常规解法
class Solution {
public:
    bool containsDuplicate(vector<int>& nums) {
        map<int, int> nums_map;
        
        for (int i = 0; i < nums.size(); i++) {
            if (nums_map.find(nums[i]) != nums_map.end()) {
                return true;
            }
            nums_map[nums[i]] = 1;
        }
        return false;
    }
};