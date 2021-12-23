from collections import *
from typing import *
from heapq import *
from dataclasses import dataclass
from enum import Enum
from puzzle import PuzzleContext


class Amphipod(Enum):
    A = (2, 1)
    B = (4, 10)
    C = (6, 100)
    D = (8, 1000)

    @property
    def target_pos(self) -> int:
        return self.value[0]

    @property
    def movement_cost(self) -> int:
        return self.value[1]

    def __lt__(self, other) -> bool:
        return self.value < other.value


@dataclass(order=True)
class Room:
    pos: int
    max_size: int
    amphipods: List[Amphipod]

    def copy(self) -> "Room":
        return Room(self.pos, self.max_size, self.amphipods.copy())

    def pop(self) -> Optional[Amphipod]:
        try:
            return self.amphipods.pop()
        except IndexError:
            return None

    def has_space(self) -> bool:
        return len(self.amphipods) < self.max_size

    def is_empty(self) -> bool:
        return len(self.amphipods) == 0

    def all_same(self) -> bool:
        return len(self.amphipods) == 0 or all(
            a == self.amphipods[0] for a in self.amphipods
        )

    def all_ok(self) -> bool:
        return all(a.target_pos == self.pos for a in self.amphipods)

    def __hash__(self) -> int:
        return hash((self.pos, tuple(self.amphipods)))

@dataclass
class Hallway:
    pos: int
    amphipod: Optional[Amphipod]

    def copy(self) -> "Hallway":
        return Hallway(self.pos, self.amphipod)

    def __hash__(self) -> int:
        return hash((self.pos, self.amphipod))

    def __lt__(self, other) -> bool:
        a = self.amphipod.value if self.amphipod else (0, 0)
        b = other.amphipod.value if other.amphipod else (0, 0)
        return (self.pos, a) < (other.pos, b)


@dataclass(order=True)
class State:
    cells: List[Union[Hallway, Room]]

    def is_terminal(self) -> bool:
        for c in self.cells:
            if isinstance(c, Hallway) and c.amphipod is not None:
                return False
            if isinstance(c, Room) and not c.all_ok():
                return False
        return True

    def draw(self) -> str:
        width = len(self.cells)+2
        height = max(c.max_size for c in self.cells if isinstance(c, Room)) + 3
        grid = [[" " for _ in range(width)] for _ in range(height)]

        for j in range(0, width):
            grid[0][j] = "#"
        grid[1][0] = "#"
        for c in self.cells:
            if isinstance(c, Hallway):
                grid[1][1+c.pos] = "." if c.amphipod is None else c.amphipod.name
            elif isinstance(c, Room):
                grid[1][1+c.pos] = "."
                for i, cc in enumerate(c.amphipods):
                    grid[height-2-i][1+c.pos] = cc.name
                for i in range(len(c.amphipods), c.max_size):
                    grid[height-2-i][1+c.pos] = "."
        for j in range(0, width):
            if grid[2][j] == " ":
                grid[2][j] = "#"
        for j in range(2, width-2):
            if grid[3][j] == " ":
                grid[3][j] = "#"
        grid[1][-1] = "#"
        for j in range(2, width-2):
            grid[-1][j] = "#"

        return "\n".join(["".join(r) for r in grid])


    def get_next(self) -> Iterable[Tuple["State", int]]:
        for i, c in enumerate(self.cells):
            if isinstance(c, Hallway):
                if c.amphipod is None:
                    continue
                # move to some room
                for j in reversed(range(0, i)):
                    cc = self.cells[j]
                    if isinstance(cc, Hallway) and cc.amphipod is not None:
                        break
                    if isinstance(cc, Room) and cc.has_space() and cc.all_ok() and c.amphipod.target_pos == cc.pos:
                        new_cells = [c.copy() for c in self.cells]
                        new_cells[i].amphipod = None
                        new_cells[j].amphipods.append(c.amphipod)
                        yield State(new_cells), (abs(j-i)+(cc.max_size-len(cc.amphipods)))*c.amphipod.movement_cost
                for j in range(i+1, len(self.cells)):
                    cc = self.cells[j]
                    if isinstance(cc, Hallway) and cc.amphipod is not None:
                        break
                    if isinstance(cc, Room) and cc.has_space() and cc.all_ok() and c.amphipod.target_pos == cc.pos:
                        new_cells = [c.copy() for c in self.cells]
                        new_cells[i].amphipod = None
                        new_cells[j].amphipods.append(c.amphipod)
                        yield State(new_cells), (abs(j-i)+(cc.max_size-len(cc.amphipods)))*c.amphipod.movement_cost
                        
            elif isinstance(c, Room):
                if c.is_empty():
                    continue
                # move to some hallway
                for j in reversed(range(0, i)):
                    cc = self.cells[j]
                    if isinstance(cc, Hallway):
                        if cc.amphipod is not None:
                            break
                        new_cells = [c.copy() for c in self.cells]
                        a = new_cells[i].pop()
                        new_cells[j].amphipod = a
                        yield State(new_cells), (abs(j-i)+(c.max_size-len(c.amphipods)+1))*a.movement_cost
                
                for j in range(i+1, len(self.cells)):
                    cc = self.cells[j]
                    if isinstance(cc, Hallway):
                        if cc.amphipod is not None:
                            break
                        new_cells = [c.copy() for c in self.cells]
                        a = new_cells[i].pop()
                        new_cells[j].amphipod = a
                        yield State(new_cells), (abs(j-i)+(c.max_size-len(c.amphipods)+1))*a.movement_cost
                        

    def __hash__(self) -> int:
        return hash(tuple(self.cells))

    @classmethod
    def parse(cls, s: str) -> "State":
        lines = s.split("\n")
        height = len(lines)
        width = len(lines[0])
        assert lines[0] == "#"*width
        assert lines[1] == "#"+"."*(width-2)+"#"

        room_size = height-3
        cells = [
            Hallway(0, None),
            Hallway(1, None),
            Room(2, room_size, []),
            Hallway(3, None),
            Room(4, room_size, []),
            Hallway(5, None),
            Room(6, room_size, []),
            Hallway(7, None),
            Room(8, room_size, []),
            Hallway(9, None),
            Hallway(10, None),
        ]
        for j in [3, 5, 7, 9]:
            for i in reversed(range(2, height-1)):
                cells[j-1].amphipods.append(Amphipod[lines[i][j]])

        return State(cells)


def dijkstra(start: State) -> int:
    pq = []
    costs = defaultdict(lambda: 10**100)

    heappush(pq, (0, start))
    costs[start] = 0
    while pq:
        d, u = heappop(pq)
        if d > costs[u]:
            continue
        if u.is_terminal():
            return costs[u]
        for v, c in u.get_next():
            if costs[u] + c < costs[v]:
                costs[v] = costs[u] + c
                heappush(pq, (costs[v], v))


with PuzzleContext(year=2021, day=23) as ctx:
    start = State.parse(ctx.data)
    ans1 = dijkstra(start)
    ctx.submit(1, ans1)

    lines = ctx.nonempty_lines
    lines = lines[:3] + ["  #D#C#B#A#  ", "  #D#B#A#C#  "] + lines[3:]
    start = State.parse("\n".join(lines))
    ans2 = dijkstra(start)
    ctx.submit(2, ans2)
