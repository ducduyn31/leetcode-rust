# Kadane's Algorithm

Kadane's Algorithm is an efficient way to find the maximum sum of a contiguous subarray within a one-dimensional array
of numbers, which may contain both positive and negative numbers.

## Problem Statement

Given an array of integers (both positive and negative), find the contiguous subarray (containing at least one number)
which has the largest sum and return that sum.

### Example

**Input:** `[-2, 1, -3, 4, -1, 2, 1, -5, 4]`  
**Output:** `6`  
**Explanation:** The subarray `[4, -1, 2, 1]` has the largest sum = 6.

## Algorithm Explanation

Kadane's Algorithm uses a dynamic programming approach. It maintains a running sum of the maximum subarray ending at
each position and updates the overall maximum as needed.

### Steps:

1. Initialize two variables:
    - `max_current` to keep track of the maximum sum of the subarray ending at the current index.
    - `max_global` to keep track of the overall maximum sum found so far.
2. Iterate through the array:
    - For each element, update `max_current` as the maximum of the current element or the sum of `max_current` and the
      current element.
    - Update `max_global` if `max_current` is greater.
3. Return `max_global` as the result.

### Pseudocode

```
function kadane(arr): 
    max_current = arr[0] 
    max_global = arr[0]
    
    for i = 1 to arr.length - 1:
        max_current = max(arr[i], max_current + arr[i])
        max_global = max(max_global, max_current)
    return max_global
```

### Edge Cases

- If the array contains all negative numbers, the algorithm will return the maximum negative number.
  Example: `[-1, -2, -3, -4]` will return `-1`.
- If the array contains all positive numbers, the algorithm will return the sum of all elements.
  Example: `[1, 2, 3, 4]` will return `10`.

## Complexity Analysis
- Time complexity: `O(n)` - We iterate through the array once.
- Space complexity: `O(1)` - We use only a constant amount of extra space.

