# 217. Contains Duplicate 

# Input: nums = [1,2,3,1]
# Output: true

class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        hashset = set()

        for n in nums:
            if n in hashset:
                return True
            hashset.add(n);

        return False
