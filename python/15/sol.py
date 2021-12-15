from collections import *
from typing import *
from heapq import heappush, heappop
from puzzle import PuzzleContext

INF = 10 ** 100
Board = List[List[int]]


def solve(arr: Board) -> int:
    n = len(arr)
    m = len(arr[0])
    Q = []
    dists = defaultdict(lambda: INF)
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
                    heappush(Q, (d + x, (ii, jj)))
    return dists[(n - 1, m - 1)]


def mod1(x: int, m: int) -> int:
    return (x - 1) % m + 1


def expand(arr: Board) -> Board:
    n = len(arr)
    m = len(arr[0])

    return [
        [mod1(arr[i % n][j % m] + i // n + j // m, 9) for j in range(5 * m)]
        for i in range(5 * n)
    ]


with PuzzleContext(year=2021, day=15) as ctx:
    arr = [[int(x) for x in list(r)] for r in ctx.nonempty_lines]

    ans1 = solve(arr)
    ctx.submit(1, ans1)

    ans2 = solve(expand(arr))
    ctx.submit(2, ans2)
