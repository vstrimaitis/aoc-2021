from collections import *
from typing import *
from heapq import *
from puzzle import PuzzleContext


def do_stuff(lights, xs, ys, zs, val, mn, mx):
    for x in range(max(mn, xs[0]), min(mx, xs[1]) + 1):
        for y in range(max(mn, ys[0]), min(mx, ys[1]) + 1):
            for z in range(max(mn, zs[0]), min(mx, zs[1]) + 1):
                lights[(x, y, z)] = val


def solve1(cuboids, min_coord=-50, max_coord=50):
    lights = defaultdict(lambda: 0)
    for (xs, ys, zs, t) in cuboids:
        do_stuff(lights, xs, ys, zs, t, min_coord, max_coord)

    return sum([1 for x in lights.values() if x == 1])


def calc_volume(cuboid):
    xs, ys, zs, t = cuboid
    if t == 0:
        return 0
    return max(xs[1] - xs[0] + 1, 0) * max(ys[1] - ys[0] + 1, 0) * max(zs[1] - zs[0] + 1, 0)


def contains(c1, c2):
    x1, y1, z1, _ = c1
    x2, y2, z2, _ = c2
    return (
        x1[0] <= x2[0]
        and x2[1] <= x1[1]
        and y1[0] <= y2[0]
        and y2[1] <= y1[1]
        and z1[0] <= z2[0]
        and z2[1] <= z1[1]
    )


def intersect(c1, c2):
    x1, y1, z1, _ = c1
    x2, y2, z2, _ = c2

    if (
        x1[1] < x2[0]
        or x1[0] > x2[1]
        or y1[1] < y2[0]
        or y1[0] > y2[1]
        or z1[1] < z2[0]
        or z1[0] > z2[1]
    ):
        return None

    x_final = (max(x1[0], x2[0]), min(x1[1], x2[1]))
    y_final = (max(y1[0], y2[0]), min(y1[1], y2[1]))
    z_final = (max(z1[0], z2[0]), min(z1[1], z2[1]))

    return (x_final, y_final, z_final)


def split(existing, new):
    intersection = intersect(existing, new)
    if intersection is None:
        return [existing, new]
    x1, y1, z1, t1 = existing
    x2, y2, z2 = intersection
    new_cuboids = []
    for x in [(x1[0], x2[0]-1), (x2[0], x2[1]), (x2[1]+1, x1[1])]:
        for y in [(y1[0], y2[0]-1), (y2[0], y2[1]), (y2[1]+1, y1[1])]:
            for z in [(z1[0], z2[0]-1), (z2[0], z2[1]), (z2[1]+1, z1[1])]:
                if x == x2 and y == y2 and z == z2:
                    new_cuboids.append((x, y, z, new[-1]))
                else:
                    new_cuboids.append((x, y, z, t1))
    return [c for c in new_cuboids if calc_volume(c) > 0]


def calc_new_cuboids(disjoint_cuboids, cuboid):
    ans = []
    for c in disjoint_cuboids:
        if intersect(c, cuboid) is None:
            ans.append(c)
        else:
            for x in split(c, cuboid):
                if contains(cuboid, x):
                    continue
                ans.append(x)
    ans.append(cuboid)
    return [c for c in ans if calc_volume(c) > 0]


def solve2(cuboids):
    disjoint_cuboids = []
    for i, c in enumerate(cuboids):
        disjoint_cuboids = calc_new_cuboids(disjoint_cuboids, c)
    return sum(map(calc_volume, disjoint_cuboids))


with PuzzleContext(year=2021, day=22) as ctx:
    lights = defaultdict(lambda: 0)
    cuboids = []
    for line in ctx.nonempty_lines:
        t, p = line.split(" ")
        p = p.split(",")
        x, y, z = [list(map(int, x.split("=")[1].split(".."))) for x in p]
        cuboids.append((x, y, z, 1 if t == "on" else 0))

    ctx.submit(1, solve1(cuboids))
    ctx.submit(2, solve2(cuboids))
