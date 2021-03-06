# 242. Valid Anagram
# Definition: An anagram is a word, phrase, or name formed by rearranging the
#             letters of another, such as cinema, formed from iceman.

def Anagram_Hash(s: str, t: str) -> bool:
    if len(s) != len(t):
        return False

    countS, countT = {}, {} 

    for i in range(len(s)):
        countS[s[i]] = countS.get(s[i], 0) + 1
        countT[t[i]] = countT.get(t[i], 0) + 1

    for c in countS:
        if countS[c] != countT.get(c, 0):
            return False

    return True


def Anagram_Sort(s: str, t: str) -> bool:
    return sorted(s) == sorted(t)

