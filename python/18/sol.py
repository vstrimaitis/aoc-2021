from collections import *
from typing import *
from heapq import *
from puzzle import PuzzleContext
from dataclasses import dataclass


@dataclass
class FishNumber:
    """
    `nums` represents the actual numbers contained in the fish number.
    `pair_idxs` contains a list of 0s and 1s for each number. 0 means that this
        number is the first element of a pair and 1 means that it is the second
        number of a pair. The first element in the list represent the number's
        position in the innermost pair, the second element - the *pair's* position
        in the contianing pair (i.e. one level up in the hierarchy), etc.

    Here are some examples:

        The fish number "[1, 2]" resolves to:
            nums = [1, 2]
            pair_idxs = [[0], [1]]

        The fish number "[[1, 2], [3, 4]]" resolves to:
            nums = [1, 2, 3, 4]
            pair_idxs = [[0, 0], [1, 0], [0, 1], [1, 1]]

        The fish number "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]" resolves to:
            nums = [7, 3, 7, 4, 3, 6, 3, 8, 8]
            pair_idxs = [[0], [0, 0, 0, 1], [1, 0, 0, 1], [0, 1, 0, 1], [1, 1, 0, 1], [0, 0, 1, 1], [1, 0, 1, 1], [0, 1, 1, 1], [1, 1, 1, 1]]
    """

    nums: List[int]
    pair_idxs: List[List[int]]

    def __len__(self) -> int:
        return len(self.nums)

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, FishNumber):
            return False
        return self.nums == other.nums and self.pair_idxs == other.pair_idxs

    def __add__(self, other: "FishNumber") -> "FishNumber":
        f = self._concat(other)
        while True:
            new_f = f._reduce()
            if new_f == f:
                break
            f = new_f.copy()
        return f

    @classmethod
    def zero(cls) -> "FishNumber":
        return FishNumber([], [])

    def copy(self) -> "FishNumber":
        return FishNumber(self.nums.copy(), [x.copy() for x in self.pair_idxs])

    def get_nesting(self, i: int) -> int:
        return len(self.pair_idxs[i])

    def to_pair(self) -> Tuple["FishNumber", "FishNumber"]:
        left = FishNumber([], [])
        right = FishNumber([], [])
        for i in range(len(self)):
            if self.pair_idxs[i][-1] == 0:
                assert len(right) == 0
                left.nums.append(self.nums[i])
                left.pair_idxs.append(self.pair_idxs[i].copy())
                left.pair_idxs[-1].pop()
            else:
                right.nums.append(self.nums[i])
                right.pair_idxs.append(self.pair_idxs[i].copy())
                right.pair_idxs[-1].pop()
        return left, right

    def _explode(self) -> "FishNumber":
        n = len(self)
        for i in range(n):
            if (
                i + 1 < n
                and self.get_nesting(i) > 4
                and self.pair_idxs[i][0] == 0
                and self.pair_idxs[i + 1][0] == 1
            ):
                left = self.nums[:i]
                lefti = self.pair_idxs[:i]
                right = self.nums[i + 2 :]
                righti = self.pair_idxs[i + 2 :]
                if len(left) > 0:
                    left[-1] += self.nums[i]
                if len(right) > 0:
                    right[0] += self.nums[i + 1]
                ii = self.pair_idxs[i][1:]
                jj = self.pair_idxs[i + 1][1:]
                assert ii == jj

                return FishNumber(
                    left + [0] + right,
                    lefti + [ii] + righti,
                )
        return self

    def _split(self) -> "FishNumber":
        n = len(self)
        for i in range(n):
            if self.nums[i] >= 10:
                x = self.nums[i] // 2
                y = self.nums[i] // 2 + self.nums[i] % 2
                left = self.nums[:i]
                lefti = self.pair_idxs[:i]
                right = self.nums[i + 1 :]
                righti = self.pair_idxs[i + 1 :]

                return FishNumber(
                    left + [x, y] + right,
                    lefti + [[0] + self.pair_idxs[i], [1] + self.pair_idxs[i]] + righti,
                )
        return self

    def _concat(self, other: "FishNumber") -> "FishNumber":
        f1 = self.copy()
        f2 = other.copy()
        if len(f1.nums) > 0 and len(f2.nums) > 0:
            for p in f1.pair_idxs:
                p.append(0)
            for p in f2.pair_idxs:
                p.append(1)

        return FishNumber(f1.nums + f2.nums, f1.pair_idxs + f2.pair_idxs)

    def _reduce(self) -> "FishNumber":
        f = self._explode()
        if f != self:
            return f
        return self._split()

    def magnitude(self) -> int:
        if len(self) == 1:
            return self.nums[0]
        if len(self) == 2:
            l, r = self.nums
        else:
            l, r = [x.magnitude() for x in self.to_pair()]
        return 3 * l + 2 * r

    def __str__(self) -> str:
        if len(self) == 1:
            return str(self.nums[0])
        l, r = self.to_pair()
        return f"[{l},{r}]"

    def __repr__(self) -> str:
        return str(self)


def parse(line: str) -> Tuple[FishNumber, str]:
    if line[0] == "[":
        x, s = parse(line[1:])
        assert s[0] == ","
        y, s = parse(s[1:])
        assert s[0] == "]"
        for p in x.pair_idxs:
            p.append(0)
        for p in y.pair_idxs:
            p.append(1)
        return FishNumber(x.nums + y.nums, x.pair_idxs + y.pair_idxs), s[1:]
    elif line[0] in "0123456789":
        x = int(line[0])
        return FishNumber([x], [[]]), line[1:]
    assert False


with PuzzleContext(year=2021, day=18) as ctx:
    fish_nums = list(map(lambda x: parse(x)[0], ctx.nonempty_lines))

    f = sum(fish_nums, FishNumber.zero())
    ctx.submit(1, f.magnitude())

    ans2 = max(
        [(f1 + f2).magnitude() for f1 in fish_nums for f2 in fish_nums if f1 != f2]
    )
    ctx.submit(2, ans2)
