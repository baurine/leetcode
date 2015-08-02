// https://leetcode.com/problems/valid-parentheses/
class Solution {
public:
    bool isValid(string s) {
        list<char> char_stack;
        
        for (int i = 0; i < s.size(); i++) {
            if (s[i] == '(' || s[i] == '{' || s[i] == '[') {
                char_stack.push_front(s[i]);
                continue;
            }
            if (s[i] == ')') {
                if (char_stack.front() != '(') {
                    char_stack.push_front(s[i]);
                    break;
                } else {
                    char_stack.pop_front();
                }
            } else if (s[i] == ']') {
                if (char_stack.front() != '[') {
                    char_stack.push_front(s[i]);
                    break;
                } else {
                    char_stack.pop_front();
                }
            } else if (s[i] == '}') {
                if (char_stack.front() != '{') {
                    char_stack.push_front(s[i]);
                    break;
                } else {
                    char_stack.pop_front();
                }
            }
        }
        
        return char_stack.empty();
    }
};
