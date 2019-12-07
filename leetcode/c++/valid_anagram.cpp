// https://leetcode.com/problems/valid-anagram/
class Solution {
public:
    bool isAnagram(string s, string t) {
        if (s.size() != t.size()) {
            return false;
        }
        
        char src[128] = {0}, target[128] = {0};
        for (int i = 0; i < s.size(); i++) {
            src[s[i]]++;
            target[t[i]]++;
        }
        
        for (int i = 0; i < 128; i++) {
            if (src[i] != target[i]) {
                return false;
            }
        }
        
        return true;
    }
};
