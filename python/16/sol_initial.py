from collections import *
from typing import *
from heapq import *
from functools import reduce
from puzzle import PuzzleContext

class Packet:
    type_id: str
    id: int
    version: int

class Literal(Packet):
    value: int

    def __str__(self):
        return f"Literal(type={self.type_id}, version={self.version}, value={self.value})"

    def __repr__(self) -> str:
        return self.__str__()

class Operator(Packet):
    children: List[Packet]

    def __str__(self):
        return f"Operator(type_id={self.type_id}, version={self.version}, children={self.children})"

    def __repr__(self) -> str:
        return self.__str__()

def parse_literal(s: str) -> Tuple[Packet, str]:
    i = 0
    res = ""
    while i < len(s):
        flag = s[i]
        res += s[i+1:i+5]
        i += 5
        if flag == "0":
            break
    p = Literal()
    p.value = int(res, 2)
    return p, s[i:]

def parse_operator_0(s: str) -> Tuple[Packet, str]:
    l = int(s[:15], 2)
    p = Operator()
    p.children = parse(s[15:15+l])
    return p, s[15+l:]


def parse_operator_1(s: str) -> Tuple[Packet, str]:
    n_sub = int(s[:11], 2)
    p = Operator()
    p.children = []
    s = s[11:]
    for _ in range(n_sub):
        pp, ss = parse_1(s)
        p.children.append(pp)
        s = ss
    return p, s
    

def parse_operator(s: str) -> Tuple[Packet, str]:
    l = s[0]
    if l == "0":
        return parse_operator_0(s[1:])
    return parse_operator_1(s[1:])

def parse_1(s: str) -> Tuple[Packet, str]:
    v = s[:3]
    t = s[3:6]
    if t == "100":
        p, ss = parse_literal(s[6:])
        p.type_id = int(t, 2)
        p.version = int(v, 2)
        return p, ss
    else:
        p, ss = parse_operator(s[6:])
        p.type_id = int(t, 2)
        p.version = int(v, 2)
        return p, ss

def parse(s: str) -> List[Packet]:
    res = []
    while len(s) > 0:
        if all(c == "0" for c in s):
            break 
        p, ss = parse_1(s)
        s = ss
        res.append(p)
    return res

def get_version_sum(p: Packet) -> int:
    res = p.version
    if isinstance(p, Operator):
        res += sum(get_version_sum(x) for x in p.children)
    return res

def evaluate(p: Packet) -> int:
    if isinstance(p, Literal):
        assert isinstance(p, Literal)
        return p.value
    assert isinstance(p, Operator)
    if p.type_id == 0:
        return sum(evaluate(x) for x in p.children)
    if p.type_id == 1:
        return reduce(lambda a, b: a*b, [evaluate(x) for x in p.children], 1)
    if p.type_id == 2:
        return min(evaluate(x) for x in p.children)
    if p.type_id == 3:
        return max(evaluate(x) for x in p.children)
    if p.type_id == 5:
        v1 = evaluate(p.children[0])
        v2 = evaluate(p.children[1])
        return 1 if v1 > v2 else 0
    if p.type_id == 6:
        v1 = evaluate(p.children[0])
        v2 = evaluate(p.children[1])
        return 1 if v1 < v2 else 0
    if p.type_id == 7:
        v1 = evaluate(p.children[0])
        v2 = evaluate(p.children[1])
        return 1 if v1 == v2 else 0

with PuzzleContext(year=2021, day=16) as ctx:
    s = "".join([bin(int(c, 16))[2:].zfill(4) for c in ctx.data])
    
    packets = parse(s)
    assert len(packets) == 1
    packet = packets[0]
    
    ctx.submit(1, get_version_sum(packet))
    ctx.submit(2, evaluate(packet))
