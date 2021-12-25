from collections import *
from typing import *
from heapq import *
from puzzle import PuzzleContext

def move(grid, c, d):
    new_grid = [list(s) for s in grid]
    n = len(grid)
    m = len(grid[0])
    for i in range(n):
        for j in range(m):
            if grid[i][j] != c:
                continue
            ii = (i + d[0])%n
            jj = (j + d[1])%m
            if grid[ii][jj] == ".":
                new_grid[i][j] = "."
                new_grid[ii][jj] = c
    return ["".join(r) for r in new_grid]

with PuzzleContext(year=2021, day=25) as ctx:
    grid = ctx.nonempty_lines
    ans1 = 0
    while True:
        ans1 += 1
        old_grid = grid.copy()
        grid = move(grid, ">", (0, 1))
        grid = move(grid, "v", (1, 0))
        if grid == old_grid:
            break
    ctx.submit(1, ans1)
    ctx.submit(2, None)
