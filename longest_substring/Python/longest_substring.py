def lengthOfLongestSubstring(string: str) -> int:
    chars = [0] * 128
    left = 0
    length = 0
    right = 0

    while right < len(string):
        rightPtr = string[right]
        chars[ord(rightPtr)] += 1

        while chars[ord(rightPtr)] > 1:
            leftPtr = string[left]
            chars[ord(leftPtr)] -= 1
            left += 1

        length = max(length, right - left + 1)
        right += 1

    return length


def lengthOfLongestSubstringOptimized(string: str) -> int:
    ans = 0
    map = {}
    offset = 1
    max_len = 0

    for idx in range(len(string)):
        if string[idx] in map:
            max_len = max(map[string[idx]], max_len)
        
        map[string[idx]] = idx + offset
        ans = max(ans, idx - max_len + offset)

    return ans 


