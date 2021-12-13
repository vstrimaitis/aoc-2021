from puzzle import PuzzleContext


def do_fold(pts, axis, coord):
    if axis == "y":
        new_pts = []
        for x, y in pts:
            if y < coord:
                new_pts.append((x, y))
            else:
                new_pts.append((x, coord-(y-coord)))
        return new_pts
    new_pts = []
    for x, y in pts:
        if x < coord:
            new_pts.append((x, y))
        else:
            new_pts.append((coord-(x-coord), y))
    return new_pts

def to_string(pts):
    xs = [x for x, _ in pts]
    ys = [y for _, y in pts]
    lines = []
    for i in range(min(ys), max(ys)+1):
        row = []
        for j in range(min(xs), max(xs)+1):
            if (j, i) in pts:
                row.append("#")
            else:
                row.append(" ")
        lines.append("".join(row))
    return "\n".join(lines)

with PuzzleContext(year=2021, day=13) as ctx:
    pts, folds = ctx.data.split("\n\n")
    pts = [(int(x.split(",")[0]), int(x.split(",")[1]) )for x in pts.split("\n")]
    folds = [x[11:].split("=") for x in folds.split("\n")]

    pts = do_fold(pts, folds[0][0], int(folds[0][1]))
    ctx.submit(1, len(list(set(pts))))

    for i in range(1, len(folds)):
        pts = do_fold(pts, folds[i][0], int(folds[i][1]))
    print("Part 2:")
    print(to_string(pts))
