from itertools import permutations
from puzzle import PuzzleContext

def decode_digit(digit, mapping):
    s = encode(digit, mapping)
    digits = {
        "abcefg": 0,
        "cf": 1,
        "acdeg": 2,
        "acdfg": 3,
        "bcdf": 4,
        "abdfg": 5,
        "abdefg": 6,
        "acf": 7,
        "abcdefg": 8,
        "abcdfg": 9,
    }
    assert s in digits
    return digits[s]

def encode(s, assignment):
    return "".join(sorted(assignment[ord(c)-ord('a')] for c in list(s)))

def is_mapping_ok(signals, assignment):
    new_signals = [encode(s, assignment) for s in signals]
    digits = {
        "abcefg": 0,
        "cf": 1,
        "acdeg": 2,
        "acdfg": 3,
        "bcdf": 4,
        "abdfg": 5,
        "abdefg": 6,
        "acf": 7,
        "abcdefg": 8,
        "abcdfg": 9,
    }
    for s in new_signals:
        if s not in digits:
            return False
    return True
    

def decode(signals, output):
    ans = 0
    for p in permutations("abcdefg"):
        assignment = "".join(p)
        # abcdefg maps to assignment
        if is_mapping_ok(signals, assignment):
            for digit in output:
                ans = 10*ans + decode_digit(digit, assignment)
    return ans

def solve2(inputs):
    ans = 0
    for signals, code in inputs:
        ans += decode(signals, code)
    return ans

with PuzzleContext(year=2021, day=8) as ctx:
    ans = 0
    for line in ctx.nonempty_lines:
        for x in line.split(" | ")[1].split(" "):
            if len(x) == 2 or len(x) == 4 or len(x) == 3 or len(x) == 7:
                ans += 1
    ctx.submit(1, ans)

    inputs = [[x.split(" ") for x in line.split(" | ")] for line in ctx.nonempty_lines]
    ctx.submit(2, solve2(inputs))
