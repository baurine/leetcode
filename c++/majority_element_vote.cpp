// https://leetcode.com/problems/majority-element/
// 20 ms
// Moore vote algorithm
class Solution {
public:
    int majorityElement(vector<int>& nums) {
        int candidate, counter;
        counter = 0;

        for (int i = 0; i < nums.size(); i++) {
            if (counter == 0) {
                candidate = nums[i];
                counter++;
            } else {
                if (candidate == nums[i]) {
                    counter++;
                } else {
                    counter--;
                }
            }
        }

        return candidate;
    }
};