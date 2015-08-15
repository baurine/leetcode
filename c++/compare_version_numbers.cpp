// https://leetcode.com/problems/compare-version-numbers/
// 这道题是目前为止花费我最多时间的一题，字符比较就是麻烦
// 采用了递归的方法
class Solution {
public:
    int compareVersion(string version1, string version2) {
        if (version1.empty() && version2.empty()) {
            return 0;
        }
        
        string str1, str2;
        int pos1 = version1.find_first_of(".");
        str1 = version1.substr(0, pos1); // 当 version1 为 "" 时，substr(0, pos1) 不会出错
        int pos2 = version2.find_first_of(".");
        str2 = version2.substr(0, pos2);
        
        int ret = compareString(str1, str2);
        if (ret == 0) {
            if (pos1 == string::npos && pos2 == string::npos) {
                return 0;
            } else if (pos1 != string::npos && pos2 != string::npos) {
                return compareVersion(version1.substr(pos1 + 1), version2.substr(pos2 + 1));    
            } else if (pos1 == string::npos) {
                return compareVersion("", version2.substr(pos2 + 1));
            } else {
                return compareVersion(version1.substr(pos1 + 1), "");
            }
        }
        
        return ret;
    }
    
    int compareString(string str1, string str2) {
        int delta = simpleAtoi(str1) - simpleAtoi(str2);
        if (delta > 0) {
            return 1;
        } else if (delta < 0) {
            return -1;
        } else {
            return 0;
        }
    }
    
    int simpleAtoi(string str) {
        int out = 0;
        for (int i = 0; i < str.size(); i++) {
            out = out * 10 + (str[i] - '0');
        }
        return out;
    }
};
