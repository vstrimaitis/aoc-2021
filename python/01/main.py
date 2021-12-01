from typing import List
from pathlib import Path

def find_increase_count(arr: List[int]) -> int:
    return sum([1 for (x1, x2) in zip(arr, arr[1:]) if x1 < x2])

if __name__ == "__main__":
    input_path = Path(__file__).parent.parent.parent / "inputs" / "01.txt"
    arr = [int(x) for x in input_path.read_text().split()]
    print("Part 1: ", find_increase_count(arr))
    windows = [sum(arr[i:i+3]) for i in range(len(arr)-2)]
    print("Part 2: ", find_increase_count(windows))
