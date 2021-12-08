from collections import defaultdict
from typing import List, Callable, Dict
from puzzle import PuzzleContext

def solve1(codes: List[List[str]]) -> int:
    all_codes = sum(codes, []) # flattens the list of lists
    return len(list(filter(lambda s: len(s) in {2, 3, 4, 7}, all_codes)))

def solve2(signals: List[List[str]], codes: List[List[str]]) -> int:
    return sum(resolve_code(s, c) for s, c in zip(signals, codes))

def resolve_code(signals: List[str], codes: List[str]) -> int:
    """Resolution logic.

    Known immediately: 1, 7, 4, 8
    Can be deduced:
    - 3 (the only signal with 5 segments that is a superset of 1)
    - 9 (the only signal with 6 segments that is a superset of 4)
    - 0 (the only signal with 6 segments left that is a superset of 1)
    - 6 (the only signal with 6 segments left)
    - 5 (the only signal with 5 segments left that is a subset of 9)
    - 2 (the only signal left)
    """
    signals_by_length = group_by_length(signals)
    digits = dict()
    digits["1"] = signals_by_length[2][0]
    digits["7"] = signals_by_length[3][0]
    digits["4"] = signals_by_length[4][0]
    digits["8"] = signals_by_length[7][0]

    digits["3"] = pick(signals_by_length[5], lambda s: is_subset(digits["1"], s))
    digits["9"] = pick(signals_by_length[6], lambda s: is_subset(digits["4"], s))
    digits["0"] = pick(signals_by_length[6], lambda s: is_subset(digits["1"], s))
    digits["6"] = signals_by_length[6][0]
    digits["5"] = pick(signals_by_length[5], lambda s: is_subset(s, digits["9"]))
    digits["2"] = signals_by_length[5][0]

    signal_to_digit = {v: k for k, v in digits.items()}
    return int("".join(signal_to_digit[s] for s in codes))

def is_subset(s1: str, s2: str) -> bool:
    return all(c in s2 for c in s1)

def pick(signals: List[str], predicate_fn: Callable[[str], bool]) -> None:
    ans = [s for s in signals if predicate_fn(s)][0]
    signals.remove(ans)
    return ans

def group_by_length(strings: List[str]) -> Dict[str, List[str]]:
    by_length = defaultdict(list)
    for s in strings:
        by_length[len(s)].append(s)
    return by_length

with PuzzleContext(year=2021, day=8) as ctx:
    signals = []
    codes = []
    for line in ctx.nonempty_lines:
        signals_part, codes_part = line.split(" | ")
        signals_part = ["".join(sorted(list(p))) for p in signals_part.split(" ")]
        codes_part = ["".join(sorted(list(p))) for p in codes_part.split(" ")]
        signals.append(signals_part)
        codes.append(codes_part)

    ctx.submit(1, solve1(codes))
    ctx.submit(2, solve2(signals, codes))
