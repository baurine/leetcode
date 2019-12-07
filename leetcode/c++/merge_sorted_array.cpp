// https://leetcode.com/problems/merge-sorted-array/
class Solution {
public:
    void merge(vector<int>& nums1, int m, vector<int>& nums2, int n) {
        int i = m + n - 1;
        int j = m - 1;
        int k = n - 1;
        
        // 从后往前合并
        while (j >= 0 && k >= 0) {
            if (nums1[j] > nums2[k]) {
                nums1[i--] = nums1[j--];
            } else {
                nums1[i--] = nums2[k--];
            }
        }
        
        if (k >= 0) {
            while (i >= 0) {
                nums1[i--] = nums2[k--]; 
            }
        }
    }
};
