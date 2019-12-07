// https://leetcode.com/problems/length-of-last-word/
class Solution {
public:
    // 思路：从尾部向头部遍历
    // 先找到第一个不是空格的位置
    // 从刚才的位置继续找第一个空格的位置
    int lengthOfLastWord(string s) {
        int i, j;
        for (i=s.size()-1; i>=0; i--) {
            if (s[i] != ' ') {
                break;
            }
        }
        for (j=i; j>=0; j--) {
            if (s[j] == ' ') {
                break;
            }
        }
        return i-j;
    }
};