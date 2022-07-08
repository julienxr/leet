#include <stdio.h>
#include <stdlib.h>
#include <string.h>


int length_of_longest_substring(char* string) {
  // Map to find if the character(index) has occurred, value of index is last
  // index of occurence
  int map[96];
  memset(map, -1, sizeof(map));

  int idx         = 0;
  int max_length  = 0;
  int left_ptr    = 0;
  int length      = 0; 
  int offset      = 1;
  int right_ptr   = 0;

  while (string[idx] != 0) {
    if (map[string[idx]] == -1) 
    {
      map[string[idx]] = idx;
      right_ptr        = idx;
      length++;
    }
    else
    {
      // reset all character indices from existing_idx down to left_ptr back to -1
      int existing_idx = map[string[idx]];

      for ( int reset = left_ptr; reset <= existing_idx; reset++) {
        map[string[reset]] = -1;
      }

      // update the latest index
      map[string[idx]]  = idx;
      left_ptr          = existing_idx + offset;
      right_ptr         = idx;
      length            = right_ptr - left_ptr + offset;
    }

    idx++;
    if (length > max_length) { max_length = length; }
  }
  return max_length;
}
