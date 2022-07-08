# 49 Group Anagrams 

# Given an array of strings 'strs', group the anagrams together.
# You can return the answer in any order.

# An Anagram is a word or phrase formed by rearranging the letters of a
# different word or phrase, typically using all the original letters exactly
# once.

from collections import defaultdict
from typing import ValuesView, List

def groupAnagrams(strs: List[str]) -> ValuesView[List[str]]:
    ans = defaultdict(list)

    for s in strs:
        count = [0] * 26
        for c in s:
            count[ord(c) - ord('a')] += 1

        # List are mutable, therefore cannot be hashed, but tuples can
        ans[tuple(count)].append(s)

    return ans.values()

