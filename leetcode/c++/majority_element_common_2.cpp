// https://leetcode.com/problems/majority-element/
// 48 ms
class Solution {
public:
    int majorityElement(vector<int>& nums) {
        map<int, int> int_map;

        int num;
        int half = nums.size() / 2;
        for (int i = 0; i < nums.size(); i++) {
            num = nums[i];
            if (int_map.find(num) == int_map.end()) {
                int_map[num] = 1;
            } else {
                int_map[num]++;
            }
            if (int_map[num] > half) {
                return num;
            }
        }

        return 0;
    }
};