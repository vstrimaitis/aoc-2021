from functools import reduce
from puzzle import PuzzleContext

def neighs(board, i, j):
    for di, dj in [(-1, 0), (0, 1), (1, 0), (0, -1)]:
        ii = i + di
        jj = j + dj
        if 0 <= ii < len(board) and 0 <= jj < len(board[0]):
            yield ii, jj

def calc_basin_size(board, i, j, visited):
    if board[i][j] == 9 or visited[i][j]:
        return 0
    visited[i][j] = True
    return 1 + sum(
        calc_basin_size(board, ii, jj, visited)
        for ii, jj in neighs(board, i, j)
    )

with PuzzleContext(year=2021, day=9) as ctx:
    board = [[int(x) for x in r] for r in ctx.nonempty_lines]
    n, m = len(board), len(board[0])
    low_points = [
        (i, j)
        for i in range(len(board))
        for j in range(len(board[0]))
        if all(
            board[i][j] < board[ii][jj]
            for ii, jj in neighs(board, i, j)
        )
    ]
    ctx.submit(1, sum(board[i][j]+1 for i, j in low_points))

    sizes = []
    visited = [[False for _ in range(m)] for _ in range(n)]
    sizes = [calc_basin_size(board, i, j, visited) for i, j in low_points]
    largest_sizes = list(sorted(sizes))[-3:]
    ctx.submit(2, reduce(lambda a, b: a*b, largest_sizes))
