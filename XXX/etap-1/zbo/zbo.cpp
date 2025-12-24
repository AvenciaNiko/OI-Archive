#include <algorithm>
#include <iostream>
#include <map>
#include <vector>

using namespace std;

struct Village {
  map<Village *, int> connections;

  Village() {}
};

int dfs(vector<Village *> visited, Village *current, Village *target,
        int cost) {
  vector<pair<Village *, int>> next_search;

  visited.push_back(current);
  for (auto con : current->connections) {
    if (con.first == target) {
      return cost + con.second;
    } else if (find(visited.begin(), visited.end(), con.first) ==
               visited.end()) {
      next_search.push_back(con);
    }
  }

  for (auto cur : next_search) {
    int x = dfs(visited, cur.first, target, cost + cur.second);

    if (x != -1) {
      return x;
    }
  }

  return -1;
}

int main() {
  int v_num, k_num;
  vector<Village> villages;
  vector<int> cur_kings = {0};
  int result = 0;

  cin >> v_num >> k_num;

  for (int i = 0; i < v_num; i++) {
    villages.push_back(Village());
  }

  for (int i = 0; i < v_num - 1; i++) {
    int a, b, c;
    cin >> a >> b >> c;

    villages[a - 1].connections[&villages[b - 1]] = c;
    villages[b - 1].connections[&villages[a - 1]] = c;
  }

  Village *vil_ptr = &villages[0];

  for (int i = 0; i < k_num; i++) {
    int king;
    cin >> king;
    king -= 1;

    for (int target : cur_kings) {
      result += dfs({}, vil_ptr + king, vil_ptr + target, 0) * 2;
    }

    cout << result << '\n';
    cur_kings.push_back(king);
  }

  return 0;
}
