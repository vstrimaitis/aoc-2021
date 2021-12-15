from collections import *
from heapq import heappush, heappop
from puzzle import PuzzleContext

def solve(arr):
    n = len(arr)
    m = len(arr[0])
    Q = []
    dists = defaultdict(lambda: 10**10)
    dists[(0, 0)] = 0
    heappush(Q, (0, (0, 0)))
    while Q:
        d, (i, j) = heappop(Q)
        for di, dj in [[-1, 0], [1, 0], [0, -1], [0, 1]]:
            ii = i + di
            jj = j + dj
            if 0 <= ii < n and 0 <= jj < m:
                x = arr[ii][jj]
                if d + x < dists[(ii, jj)]:
                    dists[(ii, jj)] = d + x
                    heappush(Q, (d+x, (ii, jj)))
    return dists[(n-1, m-1)]

def expand(arr):
    n = len(arr)
    m = len(arr[0])
    new_arr = [[None for _ in range(5*m)] for _ in range(5*n)]
    for i in range(5*n):
        for j in range(5*m):
            new_arr[i][j] = arr[i%n][j%m] + i//n + j//m
            while new_arr[i][j] > 9:
                new_arr[i][j] -= 9

    return new_arr

with PuzzleContext(year=2021, day=15) as ctx:
    arr = [[int(x) for x in list(r)] for r in ctx.nonempty_lines]
    
    ans1 = solve(arr)
    ctx.submit(1, ans1)

    ans2 = solve(expand(arr))
    ctx.submit(2, ans2)
