# 49 Group Anagrams 
from collections import defaultdict

def groupAnagrams(strs: List[str]) -> List[List[str]]:
    ans = defaultdict(list)

    for s in strs:
        count = [0] * 26
        for c in s:
            count[ord(c) - ord('a')] += 1

        # List are mutable, therefore cannot be hashed, but tuples can
        ans[tuple(count)].append(s)

    return ans.values()

