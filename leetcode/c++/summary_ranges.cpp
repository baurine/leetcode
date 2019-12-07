// https://leetcode.com/problems/summary-ranges/

class Solution {
public:
    vector<string> summaryRanges(vector<int>& nums) {
        vector<string> summary_vec;
        if (nums.size() == 0) {
            return summary_vec;
        }

        int start, end;
        start = nums[0];
        end = start;
        for (int i=0; i<nums.size()-1; i++) {
            if (nums[i+1] == end + 1) {
                end = nums[i+1];
            } else {
                summary_vec.push_back(generateSummary(start, end));
                start = nums[i+1];
                end = start;
            }
        }
        summary_vec.push_back(generateSummary(start, end));
        return summary_vec;
    }

    string generateSummary(int start, int end) {
        char buf[100];
        if (start == end) {
            snprintf(buf, sizeof(buf), "%d", start);
        } else {
            snprintf(buf, sizeof(buf), "%d->%d", start, end);
        }
        return string(buf);
    }
};