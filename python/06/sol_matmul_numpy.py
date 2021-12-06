import numpy as np
from puzzle import PuzzleContext

def solve(initial_timers, n_days):
    M = np.array([
        [0, 0, 0, 0, 0, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 0],
    ])
    counts = np.array([[0]*9])
    for t in initial_timers:
        counts[0][t] += 1

    final_counts = np.matmul(counts, np.linalg.matrix_power(M, n_days))
    return sum(final_counts[0])

with PuzzleContext(year=2021, day=6) as ctx:
    arr = [int(x) for x in ctx.data.split(",")]
    ctx.submit(1, solve(arr, 80))
    ctx.submit(2, solve(arr, 256))
