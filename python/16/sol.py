from collections import *
from typing import *
from heapq import *
from puzzle import PuzzleContext
from bits import Packet, Operator

def get_version_sum(p: Packet) -> int:
    res = p.version
    if isinstance(p, Operator):
        res += sum(get_version_sum(x) for x in p.children)
    return res


def hex_char_to_bin(c: str) -> str:
    return bin(int(c, 16))[2:].zfill(4)


def hex_to_bin(s: str) -> str:
    return "".join(map(hex_char_to_bin, s))


with PuzzleContext(year=2021, day=16) as ctx:
    s = hex_to_bin(ctx.data)

    packet, s = Packet.parse(s)
    assert all(c == "0" for c in s)

    ctx.submit(1, get_version_sum(packet))
    ctx.submit(2, packet.evaluate())
