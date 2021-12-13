from typing import List, Tuple
from functools import reduce
from puzzle import PuzzleContext

Point = Tuple[int, int]
Fold = Tuple[str, int]

def flip(p: Point) -> Point:
    x, y = p
    return y, x

def do_fold(pts: List[Point], fold: Fold) -> List[Point]:
    axis, coord = fold
    if axis == "x":
        flipped_pts = list(map(flip, pts))
        return list(map(flip, do_fold(flipped_pts, ("y", coord))))
    assert axis == "y"
    new_pts = []
    for x, y in pts:
        if y < coord:
            new_pts.append((x, y))
        else:
            new_pts.append((x, coord-(y-coord)))
    return new_pts

def to_string(pts: List[Point]) -> str:
    xs = [x for x, _ in pts]
    ys = [y for _, y in pts]
    lines = []
    for i in range(min(ys), max(ys)+1):
        draw_point = lambda j: "#" if (j, i) in pts else " "
        row = map(draw_point, range(min(xs), max(xs)+1))
        lines.append("".join(row))
    return "\n".join(lines)

def parse_point(s: str) -> Point:
    parts = s.split(",")
    return int(parts[0]), int(parts[1])

def parse_split(s: str) -> Fold:
    prefix = "fold along "
    parts = s[len(prefix):].split("=")
    return parts[0], int(parts[1])

with PuzzleContext(year=2021, day=13) as ctx:
    pts, folds = ctx.data.split("\n\n")
    pts = [parse_point(x) for x in pts.split("\n")]
    folds = [parse_split(x) for x in folds.split("\n")]

    pts_after_first_fold = do_fold(pts, folds[0])
    unique_pts = list(set(pts_after_first_fold))
    ctx.submit(1, len(unique_pts))

    final_pts = reduce(lambda p, f: do_fold(p, f), folds, pts)
    print("Part 2:")
    print(to_string(final_pts))
