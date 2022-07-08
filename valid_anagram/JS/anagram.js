// 242. Valid Anagram

const Anagram_Hash = (s, t) => {
  if (s.length != t.length) return false;

  let sArray = s.split("").sort();
  let tArray = t.split("").sort();

  let i = 0;

  while (i < s.length) {
    if (sArray[i] !== tArray[i]) return false;
    i++;
  }
  return true;
};

const Anagram_Sort = (s, t) => {
  let sArray = s
    .split("")
    .sort((a, b) => (a > b ? 1 : -1))
    .join();
  let tArray = t
    .split("")
    .sort((a, b) => (a > b ? 1 : -1))
    .join();

  return sArray === tArray;
};

module.exports = { Anagram_Hash, Anagram_Sort };
