from typing import List, Callable
from puzzle import PuzzleContext

def get_most_common_bit(bitstrings: List[str], index: int) -> int:
    counts = [0, 0]
    for bs in bitstrings:
        counts[int(bs[index])] += 1
    if counts[0] == counts[1]:
        return -1
    return 0 if counts[0] > counts[1] else 1

def apply_filter(bitstrings: List[str], index: int, bit_criteria: Callable[[int], int]) -> List[str]:
    mcb = get_most_common_bit(bitstrings, index)
    target_bit = bit_criteria(mcb)
    return [bs for bs in bitstrings if int(bs[index]) == target_bit]

with PuzzleContext(year=2021, day=3) as ctx:
    arr = ctx.nonempty_lines
    b = len(arr[0])
    gamma, epsilon = "", ""
    for i in range(b):
        mcb = get_most_common_bit(arr, i)
        assert mcb != -1
        gamma += str(mcb)
        epsilon += str((mcb+1)%2)

    ctx.submit(1, int(gamma, 2) * int(epsilon, 2))

    ox, co2 = arr.copy(), arr.copy()
    for i in range(b):
        if len(ox) > 1:
            ox = apply_filter(ox, i, lambda mcb: 0 if mcb == 0 else 1)
        if len(co2) > 1:
            co2 = apply_filter(co2, i, lambda mcb: 1 if mcb == 0 else 0)
    
    ctx.submit(2, int(ox[0], 2) * int(co2[0], 2))
