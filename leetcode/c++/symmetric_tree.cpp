/**
 * https://leetcode.com/problems/symmetric-tree/
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
    bool isSymmetric(TreeNode* root) {
        if (root == NULL) {
            return true;
        }
        return isSymmetric(root->left, root->right);
    }
    
    bool isSymmetric(TreeNode* leftNode, TreeNode* rightNode) {
        if (leftNode == NULL && rightNode == NULL) {
            return true;
        }
        if (leftNode == NULL || rightNode == NULL) {
            return false;
        }
        if (leftNode->val != rightNode->val) {
            return false;
        }
        
        bool isSymmetricInner = isSymmetric(leftNode->right, rightNode->left);
        bool isSymmetricOuter = isSymmetric(leftNode->left, rightNode->right);
        
        return isSymmetricInner && isSymmetricOuter;
    }
};
