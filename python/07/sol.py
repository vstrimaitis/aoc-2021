from puzzle import PuzzleContext

def calc_fuel_usage(positions, target_position, cost_fn):
    distances = [abs(x-target_position) for x in positions]
    return sum(cost_fn(d) for d in distances)

with PuzzleContext(year=2021, day=7) as ctx:
    arr = [int(x) for x in ctx.data.split(",")]
    possible_targets = list(range(min(arr), max(arr)+1))
    ans1 = min(calc_fuel_usage(arr, p, lambda d: d) for p in possible_targets)
    ctx.submit(1, ans1)

    ans2 = min(calc_fuel_usage(arr, p, lambda d: d*(d+1)//2) for p in possible_targets)
    ctx.submit(2, ans2)
