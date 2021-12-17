from collections import *
from typing import *
from heapq import *
from puzzle import PuzzleContext
import re
from math import sqrt, ceil, floor

BIG = 10 ** 6


def calc_coord(c0: int, v0: int, t: int, v_lower_bound: Optional[int] = None) -> int:
    # c = (c0) + v0 + (v0-1) + (v0-2) + (v0-3) + ... (v_t)
    v_t = v0 - t + 1
    if v_lower_bound is not None:
        v_t = max(v_t, v_lower_bound)
    delta = v0 * (v0 + 1) // 2 - (v_t - 1) * v_t // 2
    return c0 + delta


def will_hit_target(vx: int, vy: int, x1: int, x2: int, y1: int, y2: int) -> bool:
    tx1 = ceil(calc_time(vx, x1) or BIG)
    tx2 = floor(calc_time(vx, x2) or BIG)

    ty1 = ceil(calc_time(vy, y2) or BIG)
    ty2 = floor(calc_time(vy, y1) or BIG)

    t1 = max(tx1, ty1)
    t2 = min(tx2, ty2)

    for t in range(t1, t2 + 1):
        x = calc_coord(0, vx, t, 0)
        y = calc_coord(0, vy, t)
        if x1 <= x <= x2 and y1 <= y <= y2:
            return True
    return False


def calc_time(initial_vel: int, final_coord: int) -> Optional[float]:
    D = (1 + 2 * initial_vel) ** 2 - 8 * final_coord
    if D < 0:
        return None
    sD = sqrt(D)
    t1 = (1 + 2 * initial_vel - sD) / 2
    t2 = (1 + 2 * initial_vel + sD) / 2
    if t1 >= 0:
        return t1
    if t2 >= 0:
        return t2
    return None


with PuzzleContext(year=2021, day=17) as ctx:
    x1, x2, y1, y2 = tuple(map(int, re.findall(r"-?\d+", ctx.data)))

    ans1 = (-y1 - 1) * (-y1) // 2
    ctx.submit(1, ans1)

    ans2 = 0
    vx_min = ceil((sqrt(1 + 8 * x1) - 1) / 2)
    vx_max = x2
    for vx in range(vx_min, vx_max + 1):
        tx1 = ceil(calc_time(vx, x1) or BIG)
        tx2 = floor(calc_time(vx, x2) or BIG)

        vy1 = floor((y1 + tx1 * (tx1 - 1) / 2) / tx1)
        vy2 = ceil((y2 + tx2 * (tx2 - 1) / 2) / tx2)

        vy_min = max(y1, vy1)
        vy_max = min(-y1 - 1, vy2)

        for vy in range(vy_min, vy_max + 1):
            if will_hit_target(vx, vy, x1, x2, y1, y2):
                ans2 += 1
    ctx.submit(2, ans2)
