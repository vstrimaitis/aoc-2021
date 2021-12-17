from collections import *
from typing import *
from heapq import *
from puzzle import PuzzleContext
import re
from dataclasses import dataclass


@dataclass
class Vec2:
    x: int
    y: int

    def __add__(self, other: "Vec2") -> "Vec2":
        return Vec2(self.x + other.x, self.y + other.y)

    def __sub__(self, other: "Vec2") -> "Vec2":
        return self + Vec2(-other.x, -other.y)


@dataclass
class Rect:
    top_left: Vec2
    bottom_right: Vec2

    @property
    def left(self):
        return self.top_left.x

    @property
    def right(self):
        return self.bottom_right.x

    @property
    def top(self):
        return self.top_left.y

    @property
    def bottom(self):
        return self.bottom_right.y

    def contains(self, point: Vec2) -> bool:
        return self.left <= point.x <= self.right and self.bottom <= point.y <= self.top


class Simulation:
    def __init__(self, x1: int, x2: int, y1: int, y2: int, vx: int, vy: int):
        self.position = Vec2(0, 0)
        self.velocity = Vec2(vx, vy)
        self.target = Rect(
            Vec2(min(x1, x2), max(y1, y2)), Vec2(max(x1, x2), min(y1, y2))
        )
        self._time = 0

    def move(self):
        self.position += self.velocity
        self.velocity -= Vec2(min(1, self.velocity.x), 1)

    def is_inside_target(self) -> bool:
        return self.target.contains(self.position)

    def has_overshot(self) -> bool:
        return (
            self.position.y < self.target.bottom or self.position.x > self.target.right
        )


def will_hit_target(vx: int, vy: int, x1: int, x2: int, y1: int, y2: int) -> bool:
    sim = Simulation(x1, x2, y1, y2, vx, vy)
    while not sim.has_overshot():
        sim.move()
        if sim.is_inside_target():
            return True
    return False


with PuzzleContext(year=2021, day=17) as ctx:
    pattern = r"target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-\d+)"
    res = re.search(pattern, ctx.data.strip())
    assert res
    x1, x2, y1, y2 = tuple(map(int, res.groups()))

    ans1 = (-y1 - 1) * (-y1) // 2
    ctx.submit(1, ans1)

    ans2 = 0
    for vx in range(1, x2 + 1):
        for vy in range(y1, -y1):
            if will_hit_target(vx, vy, x1, x2, y1, y2):
                ans2 += 1
    ctx.submit(2, ans2)
