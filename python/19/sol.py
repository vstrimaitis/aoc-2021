from collections import *
from typing import *
from heapq import *
from itertools import permutations
from puzzle import PuzzleContext

Vec3 = Tuple[int, int, int]
ScannerReading = Tuple[int, int, int]
ScannerData = List[ScannerReading]


def parse(line: str) -> ScannerReading:
    return tuple(map(int, line.split(",")))


def transforms():
    for p in permutations("xyz"):
        p = "".join(p)
        for dx in [-1, 1]:
            for dy in [-1, 1]:
                for dz in [-1, 1]:
                    yield (p, dx, dy, dz)


def do_transform(x, y, z, t):
    m = {
        "xyz": (lambda x, y, z: (x, y, z)),
        "xzy": (lambda x, z, y: (x, y, z)),
        "yxz": (lambda y, x, z: (x, y, z)),
        "yzx": (lambda y, z, x: (x, y, z)),
        "zxy": (lambda z, x, y: (x, y, z)),
        "zyx": (lambda z, y, x: (x, y, z)),
    }
    p, dx, dy, dz = t
    x, y, z = m[p](x, y, z)
    x = dx * x
    y = dy * y
    z = dz * z
    return x, y, z


def sub(a: ScannerReading, b: ScannerReading) -> ScannerReading:
    return b[0] - a[0], b[1] - a[1], b[2] - a[2]


def calc_direction_vectors(s: ScannerData) -> List[Vec3]:
    return list(sorted([sub(a, b) for a in s for b in s]))


def detect_overlap(s1, s2):
    for (bx1, by1, bz1) in s1:
        for (bx2, by2, bz2) in s2:
            # assume that b1 and b2 match
            for t in transforms():
                bx2_t, by2_t, bz2_t = do_transform(bx2, by2, bz2, t)
                x2 = bx1 - bx2_t
                y2 = by1 - by2_t
                z2 = bz1 - bz2_t
                s2_new = []
                for (x, y, z) in s2:
                    x, y, z = do_transform(x, y, z, t)
                    x += x2
                    y += y2
                    z += z2
                    s2_new.append((x, y, z))
                common = list(set(s1) & set(s2_new))
                if len(common) == 12:
                    return x2, y2, z2, t
    return None


if __name__ == "__main__":
    with PuzzleContext(year=2021, day=19) as ctx:
        scanners = []
        for block in ctx.data.split("\n\n"):
            scanners.append(list(map(parse, block.split("\n")[1:])))
        positions = [None] * len(scanners)
        positions[0] = (0, 0, 0, ("xyz", 1, 1, 1))

        print("Preprocessing...")
        direction_vectors = list(map(set, map(calc_direction_vectors, scanners)))
        direction_vectors_with_transforms = [
            [
                set([do_transform(v[0], v[1], v[2], t) for v in d])
                for t in transforms()
            ]
            for d in direction_vectors
        ]
        print("Finding pairs to connect...")
        to_try = []
        for i in range(len(scanners)):
            for j in range(i+1, len(scanners)):
                if i == j:
                    continue
                if any(len(direction_vectors[i] & d) > 100 for d in direction_vectors_with_transforms[j]):
                    to_try.append((i, j))

        print("Connecting...")
        while len(to_try) > 0:
            for i, j in to_try:
                if positions[i] is not None and positions[j] is not None:
                    to_try = [
                        (ii, jj) for (ii, jj) in to_try if (ii, jj) != (i, j)
                    ]
                    break
                if positions[i] is None and positions[j] is None:
                    continue
                if positions[i] is None and positions[j] is not None:
                    i, j = j, i
                s1 = scanners[i]
                s2 = scanners[j]
                p = detect_overlap(s1, s2)
                if p is not None:
                    x1, y1, z1, _ = positions[i]
                    x2, y2, z2, t = p
                    positions[j] = (x1 + x2, y1 + y2, z1 + z2, t)
                    for k, s in enumerate(scanners[j]):
                        scanners[j][k] = do_transform(*scanners[j][k], t)
                    print("Connected: ", i, j)
                to_try = [(ii, jj) for (ii, jj) in to_try if (ii, jj) != (i, j)]
                break

        beacons = set()
        for scanner, (x, y, z, t) in zip(scanners, positions):
            for s in scanner:
                sx, sy, sz = s
                sx, sy, sz = sx + x, sy + y, sz + z
                beacons.add((sx, sy, sz))

        ans2 = 0
        for x1, y1, z1, _ in positions:
            for x2, y2, z2, _ in positions:
                ans2 = max(ans2, abs(x1 - x2) + abs(y1 - y2) + abs(z1 - z2))

        ctx.submit(1, len(beacons))
        ctx.submit(2, ans2)
