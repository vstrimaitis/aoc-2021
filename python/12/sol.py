from typing import List, Dict
from collections import defaultdict
from puzzle import PuzzleContext

AdjList = Dict[str, List[str]]

def is_small(u: str) -> bool:
    return u == u.lower()

def dfs(adj: AdjList, u: str, curr_path: List[str] = []) -> int:
    if u == "end":
        return 1
    ans = 0
    curr_path.append(u)
    for v in adj[u]:
        if v == "start" or (is_small(v) and v in curr_path):
            continue
        ans += dfs(adj, v, curr_path)
    curr_path.pop()
    return ans

def dfs2(adj: AdjList, u: str, curr_path: List[str] = []) -> int:
    if u == "end":
        return 1
    ans = 0
    curr_path.append(u)
    for v in adj[u]:
        if v == "start":
            continue
        if is_small(v) and v in curr_path:
            ans += dfs(adj, v, curr_path)
        else:
            ans += dfs2(adj, v, curr_path)
    curr_path.pop()
    return ans

with PuzzleContext(year=2021, day=12) as ctx:
    adj: AdjList = defaultdict(list)
    for line in ctx.nonempty_lines:
        u, v = line.split("-")
        adj[u].append(v)
        adj[v].append(u)

    ctx.submit(1, dfs(adj, "start"))
    ctx.submit(2, dfs2(adj, "start"))
