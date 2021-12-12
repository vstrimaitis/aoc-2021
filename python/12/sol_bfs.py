from collections import defaultdict, deque
from puzzle import PuzzleContext

def is_small(u):
    return u == u.lower()

def bfs(adj, start):
    Q = deque([(start, {start})])
    ans = 0
    while Q:
        u, visited = Q.popleft()
        for v in adj[u]:
            if v == "start" or is_small(v) and v in visited:
                continue
            if v == "end":
                ans += 1
                continue
            Q.append((v, visited | {v}))
    return ans

def bfs2(adj, start):
    Q = deque([(start, {start}, True)])
    ans = 0
    while Q:
        u, visited, can_visit_two = Q.popleft()
        for v in adj[u]:
            if v == "start":
                continue
            if v == "end":
                ans += 1
                continue
            if is_small(v) and v in visited:
                if can_visit_two:
                    Q.append((v, visited, False))
            else:
                Q.append((v, visited | {v}, can_visit_two))
    return ans

with PuzzleContext(year=2021, day=12) as ctx:
    adj = defaultdict(list)
    for line in ctx.nonempty_lines:
        u, v = line.split("-")
        adj[u].append(v)
        adj[v].append(u)

    ctx.submit(1, bfs(adj, "start"))
    ctx.submit(2, bfs2(adj, "start"))
