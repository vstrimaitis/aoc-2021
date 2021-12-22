from collections import *
from typing import *
from heapq import *
from dataclasses import dataclass
import re
from puzzle import PuzzleContext


@dataclass(eq=True, frozen=True)
class Vec3:
    x: int
    y: int
    z: int

    def to_tuple(self) -> Tuple[int, int, int]:
        return self.x, self.y, self.z


@dataclass(eq=True, frozen=True)
class Cuboid:
    low_corner: Vec3
    high_corner: Vec3

    def contains(self, other: "Cuboid") -> bool:
        return self.intersect(other) == other

    def intersect(self, other: "Cuboid") -> Optional["Cuboid"]:
        x11, y11, z11 = self.low_corner.to_tuple()
        x12, y12, z12 = self.high_corner.to_tuple()
        x21, y21, z21 = other.low_corner.to_tuple()
        x22, y22, z22 = other.high_corner.to_tuple()

        if x12 < x21 or x11 > x22 or y12 < y21 or y11 > y22 or z12 < z21 or z11 > z22:
            return None

        corner1 = Vec3(x=max(x11, x21), y=max(y11, y21), z=max(z11, z21))
        corner2 = Vec3(x=min(x12, x22), y=min(y12, y22), z=min(z12, z22))

        return Cuboid(low_corner=corner1, high_corner=corner2)

    @property
    def volume(self) -> int:
        w = max(self.high_corner.x - self.low_corner.x + 1, 0)
        l = max(self.high_corner.y - self.low_corner.y + 1, 0)
        h = max(self.high_corner.z - self.low_corner.z + 1, 0)
        return w * h * l

    def split(self, other: "Cuboid") -> List["Cuboid"]:
        intersection = self.intersect(other)
        if intersection is None:
            return [self]

        x11, y11, z11 = self.low_corner.to_tuple()
        x12, y12, z12 = self.high_corner.to_tuple()
        x21, y21, z21 = intersection.low_corner.to_tuple()
        x22, y22, z22 = intersection.high_corner.to_tuple()

        new_cuboids: List["Cuboid"] = []
        for x_range in [(x11, x21 - 1), (x21, x22), (x22 + 1, x12)]:
            for y_range in [(y11, y21 - 1), (y21, y22), (y22 + 1, y12)]:
                for z_range in [(z11, z21 - 1), (z21, z22), (z22 + 1, z12)]:
                    c = Cuboid(
                        low_corner=Vec3(x_range[0], y_range[0], z_range[0]),
                        high_corner=Vec3(x_range[1], y_range[1], z_range[1]),
                    )
                    if not other.contains(c) and c.volume > 0:
                        new_cuboids.append(c)
        return new_cuboids


@dataclass
class RebootStep:
    cuboid: Cuboid
    switch_type: int

    @classmethod
    def parse(cls, s: str) -> "RebootStep":
        pattern = (
            r"(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)"
        )
        res = re.search(pattern, s)
        assert res
        switch_type = int(res.group(1) == "on")
        x1 = int(res.group(2))
        x2 = int(res.group(3))
        y1 = int(res.group(4))
        y2 = int(res.group(5))
        z1 = int(res.group(6))
        z2 = int(res.group(7))
        corner1 = Vec3(min(x1, x2), min(y1, y2), min(z1, z2))
        corner2 = Vec3(max(x1, x2), max(y1, y2), max(z1, z2))
        return RebootStep(
            cuboid=Cuboid(low_corner=corner1, high_corner=corner2),
            switch_type=switch_type,
        )


def apply_step(disjoint_cuboids: Set[Cuboid], step: RebootStep) -> Set[Cuboid]:
    new_cuboids = set()

    for cuboid in disjoint_cuboids:
        for c in cuboid.split(step.cuboid):
            new_cuboids.add(c)
    if step.switch_type == 1:
        new_cuboids.add(step.cuboid)

    return new_cuboids


def solve(steps: List[RebootStep]) -> int:
    disjoint_cuboids = set()
    for step in steps:
        disjoint_cuboids = apply_step(disjoint_cuboids, step)
    return sum(c.volume for c in disjoint_cuboids)


with PuzzleContext(year=2021, day=22) as ctx:
    part_1_space = Cuboid(low_corner=Vec3(-50, -50, -50), high_corner=Vec3(50, 50, 50))

    steps = [RebootStep.parse(line) for line in ctx.nonempty_lines]
    ctx.submit(1, solve([s for s in steps if part_1_space.contains(s.cuboid)]))
    ctx.submit(2, solve(steps))
