from collections import defaultdict
from puzzle import PuzzleContext

def is_small(u):
    return u == u.lower()

def dfs(adj, u, curr_path=[]):
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

def dfs2(adj, u, curr_path=[]):
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
    adj = defaultdict(list)
    for line in ctx.nonempty_lines:
        u, v = line.split("-")
        adj[u].append(v)
        adj[v].append(u)

    ctx.submit(1, dfs(adj, "start"))
    ctx.submit(2, dfs2(adj, "start"))
