// 49. Grouped Anagrams

function group_anagrams_object(strs) {
  let groups = {};

  strs.forEach((str) => {
    const sortedStr = str.split("").sort();
    if (groups[sortedStr]) {
      groups[sortedStr].push(str);
    } else {
      groups[sortedStr] = [str];
    }
  });
  return Object.values(groups);
}

function group_anagrams_map(strs) {
  let map = new Map();

  strs.forEach((str) => {
    // Must use "joined" string, as the split and sorted version yields an
    // Array that has a reference to memory, that ref is new everytime, hence
    // the Map Object wants to save an instance for every identical array
    const sortedStr = str.split("").sort().join();
    if (!map.has(sortedStr)) {
      map.set(sortedStr, [str]);
    } else {
      map.get(sortedStr).push(str);
    }
  });

  return [...map.values()];
}
module.exports = { group_anagrams_object, group_anagrams_map };
