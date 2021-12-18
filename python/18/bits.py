from typing import Tuple, List, Optional, Type
from functools import reduce
from abc import ABC, abstractmethod


class Packet(ABC):
    def __init__(self, version: int):
        self.version = version

    @classmethod
    @abstractmethod
    def type_id(cls) -> int:
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
        return None

    @abstractmethod
    def _render(self) -> str:
        pass

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
        operator_type = cls._determine_operator_type(type_id)
        if not operator_type:
            return None
        children, rest = cls._parse_children(s[6:])
        return operator_type(version, children), rest

    @classmethod
    def _determine_operator_type(cls, type_id: int) -> Optional[Type["Operator"]]:
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
            res = Packet.parse(children_part)
            assert res is not None
            p, children_part = res
            children.append(p)
        return children, rest_part

    @classmethod
    def _parse_children_1(cls, s: str) -> Tuple[List[Packet], str]:
        subpacket_count = int(s[:11], 2)
        s = s[11:]
        children = []
        for _ in range(subpacket_count):
            res = Packet.parse(s)
            assert res is not None
            pp, s = res
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
        assert len(self.children) == 2
        v1 = self.children[0].evaluate()
        v2 = self.children[1].evaluate()
        return 1 if v1 > v2 else 0


class LessThan(Operator):
    @classmethod
    def type_id(cls) -> int:
        return 6

    def evaluate(self) -> int:
        assert len(self.children) == 2
        v1 = self.children[0].evaluate()
        v2 = self.children[1].evaluate()
        return 1 if v1 < v2 else 0


class EqualTo(Operator):
    @classmethod
    def type_id(cls) -> int:
        return 7

    def evaluate(self) -> int:
        assert len(self.children) == 2
        v1 = self.children[0].evaluate()
        v2 = self.children[1].evaluate()
        return 1 if v1 == v2 else 0


def parse_hex(s: str) -> Optional[Tuple[Packet, str]]:
    s_bin = _hex_to_bin(s)
    return Packet.parse(s_bin)


def _hex_char_to_bin(c: str) -> str:
    return bin(int(c, 16))[2:].zfill(4)


def _hex_to_bin(s: str) -> str:
    return "".join(map(_hex_char_to_bin, s))


def pretty_print(p: Packet, padding_size: int = 2, curr_padding: int = 0):
    pad = " " * (curr_padding * padding_size)
    next_pad = pad + " " * padding_size
    print(f"{pad}{p.__class__.__name__}: {{")
    print(f"{next_pad}version: {p.version}")
    if isinstance(p, Literal):
        print(f"{next_pad}value: {p.value}")
    elif isinstance(p, Operator):
        print(f"{next_pad}children: [")
        for ch in p.children:
            pretty_print(ch, padding_size, curr_padding + 2)
        print(f"{next_pad}]")
    print(f"{pad}}}")


def _extract_header(s: str) -> Tuple[int, int]:
    version = int(s[:3], 2)
    type_id = int(s[3:6], 2)
    return version, type_id
