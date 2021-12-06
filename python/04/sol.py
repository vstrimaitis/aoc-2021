from typing import List
from puzzle import PuzzleContext

class Board:
    def __init__(self, values: List[List[int]]):
        n = len(values)
        m = len(values[0])
        self._init_coordinate_lookup(values)
        self._unmarked_cols = [m]*n
        self._unmarked_rows = [n]*m
        self._is_marked = [[False for _ in range(m)] for _ in range(n)]
        self.is_finished = False
        self.unmarked_sum = sum(sum(values, []))

    def _init_coordinate_lookup(self, values):
        self._coords = {}
        for i, r in enumerate(values):
            for j, x in enumerate(r):
                self._coords[x] = (i, j)

    def mark(self, num):
        if num not in self._coords:
            return
        i, j = self._coords[num]
        self._is_marked[i][j] = True
        self._unmarked_cols[i] -= 1
        self._unmarked_rows[j] -= 1
        self.unmarked_sum -= num
        if self._unmarked_rows[j] == 0 or self._unmarked_cols[i] == 0:
            self.is_finished = True


def parse_board(s):
    values = [[int(y) for y in x.split(" ") if y] for x in s.split("\n")]
    return Board(values)

with PuzzleContext(year=2021, day=4) as ctx:
    parts = ctx.data.split("\n\n")
    nums = [int(x) for x in parts[0].split(",")]
    boards = [parse_board(x) for x in parts[1:]]

    first_ans = None
    last_ans = None
    for num in nums:
        unfinished_boards = []
        for b in boards:
            b.mark(num)
            if b.is_finished:
                last_ans = b.unmarked_sum * num
                if first_ans is None:
                    first_ans = last_ans
            else:
                unfinished_boards.append(b)
        boards = unfinished_boards

    ctx.submit(1, first_ans)
    ctx.submit(2, last_ans)
