from puzzle import PuzzleContext
from time import perf_counter

MOD = 1_000_000_007

def mul(x, y):
    n = len(x)
    m = len(y[0])
    ans = [[0]*m for _ in range(n)]
    for i in range(n):
        for j in range(m):
            for k in range(len(y)):
                ans[i][j] += x[i][k] * y[k][j]
                ans[i][j] %= MOD
    return ans

def unit(n, m):
    return [[int(i == j) for j in range(m)] for i in range(n)]

def pwr(a, e):
    if e == 0:
        return unit(len(a), len(a[0]))
    if e % 2 == 1:
        return mul(a, pwr(a, e-1))
    h = pwr(a, e//2)
    return mul(h, h)


def solve(initial_timers, n_days):
    M = [
        [0, 0, 0, 0, 0, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 0],
    ]
    counts = [[0]*9]
    for t in initial_timers:
        counts[0][t] += 1

    final_counts = mul(counts, pwr(M, n_days))
    return sum(final_counts[0]) % MOD

with PuzzleContext(year=2021, day=6) as ctx:
    arr = [int(x) for x in ctx.data.split(",")]

    t_start = perf_counter()
    print(solve(arr, 1_000_000_000_000_000_000))
    t_end = perf_counter()

    duration = t_end - t_start
    print(f"Took {duration} seconds")
