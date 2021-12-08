from collections import defaultdict
from typing import List, Tuple, Dict
from puzzle import PuzzleContext

ALL_LETTERS = "abcdefg"

def solve1(codes: List[List[str]]) -> int:
    all_codes = sum(codes, []) # flattens the list of lists
    return len(list(filter(lambda s: len(s) in {2, 3, 4, 7}, all_codes)))

def solve2(signals: List[List[str]], codes: List[List[str]]) -> int:
    """Solution idea.
    
    1. Collect all "useful" mappings - the ones that reduce the number of possibilities:
        * signal with 2 letters maps to "cf" (1)
        * signal with 3 letters maps to "acf" (7)
        * signal with 4 letters maps to "bcdf" (4)
        * signal with 7 letters maps to "abcdefg" (8) - can be skipped
        * there will be 3 signals left with 5 letters:
            * they correspond to digits 2, 3 and 5
            * the segments that are used in *all* of these digits are a, d, and g
            * therefore - find the common letters between these three signals and map them to "adg"
        * there will be 3 signals left with 6 letters:
            * they correspond to digits 0, 6, 9
            * do the same as before and map the result to "abfg"
    2. Construct a 7x7 boolean table where each row and column corresponds to one of the letters in "abcdefg"
       And a cell at (i, j) corresponds whether the letter at row i can be mapped to a letter in column j.
    3. For each useful mapping (s, t):
        * Let
            - c_s be a letter in s
            - c_t be a letter in t
            - v_s be a letter *not* in s
            - v_t be a letter *not* in t
        * Mark the cell at row c_s and column v_t as False
        * Mark the cell at row v_s and column c_t as False
    4. Each row and each column should contain exactly one cell with the value True - these cells correspond
       to the mapping that we're looking for
    """
    return sum(resolve_code(s, c) for s, c in zip(signals, codes))

def resolve_code(signals: List[str], codes: List[str]) -> int:
    signals_by_length = group_by_length(signals) 
    useful_mappings = [
        (signals_by_length[2][0], "cf"),                        # 1
        (signals_by_length[3][0], "acf"),                       # 7
        (signals_by_length[4][0], "bcdf"),                      # 4
        (extract_common_letters(signals_by_length[5]), "adg"),  # 2, 3, 5
        (extract_common_letters(signals_by_length[6]), "abfg"), # 0, 6, 9
    ]
    mapping = resolve_mapping(useful_mappings)
    return int("".join(to_digit(encode(s, mapping)) for s in codes))

def group_by_length(strings: List[str]) -> Dict[str, List[str]]:
    by_length = defaultdict(list)
    for s in strings:
        by_length[len(s)].append(s)
    return by_length

def extract_common_letters(patterns: List[str]) -> str:
    letters = set(list(ALL_LETTERS))
    for p in patterns:
        letters = {c for c in letters if c in p}
    return "".join(sorted(list(letters)))

def resolve_mapping(useful_mappings: List[Tuple[str, str]]) -> str:
    possibilities = {c: set(list(ALL_LETTERS)) for c in ALL_LETTERS}
    for p_from, p_to in useful_mappings:
        p_from_inv = invert(p_from)
        p_to_inv = invert(p_to)
        for c1 in p_from:
            for c2 in p_to_inv:
                possibilities[c1].discard(c2)
        for c1 in p_from_inv:
            for c2 in p_to:
                possibilities[c1].discard(c2)
    
    assert all(len(p) == 1 for _, p in possibilities.items())
    return "".join([list(possibilities[c])[0] for c in ALL_LETTERS])

def invert(s: str) -> str:
    letters = set(list(s))
    return "".join(c for c in ALL_LETTERS if c not in letters)

def encode(s: str, mapping: str) -> str:
    return "".join(sorted(mapping[ord(c)-ord('a')] for c in list(s)))

def to_digit(s: str) -> str:
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
    return digits[s]

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
