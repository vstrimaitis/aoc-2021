from collections import deque
from puzzle import PuzzleContext

def solve(initial_timers, n_days):
    counts = deque([0]*9)
    for t in initial_timers:
        counts[t] += 1
    for _ in range(n_days):
        counts.rotate(-1)
        counts[6] += counts[8]
    return sum(counts)

with PuzzleContext(year=2021, day=6) as ctx:
    arr = [int(x) for x in ctx.data.split(",")]
    ctx.submit(1, solve(arr, 80))
    ctx.submit(2, solve(arr, 256))
