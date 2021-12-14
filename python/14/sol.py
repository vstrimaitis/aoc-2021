from typing import Dict
from collections import defaultdict
from puzzle import PuzzleContext

FreqTable = Dict[str, int]
Rules = Dict[str, str]

PADDING = "$"

def parse_rule(s: str) -> str:
    parts = s.split(" -> ")
    return parts[0], parts[1]

def get_pair_freqs(s: str) -> FreqTable:
    d = defaultdict(lambda: 0)
    for i in range(len(s)-1):
        pair = s[i:i+2]
        d[pair] += 1
    return d

def apply_rules(pair_freqs: FreqTable, rules: Rules) -> FreqTable:
    new_freqs = defaultdict(lambda: 0)
    for pair, f in pair_freqs.items():
        if pair in rules:
            a, b = list(pair)
            mid = rules[pair]
            new_freqs[a + mid] += f
            new_freqs[mid + b] += f
        else:
            new_freqs[pair] += f
    return new_freqs

def calc_answer(pair_freqs: FreqTable) -> int:
    d = defaultdict(lambda: 0)
    for pair, f in pair_freqs.items():
        d[pair[0]] += f
        d[pair[1]] += f
    d.pop(PADDING, None)
    mn = min(d.values()) // 2
    mx = max(d.values()) // 2
    return mx - mn

with PuzzleContext(year=2021, day=14) as ctx:
    parts = ctx.data.split("\n\n")
    polymer_template = PADDING + parts[0] + PADDING
    rules = dict([parse_rule(x) for x in parts[1].split("\n")])

    pair_freqs = get_pair_freqs(polymer_template)

    ans1 = 0
    for step in range(40):
        if step == 10:
            ans1 = calc_answer(pair_freqs)
        pair_freqs = apply_rules(pair_freqs, rules)
    ans2 = calc_answer(pair_freqs)

    ctx.submit(1, ans1)
    ctx.submit(2, ans2)
