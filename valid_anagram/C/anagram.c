// 241. Valid Anagram

#include <stdio.h>
#include <stdbool.h>
#include <string.h>


bool is_anagram(char* s, char* t) 
{
  int s_length = strlen(s);
  int t_length = strlen(t);

  if (s_length != t_length) { return false;}

  int s_count[26] = {0};
  int t_count[26] = {0};


  for (int i = 0; i < s_length; i++) {
    int c = s[i] - 'a';
    printf("%c - 'a' = %d\n", s[i], c);
    s_count[c] += 1;
  }

  puts("\n");

  for (int i = 0; i < t_length; i++) {
    int c = t[i] - 'a';
    printf("%c - 'a' = %d\n", t[i], c);
    t_count[c] += 1;
  }


  for (int i = 0; i < 26; i++) {
    if (s_count[i] != t_count[i])
      return false;
  }

  return true;
}


int main(int argc, char* arg[]) 
{
  char* s1 = "xavier";
  char* s2 = "vxreai";

  is_anagram(s1, s2);

  return 0;
}
