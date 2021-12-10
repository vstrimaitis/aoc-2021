from typing import Tuple
from puzzle import PuzzleContext

def get_corruption(line):
    s = []
    for c in line:
        if c in "([{<":
            s.append(c)
        else:
            cc = s.pop()
            if cc == "(" and c != ")" or cc == "[" and c != "]" or cc == "{" and c != "}" or cc == "<" and c != ">":
                return c
    return None

def flip(c):
    cc = {
        "(": ")",
        "{": "}",
        "[": "]",
        "<": ">"
    }
    return cc[c]

def get_corruption_2(line):
    s = []
    for c in line:
        if c in "([{<":
            s.append(c)
        else:
            cc = s.pop()
            # if cc == "(" and c != ")" or cc == "[" and c != "]" or cc == "{" and c != "}" or cc == "<" and c != ">":
            #     return c
    ans = ""
    while len(s) > 0:
        ans += flip(s.pop())
    return ans

def calc_score(s):
    pts = {
        ")": 1,
        "]": 2,
        "}": 3,
        ">": 4,
    }
    ans = 0
    for c in s:
        ans = 5*ans + pts[c]
    return ans

# ---
OPENING = "([{<"
CLOSING = ")]}>"

def flip(c: str) -> str:
    for open, close in zip(OPENING, CLOSING):
        if c == open:
            return close
        if c == close:
            return open
    assert False, "well shit"

def analyse_line(line: str) -> Tuple[bool, str]:
    opens = []
    for c in line:
        if c in OPENING:
            opens.append(c)
        else:
            matching_open = opens.pop()
            if flip(matching_open) != c:
                return True, c
    return False, "".join(map(flip, reversed(opens)))


def calc_syntax_error_score(mismatched_char: str) -> int:
    points = {
        ")": 3,
        "]": 57,
        "}": 1197,
        ">": 25137
    }
    return points[mismatched_char]

def calc_completion_score(completion_string: str) -> int:
    ans = 0
    for c in completion_string:
        ans = 5*ans + (")]}>".index(c) + 1)
    return ans

with PuzzleContext(year=2021, day=10) as ctx:
    syntax_error_scores = []
    completion_scores = []
    for line in ctx.nonempty_lines:
        is_corrupt, s = analyse_line(line)
        if is_corrupt:
            syntax_error_scores.append(calc_syntax_error_score(s))
        else:
            completion_scores.append(calc_completion_score(s))
    ans1 = sum(syntax_error_scores)
    ans2 = list(sorted(completion_scores))[len(completion_scores) // 2]
    ctx.submit(1, ans1)
    ctx.submit(2, ans2)
