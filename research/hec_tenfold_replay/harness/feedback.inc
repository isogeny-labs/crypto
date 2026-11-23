if (n == limit) {
  struct Stat {
    long long ops = 0, rows = 0, censored = 0;
    std::vector<int> stop;
  };
  std::array<Stat, 4> stats{};
  std::vector<std::array<int, 4>> paired;
  std::mt19937_64 rng(2026111907ULL + p), randomization(2026111914ULL + p);
  std::uniform_int_distribution<int> sample(1, order - 1);
  for (int trial = 0; trial < 8192; trial++) {
    int a = sample(rng), b = sample(rng);
    std::array<int, 4> stops{};
    for (int mode = 0; mode < 4; mode++) {
      auto &s = stats[mode];
      std::vector<std::vector<int>> basis(n);
      std::vector<int> w(n + 2);
      int f0 = 1, f1 = 1, t0 = 1, t1 = 2, pow2 = 2, dindex = 0;
      int pulls[2]{}, pivots[2]{};
      bool ok = false;
      int j = 0;
      for (; j < 96 && !ok; j++) {
        int g = 0, h = 0, arm = -1;
        if (j < 2) {
          g = j == 0;
          h = j == 1;
        } else {
          if (mode == 0)
            arm = 0;
          else if (mode == 1)
            arm = 1;
          else if (mode == 2 || j < 10)
            arm = (j - 2) % 2;
          else {
            int left = pivots[0] * pulls[1], right = pivots[1] * pulls[0];
            arm = left == right ? (pulls[0] > pulls[1]) : (right > left);
          }
          ++pulls[arm];
          if (arm == 0) {
            g = f0;
            h = t0;
            int x = (f0 + f1) % order, y = (t0 + t1) % order;
            f0 = f1;
            f1 = x;
            t0 = t1;
            t1 = y;
          } else {
            g = (dindex & 1) ? 0 : pow2;
            h = (dindex & 1) ? pow2 : 0;
            if (dindex & 1)
              pow2 = (2 * pow2) % order;
            ++dindex;
          }
        }
        for (const auto &tr : table[(1LL * a * g + 1LL * h * b) % order]) {
          ++s.rows;
          std::fill(w.begin(), w.end(), 0);
          for (int t : tr)
            w[std::abs(t) - 1] = t < 0 ? order - 1 : 1;
          w[n] = g;
          w[n + 1] = h;
          bool stored = false;
          for (int v = 0; v < n; v++) {
            int f = w[v];
            if (!f)
              continue;
            if (basis[v].empty()) {
              int inv = inverse(f, order);
              for (int c = v; c < n + 2; c++)
                w[c] = w[c] * inv % order;
              basis[v] = w;
              stored = true;
              if (arm >= 0)
                ++pivots[arm];
              break;
            }
            for (int c = v + 1; c < n + 2; c++)
              if (basis[v][c]) {
                int d = (w[c] - f * basis[v][c]) % order;
                w[c] = d < 0 ? d + order : d;
                ++s.ops;
              }
          }
          if (!stored && (w[n] || w[n + 1])) {
            assert(w[n + 1] &&
                   (1LL * a * w[n] + 1LL * b * w[n + 1]) % order == 0);
            assert((order - w[n]) * inverse(w[n + 1], order) % order ==
                   b * inverse(a, order) % order);
            ok = true;
            break;
          }
        }
      }
      s.censored += !ok;
      s.stop.push_back(j);
      stops[mode] = j;
    }
    paired.push_back(stops);
  }
  for (int mode = 0; mode < 4; mode++) {
    auto &s = stats[mode];
    std::sort(s.stop.begin(), s.stop.end());
    int wins = 0, ties = 0, loss = 0;
    long long sum = 0;
    for (auto v : paired) {
      wins += v[mode] < v[0];
      ties += v[mode] == v[0];
      loss += v[mode] > v[0];
      sum += v[mode];
    }
    std::printf("first %d %d %lld %lld %lld %lld %d %d %d %d %d %d\n", p, mode,
                sum, s.rows, s.ops, s.censored, s.stop[4095], s.stop[7782],
                s.stop.back(), wins, ties, loss);
  }
}
