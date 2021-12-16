#include <bits/stdc++.h>
using namespace std;

typedef pair<int, int> pii;
template <typename T> using min_heap = priority_queue<T, vector<T>, greater<T>>;

const int oo = 1000000000;
const vector<pii> dirs = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

int mod1(int x, int m) {
    return (x-1)%m+1;
}

int dijkstra(const vector<vector<int>>& b) {
    int n = b.size();
    int m = b[0].size();
    min_heap<pair<int, pii>> Q;
    auto dists = vector<vector<int>>(n, vector<int>(m, oo));

    dists[0][0] = 0;
    Q.push({0, {0, 0}});
    while(!Q.empty()) {
        auto t = Q.top(); Q.pop();
        int d = t.first;
        int i = t.second.first;
        int j = t.second.second;
        if (d > dists[i][j]) continue;
        if (i == n-1 && j == m-1) return d;
        for (auto dir : dirs) {
            int di = dir.first;
            int dj = dir.second;
            int ii = i + di;
            int jj = j + dj;
            if (ii >= 0 && ii < n && jj >= 0 && jj < m && d + b[ii][jj] < dists[ii][jj]) {
                dists[ii][jj] = d + b[ii][jj];
                Q.push({dists[ii][jj], {ii, jj}});
            }
        }
    }
    return -1;
}

vector<vector<int>> expand(const vector<vector<int>>& b) {
    int n = b.size();
    int m = b[0].size();
    auto res = vector<vector<int>>(5*n, vector<int>(5*m));
    for(int i = 0; i < 5*n; i++) {
        for(int j = 0; j < 5*m; j++) {
            res[i][j] = mod1(b[i%n][j%m] + i/n + j/m, 9);
        }
    }
    return res;
}

double toMillis(std::chrono::steady_clock::time_point t1, std::chrono::steady_clock::time_point t2) {
    auto micros = std::chrono::duration_cast<std::chrono::microseconds>(t2-t1).count();
    return double(micros)/1e3;
}

std::chrono::steady_clock::time_point getTime() {
    return std::chrono::steady_clock::now();
}

void bench1(const vector<vector<int>>& board) {
    const int ITERS = 100;
    double durationSum = 0;
    double minDuration = oo;
    int prevAns = oo;
    for(int i = 0; i < ITERS; i++) {
        auto tStart1 = getTime();
        auto ans = dijkstra(board);
        auto tEnd1 = getTime();
        auto d = toMillis(tStart1, tEnd1);

        durationSum += toMillis(tStart1, tEnd1);
        minDuration = min(minDuration, d);

        if (prevAns == oo) {
            prevAns = ans;
        } else if(prevAns != ans) {
            cout << "WTF" << endl;
            exit(1);
        }
    }
    cout << "Part 1 average duration over " << ITERS << " runs: " << durationSum/ITERS << " ms, min: " << minDuration << " ms" << endl;
}

void bench2(const vector<vector<int>>& board) {
    const int ITERS = 100;
    double durationSum = 0;
    double minDuration = oo;
    int prevAns = oo;
    for(int i = 0; i < ITERS; i++) {
        auto tStart1 = getTime();
        auto bigBoard = expand(board);
        auto ans = dijkstra(bigBoard);
        auto tEnd1 = getTime();
        auto d = toMillis(tStart1, tEnd1);

        durationSum += d;
        minDuration = min(minDuration, d);

        if (prevAns == oo) {
            prevAns = ans;
        } else if(prevAns != ans) {
            cout << "WTF" << endl;
            exit(1);
        }
    }
    cout << "Part 2 average duration over " << ITERS << " runs: " << durationSum/ITERS << " ms, min: " << minDuration << " ms" << endl;
}

int main() {
    auto tStart = getTime();
    ios_base::sync_with_stdio(false); cin.tie(nullptr);
    vector<vector<int>> board;
    string line;
    while (cin >> line) {
        board.push_back({});
        for (char c : line) {
            board.back().push_back(c-'0');
        }
    }
    auto tEndIO = getTime();
    auto durationIO = toMillis(tStart, tEndIO);
    cout << ">>> IO time: " << fixed << setprecision(6) << durationIO << " ms" << endl;

    bench1(board);
    bench2(board);

    // auto tStart1 = getTime();
    // cout << "Part 1: " << dijkstra(board) << endl;
    // auto tEnd1 = getTime();
    // auto duration1 = toMillis(tStart1, tEnd1);
    // cout << ">>> Part 1 time: " << fixed << setprecision(6) << duration1 << " ms" << endl;
    
    // auto tStart2 = getTime();
    // board = expand(board);
    // cout << "Part 2: " << dijkstra(board) << endl;
    // auto tEnd2 = getTime();
    // auto duration2 = toMillis(tStart2, tEnd2);
    // cout << ">>> Part 2 time: " << fixed << setprecision(6) << duration2 << " ms" << endl;

    // auto duration = toMillis(tStart1, tEnd2);
    // cout << ">>> Full time: " << fixed << setprecision(6) << duration << " ms" << endl;

    // auto durationWithIO = toMillis(tStart, tEnd2);
    // cout << ">>> Full time (with IO): " << fixed << setprecision(6) << durationWithIO << " ms" << endl;
}
