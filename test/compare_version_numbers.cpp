#include <string>
using namespace std;

int compareString(string str1, string str2);
int simpleAtoi(string str);

int compareVersion(string version1, string version2) {
	if (version1.empty() && version2.empty()) {
		return 0;
	}

	string str1, str2;
	int pos1 = version1.find_first_of(".");
	printf("pos1 %d\n", pos1);
	str1 = version1.substr(0, pos1);
	int pos2 = version2.find_first_of(".");
	printf("pos2 %d\n", pos2);
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
	printf("str1 %s, str2 %s\n", str1.c_str(), str2.c_str());
	int delta = simpleAtoi(str1) - simpleAtoi(str2);
	printf("delta %d\n", delta);
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

int main() {
	// compareVersion("1.1.1", "1.1.1");
	// int ret = compareVersion("01", "1");
	int ret = compareVersion("1.0", "1");
	printf("ret=%d\n", ret);
	return 0;
}
