# 0/1 Knapsack Algorithm

## Introduction

The **0/1 Knapsack Problem** is a classic optimization problem in computer science and operations research. Given a set
of items, each with a weight and a value, the objective is to determine the number of each item to include in a
collection such that the total weight is less than or equal to a given limit, and the total value is maximized. Unlike
the fractional knapsack problem, items cannot be divided; they are either taken (1) or left (0).

## Problem Statement

Given:

- A list of `n` items, each with a weight `w[i]` and a value `v[i]`.
- A knapsack with a maximum weight capacity `W`.

Find the maximum total value that can be obtained by selecting items such that the sum of their weights does not exceed
`W`.

### Constraints

- Each item can either be included (1) or excluded (0).
- You cannot take a fraction of an item.

## Dynamic Programming Approach

### Step-by-step Explanation

1. **Define a DP Table**: Create a 2D array `dp` where `dp[i][j]` represents the maximum value achievable with the first
   `i` items and a weight limit `j`.
2. **Initialization**: Set `dp[0][j] = 0` for all `j` because if there are no items, the total value is 0.
3. **Filling the DP Table**:
    - For each item `i` from `1` to `n` and for each weight `j` from `0` to `W`:
        - If the item's weight `w[i-1]` is less than or equal to `j`, choose the maximum of:
            - Not taking the item: `dp[i-1][j]`
            - Taking the item: `v[i-1] + dp[i-1][j - w[i-1]]`
        - If `w[i-1] > j`, you cannot take the item: `dp[i][j] = dp[i-1][j]`
4. The result will be in `dp[n][W]`.

### Illustration

Consider the following example:

```
weights = [1, 2, 3]
values = [10, 15, 40]
W = 6
```

1. Initialize the DP table:

| Item / Weight | 0 | 1 | 2 | 3 | 4 | 5 | 6 |
|---------------|---|---|---|---|---|---|---|
| 0             | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 1             | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 2             | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 3             | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

For filling the first item in the bag:

| Item / Weight | 0 | 1  | 2  | 3  | 4  | 5  | 6  |
|---------------|---|----|----|----|----|----|----|
| 0             | 0 | 0  | 0  | 0  | 0  | 0  | 0  |
| 1             | 0 | 10 | 10 | 10 | 10 | 10 | 10 |
| 2             | 0 | 0  | 0  | 0  | 0  | 0  | 0  |
| 3             | 0 | 0  | 0  | 0  | 0  | 0  | 0  |

For filling the second item in the bag:

| Item / Weight | 0 | 1  | 2  | 3  | 4  | 5  | 6  |
|---------------|---|----|----|----|----|----|----|
| 0             | 0 | 0  | 0  | 0  | 0  | 0  | 0  |
| 1             | 0 | 10 | 10 | 10 | 10 | 10 | 10 |
| 2             | 0 | 10 | 15 | 25 | 25 | 25 | 25 |
| 3             | 0 | 0  | 0  | 0  | 0  | 0  | 0  |

For filling the third item in the bag:

| Item / Weight | 0 | 1  | 2  | 3  | 4  | 5  | 6  |
|---------------|---|----|----|----|----|----|----|
| 0             | 0 | 0  | 0  | 0  | 0  | 0  | 0  |
| 1             | 0 | 10 | 10 | 10 | 10 | 10 | 10 |
| 2             | 0 | 10 | 15 | 25 | 25 | 25 | 25 |
| 3             | 0 | 10 | 15 | 40 | 50 | 55 | **65** |

### Pseudocode

```plaintext
function knapsack(values, weights, W):
    n = length(values)
    dp = array of size (n+1) x (W+1) initialized to 0
    
    for i from 1 to n:
        for j from 0 to W:
            if weights[i-1] <= j:
                dp[i][j] = max(dp[i-1][j], values[i-1] + dp[i-1][j - weights[i-1]])
            else:
                dp[i][j] = dp[i-1][j]
    
    return dp[n][W]
```
