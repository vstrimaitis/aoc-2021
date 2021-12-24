from collections import *
from typing import *
from heapq import *
import z3
from puzzle import PuzzleContext

def parse_line(l: str) -> str:
    if "#" in l:
        l = l.split("#")[0].strip()
    if not l:
        return l
    return l.split(" ")

with PuzzleContext(year=2021, day=24) as ctx:
    program = [l for l in map(parse_line, ctx.lines) if l]

    solver = z3.Optimize()
    digits = [z3.BitVec(f"d_{i}", 64) for i in range(14)]
    for d in digits:
        solver.add(d > 0)
        solver.add(d < 10)
    zero = z3.BitVecVal(0, 64)
    one = z3.BitVecVal(1, 64)
    input_digits = digits.copy()

    mem = {
        "x": zero,
        "y": zero,
        "z": zero,
        "w": zero,
    }

    for i, cmd in enumerate(program):
        if cmd[0] == "inp":
            mem[cmd[1]] = input_digits[0]
            input_digits = input_digits[1:]
            continue
        a, b = cmd[1:]
        b = mem[b] if b in "xyzw" else int(b)
        res = z3.BitVec(f"res_{i}", 64)
        if cmd[0] == "add":
            solver.add(res == mem[a]+b)
        elif cmd[0] == "mul":
            solver.add(res == mem[a]*b)
        elif cmd[0] == "div":
            solver.add(res == mem[a]/b)
        elif cmd[0] == "mod":
            solver.add(res == mem[a]%b)
        elif cmd[0] == "eql":
            solver.add(res == z3.If(mem[a] == b, one, zero))
        else:
            assert False, "invalid command"
        mem[a] = res

    solver.add(mem["z"] == 0)

    model_number = zero
    for d in digits:
        model_number = 10*model_number + d
    print("Part 1:")
    solver.push()
    solver.maximize(model_number)
    print(solver.check())
    m = solver.model()
    ctx.submit(1, "".join([str(m[d]) for d in digits]))
    solver.pop()

    print("Part 2:")
    solver.push()
    solver.minimize(model_number)
    print(solver.check())
    m = solver.model()
    ctx.submit(2, "".join([str(m[d]) for d in digits]))
    solver.pop()
