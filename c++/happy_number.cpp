// https://leetcode.com/problems/happy-number/
class Solution {
public:
    bool isHappy(int n) {
        vector<int> vec;
        bool isHappy = true;

        vec.push_back(n);
        while (n != 1) {
            n = sumOfBitSquare(n);
            if (isDuplicate(vec, n)) {
                isHappy = false;
                break;
            } else {
                vec.push_back(n);
            }
        }

        return isHappy;
    }

    int sumOfBitSquare(int n) {
        int sum = 0;
        while (n > 0) {
            int mod = n % 10;
            sum += (mod * mod);
            n /= 10;
        }
        return sum;
    }

    bool isDuplicate(const vector<int>& vec, int k) {
        for (int i = 0; i < vec.size(); i++) {
            if (vec[i] == k) {
                return true;
            }
        }
        return false;
    }
};