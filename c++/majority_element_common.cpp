// https://leetcode.com/problems/majority-element/
// 44 ms
class Solution {
public:
    int majorityElement(vector<int>& nums) {
        map<int, int> int_map;

        int num;
        for (int i = 0; i < nums.size(); i++) {
            num = nums[i];
            if (int_map.find(num) == int_map.end()) {
                int_map[num] = 1;
            } else {
                int_map[num]++;
            }
        }

        int half = nums.size() / 2;
        for (map<int, int>::const_iterator it = int_map.begin(); it != int_map.end(); it++) {
            if (it->second > half) {
                return it->first;
            }
        }

        return 0;
    }
};