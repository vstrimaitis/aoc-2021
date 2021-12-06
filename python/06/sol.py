from functools import lru_cache
from puzzle import PuzzleContext

@lru_cache
def calc_contribution(initial_count, n_days):
    ans = 1
    first_spawn_on = initial_count + 1
    for i in range(first_spawn_on, n_days+1, 7):
        ans += calc_contribution(8, n_days-i)
    return ans

def solve(initial_counts, n_days):
    ans = 0
    for x in initial_counts:
        ans += calc_contribution(x, n_days)
    return ans

with PuzzleContext(year=2021, day=6) as ctx:
    arr = [int(x) for x in ctx.data.split(",")]
    ctx.submit(1, solve(arr, 80))
    ctx.submit(2, solve(arr, 256))
