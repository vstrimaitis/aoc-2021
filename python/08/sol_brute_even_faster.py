from itertools import permutations
from functools import reduce
from collections import defaultdict
from typing import Dict, List, Optional, Set
from puzzle import PuzzleContext

def solve1(codes: List[List[str]]) -> int:
    all_codes = sum(codes, []) # flattens the list of lists
    return len(list(filter(lambda s: len(s) in {2, 3, 4, 7}, all_codes)))

def encode(s: str, mapping: str) -> str:
    return "".join(sorted(mapping[ord(c)-ord('a')] for c in list(s)))

def to_digit(s: str) -> Optional[str]:
    digits = {
        "abcefg": "0",
        "cf": "1",
        "acdeg": "2",
        "acdfg": "3",
        "bcdf": "4",
        "abdfg": "5",
        "abdefg": "6",
        "acf": "7",
        "abcdefg": "8",
        "abcdfg": "9",
    }
    if s in digits:
        return digits[s]
    return None

def resolve_code(signals: List[str], codes: List[str], mappings: Dict[str, Set[str]]) -> int:
    # Find an intersection of all valid mappings
    valid_mappings = reduce(lambda a, b: a & b, [mappings[s] for s in signals + codes])
    assert len(valid_mappings) == 1
    mapping = list(valid_mappings)[0]
    return int("".join(to_digit(encode(s, mapping)) for s in codes))

def solve2(signals: List[List[int]], codes: List[List[str]]) -> int:
    all_patterns = set(sum(signals, []) + sum(codes, []))
    # for each pattern, find a set of mappings that map it to a valid digit
    valid_mappings = defaultdict(lambda: set())
    for p in permutations("abcdefg"):
        mapping = "".join(p)
        for s in all_patterns:
            s_enc = encode(s, mapping)
            if to_digit(s_enc) is not None:
                valid_mappings[s].add(mapping)
    ans = 0
    for s, c in zip(signals, codes):
        ans += resolve_code(s, c, valid_mappings)
    return ans

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

