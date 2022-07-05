double
findMedianSortedArray(int* nums1, int nums1_size, int* nums2, int nums2_size) 
{
  int nums3_size  = nums1_size + nums2_size;
  double nums3[nums3_size];

  double median   = 0.0;
  int middle      = 0;

  int idx = 0;
  int jdx = 0;
  int kdx = 0;

  while (idx < nums1_size && jdx < nums2_size) {
    if (nums1[idx] < nums2[jdx]) { nums3[kdx++] = nums1[idx++]; }
    else { nums3[kdx++] = nums2[jdx++]; }
  }

  for (;idx < nums1_size; idx++) { nums3[kdx++] = nums1[idx]; }
  for (;jdx < nums2_size; jdx++) { nums3[kdx++] = nums2[jdx]; }

  if (nums3_size % 2 == 0) 
  {
    middle = nums3_size / 2;
    median = (nums3[middle] + nums3[middle - 1]) / 2;
    return median;
  }
  else 
  {
    median = nums3[middle];
    return median;
  }
  return -1;
}
