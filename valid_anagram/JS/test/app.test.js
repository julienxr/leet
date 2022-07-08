const longestSubstring = require("./longest_substring");
const { Anagram_Hash, Anagram_Sort } = require("./anagram");
const {
  group_anagrams_object,
  group_anagrams_map,
} = require("./group_anagrams");

// Longest Substring Without Repeating Characters
describe("longestSubstring", () => {
  it("should return with the length of 3", () => {
    const string1 = "abcabcbb";
    expect(longestSubstring(string1)).toBe(3);
  });

  it("should return with the length of 1", () => {
    const string2 = "bbbbb";
    expect(longestSubstring(string2)).toBe(1);
  });

  it("should return with the length of 3", () => {
    const string3 = "pwwkew";
    expect(longestSubstring(string3)).toBe(3);
  });
});

// Are they anagrams
describe("isAnagram using 'Anagram_Hash' implementation.", () => {
  it("should return true", () => {
    const word1 = "anagram";
    const word2 = "nagaram";
    expect(Anagram_Hash(word1, word2)).toBeTruthy();
  });
  it("should return false", () => {
    const word1 = "rat";
    const word2 = "car";
    expect(Anagram_Hash(word1, word2)).toBeFalsy();
  });
});

describe("isAnagram using 'Anagram_Sort' implementation.", () => {
  it("should return true", () => {
    const word1 = "anagram";
    const word2 = "nagaram";
    expect(Anagram_Sort(word1, word2)).toBeTruthy();
  });
  it("should return false", () => {
    const word1 = "rat";
    const word2 = "car";
    expect(Anagram_Sort(word1, word2)).toBeFalsy();
  });
});

describe("Grouping of anagrams, return in any order.", () => {
  it("should equal provided array using Object", () => {
    const input = ["eat", "tea", "tan", "ate", "nat", "bat"];
    const output = [["bat"], ["nat", "tan"], ["ate", "eat", "tea"]];
    const sorted_anagrams_w_object = group_anagrams_object(input).map((group) =>
      group.sort()
    );
    expect(sorted_anagrams_w_object).toEqual(expect.arrayContaining(output));
  });
  it("should equal provided array using Map", () => {
    const input = ["eat", "tea", "tan", "ate", "nat", "bat"];
    const output = [["bat"], ["nat", "tan"], ["ate", "eat", "tea"]];
    const sorted_anagrams_w_map = group_anagrams_map(input).map((group) =>
      group.sort()
    );
    expect(sorted_anagrams_w_map).toEqual(expect.arrayContaining(output));
  });
});
