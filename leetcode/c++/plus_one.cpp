// https://leetcode.com/problems/plus-one/
class Solution {
public:
    vector<int> plusOne(vector<int>& digits) {
        vector<int> plus_one;
        
        int carry = 0;
        int val = 0;
        
        for (int i = digits.size() - 1; i >= 0; i--) {
            val = digits[i] + carry;
            carry = val > 9 ? 1 : 0;
            val = val > 9 ? val - 10 : val;
            
            plus_one.insert(plus_one.begin(), val);
        }
        if (carry > 0) {
            plus_one.insert(plus_one.begin(), 1);
        }
        
        return plus_one;
    }
};
