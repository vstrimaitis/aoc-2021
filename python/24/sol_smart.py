from collections import *
from typing import *
from heapq import *
from puzzle import PuzzleContext

def parse_line(l: str) -> str:
    if "#" in l:
        l = l.split("#")[0].strip()
    if not l:
        return l
    return l.split(" ")

with PuzzleContext(year=2021, day=24) as ctx:
    program = [l for l in map(parse_line, ctx.lines) if l]
    
    stack = []
    pairs = dict()
    for i in range(14):
        # div z [1 | 26]
        op = "top" if program[i*18+4][-1] == "1" else "pop"
        # add x [c1]
        c1 = int(program[i*18+5][-1])
        # add y [c2]
        c2 = int(program[i*18+15][-1])
        if op == "top":
            stack.append((i, c1, c2))
        else:
            prev_i, prev_c1, prev_c2 = stack.pop()
            delta = prev_c2 + c1
            # d_{prev_i} + delta = d_i
            pairs[prev_i] = (delta, i)
    
    digits = [0]*14
    for i in range(14):
        if digits[i] > 0:
            continue
        delta, j = pairs[i]
        if delta < 0:
            digits[i] = 9
            digits[j] = digits[i] + delta
        else:
            digits[j] = 9
            digits[i] = digits[j] - delta
    ctx.submit(1, "".join(str(d) for d in digits))

    digits = [0]*14
    for i in range(14):
        if digits[i] > 0:
            continue
        delta, j = pairs[i]
        if delta < 0:
            digits[j] = 1
            digits[i] = digits[j] - delta
        else:
            digits[i] = 1
            digits[j] = digits[i] + delta
    ctx.submit(2, "".join(str(d) for d in digits))
