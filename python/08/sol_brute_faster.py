from itertools import permutations
from typing import Dict, List, Optional
from puzzle import PuzzleContext

Mapping = Dict[str, str]

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

def decode(signals: List[str], codes: List[str], mappings: Dict[str, Mapping]) -> int:
    for p in mappings.keys():
        signal_digits = [to_digit(mappings[p][s]) for s in signals]
        code_digits = [to_digit(mappings[p][s]) for s in codes]
        if all(s is not None for s in signal_digits + code_digits):
            return int("".join(code_digits))
    assert False

def solve2(signals: List[List[int]], codes: List[List[str]]) -> int:
    all_patterns = set(sum(signals, []) + sum(codes, []))
    # For each (mapping, pattern) pair find what the pattern maps to when the mapping is applied
    mappings: Dict[str, Mapping] = dict()
    for p in permutations("abcdefg"):
        mapping = "".join(p)
        mappings[mapping] = dict()
        for s in all_patterns:
            mappings[mapping][s] = encode(s, mapping)
    
    ans = 0
    for s, c in zip(signals, codes):
        ans += decode(s, c, mappings)
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

