from math import ceil, floor
from random import randint


def calc_fuel_usage(positions, target_position, cost_fn):
    distances = [abs(x-target_position) for x in positions]
    return sum(cost_fn(d) for d in distances)

def solve1(arr):
    possible_targets = list(range(min(arr), max(arr)+1))
    return min(calc_fuel_usage(arr, p, lambda d: d) for p in possible_targets)

def solve1_median(arr):
    p = list(sorted(arr))[len(arr)//2]
    return calc_fuel_usage(arr, p, lambda d: d)

def solve2(arr):
    possible_targets = list(range(min(arr), max(arr)+1))
    return min(calc_fuel_usage(arr, p, lambda d: d*(d+1)//2) for p in possible_targets)

def solve2_mean(arr):
    avg = sum(arr) / len(arr)
    target1 = floor(avg)
    target2 = ceil(avg)
    return min(
        calc_fuel_usage(arr, target1, lambda d: d*(d+1)//2),
        calc_fuel_usage(arr, target2, lambda d: d*(d+1)//2)
    )

if __name__ == "__main__":
    NUM_TESTS = 1000
    RANGE_N = (1, 1000)
    RANGE_POS = (1, 3000)

    for t in range(NUM_TESTS):
        n = randint(*RANGE_N)
        arr = [randint(*RANGE_POS) for _ in range(n)]
        ans1_expected = solve1(arr)
        ans1_actual = solve1_median(arr)
        ans2_expected = solve2(arr)
        ans2_actual = solve2_mean(arr)
        print(f"{t} ", end="")

        if ans1_expected != ans1_actual:
            print("WA", ",".join([str(x) for x in arr]))
            print(f"Expected: {ans1_expected}, got: {ans1_actual}")
            exit(0)
        else:
            print("OK ", end="")
        
        if ans2_expected != ans2_actual:
            print("WA", ",".join([str(x) for x in arr]))
            print(f"Expected: {ans2_expected}, got: {ans2_actual}")
            exit(0)
        else:
            print("OK")
