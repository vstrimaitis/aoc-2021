from typing import List
from puzzle import PuzzleContext

class Board:
    def __init__(self, values: List[List[int]]):
        self._values = values
        self._n = len(values)
        self._m = len(values[0])
        self._is_marked = [[False for _ in range(self._m)] for _ in range(self._n)]
        self.is_finished = False
        self._init_coordinate_lookup()
        self._init_unmarked_counters()

    def _init_coordinate_lookup(self):
        self._coords = {}
        for i in range(self._n):
            for j in range(self._m):
                self._coords[self._values[i][j]] = (i, j)
    
    def _init_unmarked_counters(self):
        self._unmarked_cols = [self._m]*self._n
        self._unmarked_rows = [self._n]*self._m

    def mark(self, num):
        if num not in self._coords:
            return
        i, j = self._coords[num]
        self._is_marked[i][j] = True
        self._unmarked_cols[i] -= 1
        self._unmarked_rows[j] -= 1
        if self._unmarked_rows[j] == 0 or self._unmarked_cols[i] == 0:
            self.is_finished = True

    def get_unmarked_sum(self):
        return sum(
            sum(
                self._values[i][j]
                for j in range(self._m)
                if not self._is_marked[i][j]
            )
            for i in range(self._n)
        )

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
                last_ans = b.get_unmarked_sum() * num
                if first_ans is None:
                    first_ans = last_ans
            else:
                unfinished_boards.append(b)
        boards = unfinished_boards

    ctx.submit(1, first_ans)
    ctx.submit(2, last_ans)
