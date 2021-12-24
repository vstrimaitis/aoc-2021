from collections import *
from typing import *
from heapq import *
from puzzle import PuzzleContext


def run(program, inputs, initial_mem=None, start_ip=0):
    mem = {"w": 0, "x": 0, "y": 0, "z": 0, "ip": 0}
    if initial_mem is not None:
        mem = initial_mem.copy()
    next_input_idx = 0
    ip = start_ip
    if "ip" in mem:
        ip = mem["ip"]
    while ip < len(program):
        line = program[ip]
        mem["ip"] = ip
        ip += 1
        if not line or line.startswith("#"):
            continue
        if "#" in line:
            line = line.split("#", 1)[0].strip()
        cmd, args = line.split(" ", 1)
        if cmd == "inp":
            if next_input_idx >= len(inputs):
                return ip-1, mem
            mem[args] = inputs[next_input_idx]
            next_input_idx += 1
        elif cmd == "add":
            a, b = args.split(" ")
            mem[a] += mem[b] if b in "xyzw" else int(b)
        elif cmd == "mul":
            a, b = args.split(" ")
            mem[a] *= mem[b] if b in "xyzw" else int(b)
        elif cmd == "div":
            a, b = args.split(" ")
            mem[a] //= mem[b] if b in "xyzw" else int(b)
        elif cmd == "mod":
            a, b = args.split(" ")
            mem[a] %= mem[b] if b in "xyzw" else int(b)
        elif cmd == "eql":
            a, b = args.split(" ")
            mem[a] = 1 if mem[a] == (mem[b] if b in "xyzw" else int(b)) else 0
        else:
            assert False, "invalid command"
    mem["ip"] = ip
    return ip, mem


def encode(d: dict):
    return (d["ip"], d["x"], d["y"], d["z"], d["w"])

def decode(t) -> dict:
    return {
        "ip": t[0],
        "x": t[1],
        "y": t[2],
        "z": t[3],
        "w": t[4], 
    }


with PuzzleContext(year=2021, day=24) as ctx:
    program = ctx.data.split("\n")

    # # part 1
    # all_mems = dict()
    # all_mems[encode({"w": 0, "x": 0, "y": 0, "z": 0, "ip": 0})] = ([], 0)
    
    # min_z = 10**100
    # max_z = 0
    # for _ in range(14):
    #     new_mems = dict()
    #     min_here = 10**1000
    #     for m, (prev_ds, start_ip) in all_mems.items():
    #         m = decode(m)
    #         for d in range(1, 10):
    #             new_ds = prev_ds + [d]
    #             ip, mem = run(program, [d], m)
    #             new_mems[encode(mem)] = (new_ds, ip)
    #             if mem["z"] == 0:
    #                 print("!!!", new_ds)
    #             if mem["z"] < min_here:
    #                 min_here = mem["z"]
    #     print("min z here: ", min_here)
    #     all_mems = new_mems
    #     print(len(all_mems))

    # part 2
    all_mems = dict()
    all_mems[encode({"w": 0, "x": 0, "y": 0, "z": 0, "ip": 0})] = ([], 0)
    
    min_z = 10**100
    max_z = 0
    for _ in range(14):
        new_mems = dict()
        min_here = 10**1000
        for m, (prev_ds, start_ip) in all_mems.items():
            m = decode(m)
            for d in reversed(range(1, 10)):
                new_ds = prev_ds + [d]
                ip, mem = run(program, [d], m)
                new_mems[encode(mem)] = (new_ds, ip)
                if mem["z"] == 0:
                    print("!!!", new_ds)
                if mem["z"] < min_here:
                    min_here = mem["z"]
        print("min z here: ", min_here)
        all_mems = new_mems
        print(len(all_mems))
