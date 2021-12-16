from collections import *
from typing import *
from heapq import *
from functools import reduce
from puzzle import PuzzleContext
from abc import ABC, abstractclassmethod, abstractmethod


def _extract_header(s: str) -> Tuple[int, int]:
    version = int(s[:3], 2)
    type_id = int(s[3:6], 2)
    return version, type_id


class Packet(ABC):
    def __init__(self, version: int):
        self.version = version

    @abstractclassmethod
    def type_id(self) -> int:
        pass

    @abstractmethod
    def evaluate(self) -> int:
        pass

    @classmethod
    @abstractmethod
    def parse(cls, s: str) -> Optional[Tuple["Packet", str]]:
        for subclass in cls.__subclasses__():
            res = subclass.parse(s)
            if res:
                return res

    def _render(self) -> str:
        return f"Packet(type={self.type_id()}, version={self.version})"

    def __str__(self):
        return self._render()

    def __repr__(self) -> str:
        return str(self)


class Literal(Packet):
    def __init__(self, version: int, value: int):
        super().__init__(version)
        self.value = value

    @classmethod
    def type_id(cls) -> int:
        return 4

    @classmethod
    def parse(cls, s: str) -> Optional[Tuple[Packet, str]]:
        version, type_id = _extract_header(s)
        if type_id != 4:
            return None
        i = 6
        res = ""
        while i < len(s):
            flag = s[i]
            res += s[i + 1 : i + 5]
            i += 5
            if flag == "0":
                break
        packet = cls(version, int(res, 2))
        return packet, s[i:]

    def evaluate(self) -> int:
        return self.value

    def _render(self) -> str:
        return f"Literal(version={self.version}, value={self.value})"


class Operator(Packet):
    def __init__(self, version: int, children: List[Packet]):
        super().__init__(version)
        self.children = children
        self._name = self.__class__.__name__

    @classmethod
    def parse(cls, s: str) -> Optional[Tuple[Packet, str]]:
        version, type_id = _extract_header(s)
        operator = cls._determine_operator_type(type_id)
        if not operator:
            return None
        children, rest = cls._parse_children(s[6:])
        return operator(version, children), rest

    @classmethod
    def _determine_operator_type(cls, type_id: int) -> Optional[Type[Packet]]:
        subclasses = [sub for sub in cls.__subclasses__() if sub.type_id() == type_id]
        if len(subclasses) != 1:
            return None
        return subclasses[0]

    @classmethod
    def _parse_children(cls, s: str) -> Tuple[List[Packet], str]:
        length_type_id = s[0]
        if length_type_id == "0":
            return cls._parse_children_0(s[1:])
        else:
            return cls._parse_children_1(s[1:])

    @classmethod
    def _parse_children_0(cls, s: str) -> Tuple[List[Packet], str]:
        subpackets_bit_length = int(s[:15], 2)
        children_part = s[15 : 15 + subpackets_bit_length]
        rest_part = s[15 + subpackets_bit_length :]
        children = []
        while len(children_part) > 0:
            p, children_part = Packet.parse(children_part)
            children.append(p)
        return children, rest_part

    @classmethod
    def _parse_children_1(cls, s: str) -> Tuple[List[Packet], str]:
        subpacket_count = int(s[:11], 2)
        s = s[11:]
        children = []
        for _ in range(subpacket_count):
            pp, s = Packet.parse(s)
            children.append(pp)
        return children, s

    def _render(self) -> str:
        return f"{self._name}(version={self.version}, children={self.children})"


class Sum(Operator):
    @classmethod
    def type_id(cls) -> int:
        return 0

    def evaluate(self) -> int:
        return sum(x.evaluate() for x in self.children)


class Product(Operator):
    @classmethod
    def type_id(cls) -> int:
        return 1

    def evaluate(self) -> int:
        return reduce(lambda a, b: a * b, (x.evaluate() for x in self.children), 1)


class Minimum(Operator):
    @classmethod
    def type_id(cls) -> int:
        return 2

    def evaluate(self) -> int:
        return min(x.evaluate() for x in self.children)


class Maximum(Operator):
    @classmethod
    def type_id(cls) -> int:
        return 3

    def evaluate(self) -> int:
        return max(x.evaluate() for x in self.children)


class GreaterThan(Operator):
    @classmethod
    def type_id(cls) -> int:
        return 5

    def evaluate(self) -> int:
        v1 = self.children[0].evaluate()
        v2 = self.children[1].evaluate()
        return 1 if v1 > v2 else 0


class LessThan(Operator):
    @classmethod
    def type_id(cls) -> int:
        return 6

    def evaluate(self) -> int:
        v1 = self.children[0].evaluate()
        v2 = self.children[1].evaluate()
        return 1 if v1 < v2 else 0


class EqualTo(Operator):
    @classmethod
    def type_id(cls) -> int:
        return 7

    def evaluate(self) -> int:
        v1 = self.children[0].evaluate()
        v2 = self.children[1].evaluate()
        return 1 if v1 == v2 else 0


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
