# Unbounded Knapsack Algorithm

## Problem Description

In the **Unbounded Knapsack** problem, we are given:

- `n` items, each with a weight `w[i]` and a value `v[i]`.
- A knapsack with a capacity `W`.

Unlike the classic 0/1 knapsack, we can take an unlimited number of each item. The goal is to determine the maximum
value we can achieve without exceeding the knapsack's capacity.

## Algorithm

The dynamic programming approach to solve this problem follows these steps:

1. Create a DP array `dp[]` where `dp[j]` represents the maximum value for a knapsack with capacity `j`.
2. Initialize `dp[0]` to 0, as a knapsack with 0 capacity can hold no items.
3. For each capacity `j` from 1 to `W`, check all items. If the item's weight is less than or equal to `j`, then we
   check if adding this item would increase the value in `dp[j]`.

The recursive relation for this problem is:

```
dp[j] = max(dp[j], dp[j - w[i]] + v[i]) for all items i
```

## Step-by-Step Visualization of the DP Table

Let’s say we have the following example:

- **Items**: `{weight: 1, value: 1}`, `{weight: 3, value: 4}`, `{weight: 4, value: 5}`
- **Knapsack capacity**: `W = 6`

### Initialization

We initialize the `dp` array with `W + 1` elements, all set to 0:

| Capacity | 0 | 1 | 2 | 3 | 4 | 5 | 6 |
|----------|---|---|---|---|---|---|---|
| dp       | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

### Iteration 1: Capacity 1

For capacity `1`, we can only take the item with weight `1`:

```python
dp[1] = max(dp[1], dp[1 - 1] + 1) = max(0, 0 + 1) = 1
```


| Capacity | 0  | 1  | 2  | 3  | 4  | 5  | 6  |
|----------|----|----|----|----|----|----|----|
| dp       | 0  | 1  | 0  | 0  | 0  | 0  | 0  |

### Iteration 2: Capacity 2

For capacity `2`, we can take two items of weight `1`:

```python
dp[2] = max(dp[2], dp[2 - 1] + 1) = max(0, 1 + 1) = 2
```

| Capacity | 0  | 1  | 2  | 3  | 4  | 5  | 6  |
|----------|----|----|----|----|----|----|----|
| dp       | 0  | 1  | 2  | 0  | 0  | 0  | 0  |

### Iteration 3: Capacity 3

For capacity `3`, we can either take three items of weight `1` or one item of weight `3` (value 4):

```python
dp[3] = max(dp[3], dp[3 - 1] + 1, dp[3 - 3] + 4) = max(0, 2 + 1, 0 + 4) = 4
```


| Capacity | 0  | 1  | 2  | 3  | 4  | 5  | 6  |
|----------|----|----|----|----|----|----|----|
| dp       | 0  | 1  | 2  | 4  | 0  | 0  | 0  |

### Iteration 4: Capacity 4

For capacity `4`, we can either take four items of weight `1` or one item of weight `3` and one of weight `1`:

```python
dp[4] = max(dp[4], dp[4 - 1] + 1, dp[4 - 3] + 4) = max(0, 4 + 1, 1 + 4) = 5
```


| Capacity | 0  | 1  | 2  | 3  | 4  | 5  | 6  |
|----------|----|----|----|----|----|----|----|
| dp       | 0  | 1  | 2  | 4  | 5  | 0  | 0  |

### Iteration 5: Capacity 5

For capacity `5`, we can either take five items of weight `1` or one item of weight `3` and two items of weight `1`:

```python
dp[5] = max(dp[5], dp[5 - 1] + 1, dp[5 - 3] + 4) = max(0, 5 + 1, 2 + 4) = 6
```

| Capacity | 0  | 1  | 2  | 3  | 4  | 5  | 6  |
|----------|----|----|----|----|----|----|----|
| dp       | 0  | 1  | 2  | 4  | 5  | 6  | 0  |

### Iteration 6: Capacity 6

For capacity `6`, we can either take six items of weight `1`, or two items of weight `3` (total value = 8):

```python
dp[6] = max(dp[6], dp[6 - 1] + 1, dp[6 - 3] + 4) = max(0, 6 + 1, 4 + 4) = 8
```

| Capacity | 0  | 1  | 2  | 3  | 4  | 5  | 6  |
|----------|----|----|----|----|----|----|----|
| dp       | 0  | 1  | 2  | 4  | 5  | 6  | 8  |

## Time Complexity

The time complexity of this algorithm is `O(n * W)`, where `n` is the number of items and `W` is the knapsack's capacity.

## Conclusion

The **Unbounded Knapsack** algorithm allows us to solve the problem where we can take unlimited quantities of each item. By using a dynamic programming approach, we efficiently compute the maximum value we can fit into a knapsack of given capacity.

