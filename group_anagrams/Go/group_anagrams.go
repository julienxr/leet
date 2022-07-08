// 49. Grouped Anagrams
package leet_go
import (
  "strings";
  "sort";

)

func groupAnagrams(strs []string) [][]string {
  ret := make([][]string, 0)
  tmp := make(map[string][]string, 0)
  for _, str := range strs {
    org := str
    ss := strings.Split(str, "")
    sort.Strings(ss)
    str = strings.Join(ss, "")
    tmp[str] = append(tmp[str], org)
  }
  for _, s := range tmp {
    ret = append(ret, s)
  }
  return ret
}
