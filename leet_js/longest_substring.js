function lengthOfLongestSubstring(string) {
  let beginning = 0;
  let maxLength = 0;
  let map = new Map();

  for (let leftPtrIdx = beginning; leftPtrIdx < string.length; leftPtrIdx++) {
    let char = string[leftPtrIdx];

    if (map.get(char) >= beginning) {
      beginning = map.get(char) + 1;
    }
    map.set(char, leftPtrIdx);

    let currSubstringLength = leftPtrIdx - beginning + 1;
    maxLength = Math.max(maxLength, currSubstringLength);
  }
  return maxLength;
}

module.exports = lengthOfLongestSubstring;
