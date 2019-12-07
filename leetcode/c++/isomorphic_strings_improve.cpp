// https://leetcode.com/problems/isomorphic-strings/
// 优化到效率最高，使用两个 char 数组实现 hash table
// isomorphic 的两个规则：source 字符串中的相同字符，映射到 target 时字符也要相同；
// source 字符串中不同的字符，映射到 target 时字符不能相同。
class Solution {
public:
    bool isIsomorphic(string s, string t) {
        // source 是一个 hash 表，存储 key-value 映射关系，且 key = position
        // target 是一个位图 hash 表，用于存储 value 是否已经被映射
        char source[128] = {0}, target[128] = {0};
        char s_c, t_c;
        bool is_isomorphic = true;
        int len = s.size();

        for (int i=0; i<len; i++) {
            s_c = s[i];
            t_c = t[i];
            
            // 如果 s_c 作为 key 是第一次出现
            if (source[s_c] == 0) {
                // 再看 key 对应的 value 是不是也是第一次出现
                if (target[t_c] == 0) {
                    // 是第一次出现
                    target[t_c] = 1;
                    source[s_c] = t_c;
                } else {
                    // 说明 value 已经被其它 key 映射过了
                    is_isomorphic = false;
                    break;
                }
            } else if (source[s_c] != t_c) {
                // 新的 key-value 映射与之前的不相同
                is_isomorphic = false;
                break;
            }
        }

        return is_isomorphic;
    }
};