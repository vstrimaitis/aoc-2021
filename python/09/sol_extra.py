from functools import reduce
from posixpath import split
from puzzle import PuzzleContext

def neighs(board, i, j):
    for di, dj in [(-1, 0), (0, 1), (1, 0), (0, -1)]:
        ii = i + di
        jj = j + dj
        if 0 <= ii < len(board) and 0 <= jj < len(board[0]):
            yield ii, jj

def calc_basin_size(board, i, j, assigned_components, curr_component):
    if curr_component in assigned_components[i][j]:
        return 0
    assigned_components[i][j].add(curr_component)
    return 1 + sum(
        calc_basin_size(board, ii, jj, assigned_components, curr_component)
        for ii, jj in neighs(board, i, j)
        if board[ii][jj] > board[i][j]
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
    # ctx.submit(1, sum(board[i][j]+1 for i, j in low_points))

    sizes = []
    assigned_components = [[set() for _ in range(m)] for _ in range(n)]
    next_component = 0
    for i, j in low_points:
        if next_component not in assigned_components[i][j]:
            calc_basin_size(board, i, j, assigned_components, next_component)
            next_component += 1
    split_points = [(i, j) for i in range(n) for j in range(m) if len(assigned_components[i][j]) > 1]
    print([assigned_components[i][j] for i, j in split_points])
    print(split_points)
    print("Part 2:", len(split_points))
