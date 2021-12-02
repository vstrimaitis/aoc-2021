from typing import Tuple
from functools import reduce
from puzzle import PuzzleContext

def apply_1(acc: Tuple[int, int], el: Tuple[str, int]) -> Tuple[int, int]:
    op, d = el
    deltas = {
        "down": (0, d),
        "up": (0, -d),
        "forward": (d, 0)
    }
    return tuple(map(sum, zip(acc, deltas[op])))

def apply_2(acc: Tuple[int, int, int], el: Tuple[str, int]) -> Tuple[int, int, int]:
    op, d = el
    deltas = {
        "down": (0, 0, d),
        "up": (0, 0, -d),
        "forward": (d, acc[2]*d, 0)
    }
    return tuple(map(sum, zip(acc, deltas[op])))

def parse(line: str) -> Tuple[str, int]:
    parts = line.split(" ")
    return parts[0], int(parts[1])

with PuzzleContext(year=2021, day=2) as ctx:
    arr = [parse(x) for x in ctx.nonempty_lines]
    x1, y1 = reduce(apply_1, arr, (0, 0))
    x2, y2, _ = reduce(apply_2, arr, (0, 0, 0))
    ctx.submit(1, x1*y1)
    ctx.submit(2, x2*y2)
