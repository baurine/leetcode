/**
 * https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/
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
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        TreeNode* ancestor = root;
        
        while (1) {
            int deltap = p->val - ancestor->val;
            int deltaq = q->val - ancestor->val;
            
            if ((deltap >= 0 && deltaq <= 0) || (deltap <= 0 && deltaq >= 0)) {
                break;
            }
            
            if (deltap > 0 && deltaq > 0) {
                ancestor = ancestor->right;
            }
            
            if (deltap < 0 && deltaq < 0) {
                ancestor = ancestor->left;
            }
        }
        
        return ancestor;
    }
};