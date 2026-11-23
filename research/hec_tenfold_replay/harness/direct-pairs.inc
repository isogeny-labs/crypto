if (n == limit) {
  std::vector<int> allx, ally, pos(p, -1), root(p, -1);
  for (int y = 0; y <= p / 2; y++)
    root[m(y * y)] = y;
  for (int x = 0; x < p; x++) {
    int y =
        root[m(1LL * x * x * x * x * x + 3LL * x * x * x + 2 * x * x + x + c)];
    if (y > 0) {
      pos[x] = allx.size();
      allx.push_back(x);
      ally.push_back(y);
    }
  }
  int totaln = allx.size(), outside = totaln - n;
  for (int i = 0; i < n; i++)
    assert(xs[i] == allx[i]);
  struct Pair {
    int t[2]{}, large = 3;
  };
  std::vector<Pair> pairtable(order);
  for (auto [packed, idx] : subgroup) {
    auto key = packed;
    int b = key % p;
    key /= p;
    int a = key % p;
    key /= p;
    int v = key % p, u = key / p, r = root[m(v * v - 4 * u)];
    if (r < 0)
      continue;
    auto &pr = pairtable[idx];
    pr.large = 0;
    int prev = -1;
    Div rebuilt = identity;
    for (int h = 0; h < 2; h++) {
      int x = m((-v + (h ? -r : r)) * inv[2]), y = m(a + b * x);
      Div P{make({-x, 1}), make({y})};
      rebuilt = compose(rebuilt, P);
      if (!y)
        continue;
      int id = pos[x];
      assert(id >= 0);
      if (id >= n && id != prev)
        pr.large++;
      prev = id;
      pr.t[h] = (y == ally[id] ? -1 : 1) * (1 + id);
    }
    int coords[] = {u, v, a, b};
    assert(equal(rebuilt, readdiv(coords)));
  }
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
      int dim = mode >= 2 ? totaln : n;
      std::vector<std::vector<int>> basis(dim);
      std::vector<int> w(dim + 2);
      int f0 = 1, f1 = 0, t0 = 0, t1 = 1, pow2 = 1;
      bool ok = false;
      int j = 0;
      for (; j < 96 && !ok; j++) {
        int g = (j & 1) ? 0 : pow2, h = (j & 1) ? pow2 : 0;
        if (j & 1)
          pow2 = (2 * pow2) % order;
        int idx = (1LL * a * g + 1LL * h * b) % order;
        auto &triples = table[idx];
        auto pr = pairtable[idx];
        bool extra = mode && pr.large <= mode - 1;
        for (int rid = extra ? -1 : 0; rid < (int)triples.size(); ++rid) {
          Triple tr = rid < 0 ? Triple{pr.t[0], pr.t[1], 0} : triples[rid];
          ++s.rows;
          std::fill(w.begin(), w.end(), 0);
          for (int t : tr)
            if (t) {
              int k = std::abs(t) - 1,
                  col = dim == n ? k : (k >= n ? k - n : k + outside);
              w[col] = (w[col] + (t < 0 ? order - 1 : 1)) % order;
            }
          w[dim] = g;
          w[dim + 1] = h;
          bool stored = false;
          for (int v = 0; v < dim; v++) {
            int f = w[v];
            if (!f)
              continue;
            if (basis[v].empty()) {
              int inv = inverse(f, order);
              for (int c = v; c < dim + 2; c++)
                w[c] = w[c] * inv % order;
              basis[v] = w;
              stored = true;
              break;
            }
            for (int c = v + 1; c < dim + 2; c++)
              if (basis[v][c]) {
                int d = (w[c] - f * basis[v][c]) % order;
                w[c] = d < 0 ? d + order : d;
                ++s.ops;
              }
          }
          if (!stored && (w[dim] || w[dim + 1])) {
            assert(w[dim + 1] &&
                   (1LL * a * w[dim] + 1LL * b * w[dim + 1]) % order == 0);
            assert((order - w[dim]) * inverse(w[dim + 1], order) % order ==
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
