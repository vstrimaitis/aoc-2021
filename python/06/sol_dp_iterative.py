from puzzle import PuzzleContext

def solve(initial_timers, n_days):
    dp = [[0 for _ in range(n_days+1)] for _ in range(9)]
    for j in range(n_days+1):
        for i in range(9):
            dp[i][j] = 1 + sum(dp[8][j-k] for k in range(i+1, j+1, 7))
    ans = 0
    for x in initial_timers:
        ans += dp[x][n_days]
    return ans


with PuzzleContext(year=2021, day=6) as ctx:
    arr = [int(x) for x in ctx.data.split(",")]
    ctx.submit(1, solve(arr, 80))
    ctx.submit(2, solve(arr, 256))
