// https://leetcode.com/problems/minimum-depth-of-binary-tree/

/**
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
    int minDepth(TreeNode* root) {
        return minDepth(root, 0);
    }

    int minDepth(TreeNode* t, int depth) {
        if (t == NULL)
            return depth;

        if (t->left == NULL && t->right != NULL)
            return minDepth(t->right, depth+1);

        if (t->right == NULL && t->left != NULL)
            return minDepth(t->left, depth+1);

        int leftMin = minDepth(t->left, depth + 1);
        int rightMin = minDepth(t->right, depth + 1);

        return leftMin > rightMin ? rightMin : leftMin;
    }
};