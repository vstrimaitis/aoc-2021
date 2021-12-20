from collections import *
from typing import *
from heapq import *
from puzzle import PuzzleContext

Coord = Tuple[int, int]
Board = Dict[Coord, int]
Algorithm = List[int]


def to_int(c: str) -> int:
    return 0 if c == "." else 1


def enhance(board: Board, algo: Algorithm, default: int = 0) -> Board:
    ans = dict()

    Is = [i for (i, _) in board.keys()]
    Js = [j for (_, j) in board.keys()]
    min_i = min(Is) - 1
    max_i = max(Is) + 1
    min_j = min(Js) - 1
    max_j = max(Js) + 1

    for i in range(min_i, max_i + 1):
        for j in range(min_j, max_j + 1):
            idx = 0
            for di in [-1, 0, 1]:
                for dj in [-1, 0, 1]:
                    ii = i + di
                    jj = j + dj
                    c = board.get((ii, jj), default)
                    idx = 2 * idx + c
            ans[(i, j)] = algo[idx]
    return ans


def apply(board: Board, algo: Algorithm, n_times: int) -> Board:
    assert (algo[0], algo[-1]) in [(0, 1), (1, 0)]
    if algo[0] == 1:
        assert n_times % 2 == 0
    default = 0
    for _ in range(n_times):
        board = enhance(board, algo, default)
        if algo[0] == 1:
            default ^= 1
    return board


def count_lights(board: Board) -> int:
    return sum(board.values())


def parse(inp: str) -> Tuple[Board, Algorithm]:
    algo, board = inp.split("\n\n")
    algo = list(map(to_int, list(algo)))
    board = {
        (i, j): to_int(c)
        for i, line in enumerate(board.split("\n"))
        for j, c in enumerate(list(line))
    }
    return board, algo


with PuzzleContext(year=2021, day=20) as ctx:
    board, algo = parse(ctx.data)

    ans1 = count_lights(apply(board, algo, 2))
    ctx.submit(1, ans1)

    ans2 = count_lights(apply(board, algo, 50))
    ctx.submit(2, ans2)
