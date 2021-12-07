from math import ceil, floor
from puzzle import PuzzleContext

def solve1(arr):
    p = list(sorted(arr))[len(arr)//2]
    return sum(abs(x-p) for x in arr)

def solve2(arr):
    avg = sum(arr) / len(arr)
    p1 = floor(avg)
    p2 = ceil(avg)
    return min(
        sum(abs(x-p1)*(abs(x-p1)+1)//2 for x in arr),
        sum(abs(x-p2)*(abs(x-p2)+1)//2 for x in arr)
    )

with PuzzleContext(year=2021, day=7) as ctx:
    arr = [int(x) for x in ctx.data.split(",")]
    ctx.submit(1, solve1(arr))
    ctx.submit(2, solve2(arr))
