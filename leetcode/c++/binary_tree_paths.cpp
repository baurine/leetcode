/**
 * https://leetcode.com/problems/binary-tree-paths/
 * 
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    vector<string> binaryTreePaths(TreeNode* root) {
        vector<string> paths;
        
        binaryTreePaths(root, "", paths);
        
        return paths;
    }

private:
    void binaryTreePaths(TreeNode* node, string currentPath, vector<string>& allPaths) {
        if (node == NULL) {
            return;
        }
        
        char buf[10];
        sprintf(buf, "%d", node->val);
        if (currentPath.empty()) {
            currentPath.append(buf);
        } else {
            currentPath.append("->").append(buf);
        }
        
        if (node->left == NULL && node->right == NULL) {
            allPaths.push_back(currentPath);
        }
        if (node->left != NULL) {
            binaryTreePaths(node->left, currentPath, allPaths);
        }
        if (node->right != NULL) {
            binaryTreePaths(node->right, currentPath, allPaths);
        }
    }
};

