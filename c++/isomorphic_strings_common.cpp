// https://leetcode.com/problems/isomorphic-strings/
// 此法比较常规，效率不够高，有优化空间
class Solution {
private:
    map<char, char> char_map;
    // 用于存储映射后的字符集
    set<char> char_used;

public:
    bool isIsomorphic(string s, string t) {
        char s_c, t_c;
        bool is_isomorphic = true;
        for (int i=0; i<s.size(); i++) {
            s_c = s[i];
            t_c = t[i];
            if (char_map.find(s_c) == char_map.end()) {
                // 下面的逻辑用于检测是否有不同的字符映射到同一个字符上
                if (char_used.find(t_c) == char_used.end()) {
                    char_map[s_c] = t_c;
                    char_used.insert(t_c);
                } else {
                    is_isomorphic = false;
                    break;
                }
            } else if (t_c != char_map[s_c]) {
                is_isomorphic = false;
                break;
            }
        }
        return is_isomorphic;
    }
};