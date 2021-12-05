from collections import defaultdict
from puzzle import PuzzleContext

def sign(x):
    return 1 if x > 0 else -1 if x < 0 else 0

def count_intersections(lines):
    counts = defaultdict(lambda: 0)
    for (x1, y1), (x2, y2) in lines:
        dx = sign(x2-x1)
        dy = sign(y2-y1)

        x, y = x1, y1
        while x != x2+dx or y != y2+dy:
            counts[(x, y)] += 1
            x += dx
            y += dy
    return sum(1 for x in counts.values() if x > 1)

def parse_line(l):
    return [[int(y) for y in x.split(",")] for x in l.split(" -> ")]

def is_line_axial(l):
    (x1, y1), (x2, y2) = l
    return x1 == x2 or y1 == y2

with PuzzleContext(year=2021, day=5) as ctx:
    lines = list(map(parse_line, ctx.nonempty_lines))
    axial_lines = list(filter(is_line_axial, lines))
    ctx.submit(1, count_intersections(axial_lines))
    ctx.submit(2, count_intersections(lines))
