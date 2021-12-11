from typing import Generator, List, Set, Tuple
from puzzle import PuzzleContext

Board = List[List[int]]
Coord = Tuple[int, int]
CoordGen = Generator[Coord, None, None]

def size(g: Board) -> Coord:
    return len(g), len(g[0])

def neighs(g: Board, i: int, j: int) -> CoordGen:
    n, m = size(g)
    for di in [-1, 0, 1]:
        for dj in [-1, 0, 1]:
            if di == 0 and dj == 0:
                continue
            ii = i+di
            jj = j+dj
            if 0 <= ii < n and 0 <= jj < m:
                yield ii, jj

def indices(g: Board) -> CoordGen:
    n, m = size(g)
    for i in range(n):
        for j in range(m):
            yield i, j

def flash(board: Board, i: int, j: int, flashed: Set[int]):
    if (i, j) in flashed:
        return
    flashed.add((i, j))
    for ii, jj in neighs(board, i, j):
        board[ii][jj] += 1
        if board[ii][jj] > 9:
            flash(board, ii, jj, flashed)

def simulate_step(g: Board) -> int:
    I = list(indices(g))
    for i, j in I:
        g[i][j] += 1
    
    flashed = set()
    for i, j in I:
        if g[i][j] > 9:
            flash(g, i, j, flashed)
    
    for i, j in flashed:
        g[i][j] = 0

    return len(flashed)

with PuzzleContext(year=2021, day=11) as ctx:
    board = [[int(x) for x in r] for r in ctx.nonempty_lines]
    board_size = len(board)*len(board[0])
    ans1, ans2, step = 0, None, 0
    while not (step > 100 and ans2 is not None):
        step += 1
        n_flashed = simulate_step(board)
        if step <= 100:
            ans1 += n_flashed
        if n_flashed == board_size:
            ans2 = step
    
    ctx.submit(1, ans1)
    ctx.submit(2, ans2)
