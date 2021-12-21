from collections import *
from typing import *
from heapq import *
from puzzle import PuzzleContext
from functools import lru_cache


def mod1(x: int, m: int) -> int:
    return (x - 1) % m + 1


def roll(state: int) -> Tuple[int, int]:
    return state, mod1(state + 1, 100)


def solve1(starts: Tuple[int, int]) -> int:
    pos = list(starts)
    next_val = 1
    scores = [0, 0]
    turn = 0
    n_rolls = 0
    while True:
        total_rolled = 0
        for _ in range(3):
            val, next_val = roll(next_val)
            total_rolled += val
            n_rolls += 1
        pos[turn] = mod1(pos[turn] + total_rolled, 10)
        scores[turn] += pos[turn]
        if scores[turn] >= 1000:
            break
        turn ^= 1
    return n_rolls * min(scores)


@lru_cache(maxsize=None)
def dp(turn: int, s1: int, s2: int, p1: int, p2: int) -> Tuple[int, int]:
    if s1 >= 21:
        return (1, 0)
    if s2 >= 21:
        return (0, 1)

    p1_wins, p2_wins = 0, 0
    for r1 in [1, 2, 3]:
        for r2 in [1, 2, 3]:
            for r3 in [1, 2, 3]:
                if turn == 0:
                    pp1 = mod1(p1 + r1 + r2 + r3, 10)
                    ss1 = s1 + pp1
                    subans = dp(turn ^ 1, ss1, s2, pp1, p2)
                    p1_wins += subans[0]
                    p2_wins += subans[1]
                else:
                    pp2 = mod1(p2 + r1 + r2 + r3, 10)
                    ss2 = s2 + pp2
                    subans = dp(turn ^ 1, s1, ss2, p1, pp2)
                    p1_wins += subans[0]
                    p2_wins += subans[1]

    return p1_wins, p2_wins


def solve2(starts: Tuple[int, int]) -> int:
    n_wins_p1, n_wins_p2 = dp(0, 0, 0, starts[0], starts[1])
    return max(n_wins_p1, n_wins_p2)


with PuzzleContext(year=2021, day=21) as ctx:
    pos = tuple(int(line.split(": ")[1]) for line in ctx.nonempty_lines)
    ctx.submit(1, solve1(pos))
    ctx.submit(2, solve2(pos))
