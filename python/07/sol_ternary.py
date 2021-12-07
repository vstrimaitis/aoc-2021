from puzzle import PuzzleContext

MODIFIER = 1

def solve1(arr):
    p = list(sorted(arr))[len(arr)//2]
    return sum(abs(x-p) for x in arr)

def solve2(arr):
    def calc_fuel_usage(target):
        return sum(abs(x-target)*(abs(x-target)+1)//2 for x in arr)
    lo = min(arr)
    hi = max(arr)
    while hi - lo > 1:
        mid = (hi + lo) // 2
        if calc_fuel_usage(mid) < calc_fuel_usage(mid+1):
            hi = mid
        else:
            lo = mid
    return calc_fuel_usage(lo+1)

with PuzzleContext(year=2021, day=7) as ctx:
    arr = [int(x)*MODIFIER for x in ctx.data.split(",")]
    ctx.submit(1, solve1(arr))
    ctx.submit(1, solve2(arr))
