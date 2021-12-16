from collections import *
from typing import *
from heapq import *
from puzzle import PuzzleContext
from bits import Packet, Operator, parse_hex

def get_version_sum(p: Packet) -> int:
    res = p.version
    if isinstance(p, Operator):
        res += sum(get_version_sum(x) for x in p.children)
    return res


with PuzzleContext(year=2021, day=16) as ctx:
    packet, s = parse_hex(ctx.data)
    assert all(c == "0" for c in s)

    ctx.submit(1, get_version_sum(packet))
    ctx.submit(2, packet.evaluate())
