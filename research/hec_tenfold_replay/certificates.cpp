#include <algorithm>
#include <array>
#include <cassert>
#include <cstdio>
#include <queue>
#include <random>
#include <unordered_map>
#include <vector>
int inverse(int x, int p) {
  int b = p, u = 1, v = 0;
  while (b) {
    int q = x / b, r = x % b;
    x = b;
    b = r;
    int w = u - q * v;
    u = v;
    v = w;
  }
  return (u % p + p) % p;
}
struct Poly {
  int a[16]{}, n = 0;
};
int prime;
int red(long long v) { return (v % prime + prime) % prime; }
Poly trim(Poly a) {
  while (a.n && !a.a[a.n - 1])
    --a.n;
  return a;
}
Poly make(std::initializer_list<int> v) {
  Poly a;
  for (int x : v)
    a.a[a.n++] = red(x);
  return trim(a);
}
Poly addp(Poly a, Poly b, int sg = 1) {
  a.n = std::max(a.n, b.n);
  for (int i = 0; i < b.n; i++)
    a.a[i] = red(a.a[i] + sg * b.a[i]);
  return trim(a);
}
Poly mulp(Poly a, Poly b) {
  Poly r;
  if (!a.n || !b.n)
    return r;
  r.n = a.n + b.n - 1;
  assert(r.n <= 16);
  for (int i = 0; i < a.n; i++)
    for (int j = 0; j < b.n; j++)
      r.a[i + j] = red(r.a[i + j] + 1LL * a.a[i] * b.a[j]);
  return trim(r);
}
Poly scale(Poly a, int k) {
  for (int i = 0; i < a.n; ++i)
    a.a[i] = red(1LL * a.a[i] * k);
  return trim(a);
}
std::pair<Poly, Poly> divrem(Poly a, Poly b) {
  assert(b.n);
  Poly q;
  int inv = inverse(b.a[b.n - 1], prime);
  while (a.n >= b.n) {
    int j = a.n - b.n, v = red(1LL * a.a[a.n - 1] * inv);
    q.n = std::max(q.n, j + 1);
    q.a[j] = v;
    for (int i = 0; i < b.n; i++)
      a.a[i + j] = red(a.a[i + j] - 1LL * b.a[i] * v);
    a = trim(a);
  }
  return {q, a};
}
Poly exact(Poly a, Poly b) {
  auto qr = divrem(a, b);
  assert(!qr.second.n);
  return qr.first;
}
struct Ext {
  Poly g, s, t;
};
Ext eg(Poly a, Poly b) {
  Poly s = make({1}), u{}, t{}, v = make({1});
  while (b.n) {
    auto qr = divrem(a, b);
    a = b;
    b = qr.second;
    auto x = addp(s, mulp(qr.first, u), -1), y = addp(t, mulp(qr.first, v), -1);
    s = u;
    u = x;
    t = v;
    v = y;
  }
  int f = inverse(a.a[a.n - 1], prime);
  return {scale(a, f), scale(s, f), scale(t, f)};
}
struct Div {
  Poly u, v;
};
Poly curve;
Div compose(Div a, Div b) {
  auto h = eg(a.u, b.u);
  auto z = eg(h.g, addp(a.v, b.v));
  auto u = exact(mulp(a.u, b.u), mulp(z.g, z.g));
  auto v = addp(
      mulp(z.s, addp(mulp(h.s, mulp(a.u, b.v)), mulp(h.t, mulp(b.u, a.v)))),
      mulp(z.t, addp(mulp(a.v, b.v), curve)));
  v = divrem(exact(v, z.g), u).second;
  while (u.n > 3) {
    u = exact(addp(curve, mulp(v, v), -1), u);
    u = scale(u, inverse(u.a[u.n - 1], prime));
    v = divrem(scale(v, -1), u).second;
  }
  return {u, v};
}
Div readdiv(int *x) { return {make({x[0], x[1], 1}), make({x[2], x[3]})}; }
bool equal(Div a, Div b) {
  return !addp(a.u, b.u, -1).n && !addp(a.v, b.v, -1).n;
}
Div times(Div a, int k) {
  Div r{make({1}), {}};
  while (k) {
    if (k & 1)
      r = compose(r, a);
    k >>= 1;
    if (k)
      a = compose(a, a);
  }
  return r;
}
void count(int p, int c, int order, int *d, int limit) {
  prime = p;
  curve = make({c, 1, 2, 3, 0, 1});
  auto m = [&](long long x) { return int((x % p + p) % p); };
  std::vector<int> inv(p), xs, ys;
  for (int i = 1; i < p; i++)
    inv[i] = inverse(i, p);
  for (int x = 0; x < p; x++) {
    for (int y = 1; y <= p / 2; y++)
      if (m(1LL * x * x * x * x * x + 3LL * x * x * x + 2 * x * x + x + c -
            y * y) == 0) {
        xs.push_back(x);
        ys.push_back(y);
        break;
      }
    if ((int)xs.size() == limit)
      break;
  }
  auto key = [&](int u, int v, int a, int b) {
    return ((1LL * u * p + v) * p + a) * p + b;
  };
  std::unordered_map<long long, int> subgroup;
  Div D = readdiv(d), Q{make({1}), {}}, identity = Q;
  for (int k = 0; k < order; k++) {
    if (k)
      assert(!equal(Q, identity));
    if (Q.u.n == 3)
      assert(subgroup.emplace(key(Q.u.a[0], Q.u.a[1], Q.v.a[0], Q.v.a[1]), k)
                 .second);
    Q = compose(Q, D);
  }
  assert(equal(Q, identity));
  using Triple = std::array<int, 3>;
  std::vector<std::vector<Triple>> table(order);
  std::vector<int> freq(order), orig(order), expect(order);
  long long total = 0, eligible = 0, overlaps = 0;
  int n = xs.size();
  for (int i = 0; i < n; i++)
    for (int j = 0; j < i; j++)
      for (int k = 0; k < j; k++) {
        int x = xs[i], y = xs[j], z = xs[k], s = m(x + y + z),
            pairs = m(x * y + x * z + y * z);
        for (int sg = 0; sg < 8; sg++) {
          int a = (sg & 1) ? ys[i] : -ys[i], b = (sg & 2) ? ys[j] : -ys[j],
              c0 = (sg & 4) ? ys[k] : -ys[k];
          int t = m((b - a) * inv[m(y - x)]), t2 = m((c0 - b) * inv[m(z - y)]),
              h2 = m((t2 - t) * inv[m(z - x)]), h1 = m(t - h2 * (x + y)),
              h0 = m(a - h1 * x - h2 * x * x);
          int v = m(s - h2 * h2), u = m(3 - 2 * h2 * h1 + s * v - pairs),
              va = m(h0 - h2 * u), vb = m(h1 - h2 * v);
          ++total;
          auto it = subgroup.find(key(u, v, va, vb));
          if (it == subgroup.end())
            continue;
          int idx = it->second;
          expect[idx]++;
          eligible++;
          table[idx].push_back({(sg & 1 ? 1 : -1) * (i + 1),
                                (sg & 2 ? 1 : -1) * (j + 1),
                                (sg & 4 ? 1 : -1) * (k + 1)});
          if (!m(x * x + v * x + u) || !m(y * y + v * y + u) ||
              !m(z * z + v * z + u))
            overlaps++;
        }
      }
  int used = 0, missed_classes = 0;
  for (auto [id, k] : subgroup) {
    long long packed = id;
    int b = packed % p;
    packed /= p;
    int a = packed % p;
    packed /= p;
    int v = packed % p, u = packed / p;
    int e2 = m(-v), e1 = m(3 - u - e2 * v), e0 = m(2 - b * b - e1 * v - e2 * u);
    std::vector<int> bins(p), regular(p);
    for (int j = 0; j < n; j++) {
      int x = xs[j], y = ys[j], ux = m(x * x + v * x + u), vx = m(a + b * x);
      if (ux) {
        for (int sign : {-1, 1}) {
          int t = m((sign * y - vx) * inv[ux]);
          bins[t]++;
          regular[t]++;
        }
      } else {
        assert(m(vx * vx) == m(y * y));
        int g = m(((x + e2) * x + e1) * x + e0);
        bins[m(g * inv[m(2 * vx)])]++;
      }
    }
    for (int t = 0; t < p; t++) {
      assert(bins[t] <= 3);
      freq[k] += bins[t] == 3;
      orig[k] += regular[t] == 3;
    }
    assert(freq[k] == expect[k]);
    used += freq[k] > 0;
    missed_classes += freq[k] > orig[k];
  }
  if (n == limit) {
    struct Row {
      Triple terms;
      int first, side;
    };
    struct Stats {
      long long solved = 0, manyfail = 0, fewpass = 0, fewfail = 0, empty = 0,
                rows = 0, core = 0, ops = 0, coreops = 0;
    };
    std::array<int, 4> sizes = {16, 24, 36, 64};
    std::array<std::array<Stats, 4>, 3> counts{};
    std::mt19937_64 rng(2026111902ULL + p);
    std::uniform_int_distribution<int> sample(1, order - 1);
    for (int trial = 0; trial < 2048; trial++) {
      int a = sample(rng), b = sample(rng);
      for (int mode = 0; mode < 3; mode++)
        for (int which = 0; which < 4; which++) {
          auto &s = counts[mode][which];
          std::vector<Row> rows;
          std::vector<std::vector<int>> incident(n);
          int f0 = 1, f1 = 0, t0 = 0, t1 = 1;
          for (int j = 0; j < sizes[which]; j++) {
            int g = 1, h = j;
            if (mode == 1) {
              g = 1 + (j & 1);
              h = j / 2;
            } else if (mode == 2) {
              g = f0;
              h = t0;
              int nx = (f0 + f1) % order, ny = (t0 + t1) % order;
              f0 = f1;
              f1 = nx;
              t0 = t1;
              t1 = ny;
            }
            for (const auto &tr : table[(1LL * a * g + 1LL * h * b) % order]) {
              int id = rows.size();
              for (int e : tr)
                incident[std::abs(e) - 1].push_back(id);
              rows.push_back({tr, g, h});
            }
          }
          int nr = rows.size();
          std::vector<char> live(nr, 1);
          std::vector<int> deg(n);
          std::queue<int> one;
          for (int v = 0; v < n; v++) {
            deg[v] = incident[v].size();
            if (deg[v] == 1)
              one.push(v);
          }
          int left = nr;
          while (!one.empty()) {
            int v = one.front();
            one.pop();
            if (deg[v] != 1)
              continue;
            for (int id : incident[v])
              if (live[id]) {
                live[id] = 0;
                --left;
                for (int e : rows[id].terms) {
                  int t = std::abs(e) - 1;
                  if (--deg[t] == 1)
                    one.push(t);
                }
                break;
              }
          }
          auto solve = [&](bool peeled, long long &ops) {
            std::vector<std::vector<int>> basis(n);
            std::vector<int> w(n + 2);
            for (int id = 0; id < nr; id++) {
              if (peeled && !live[id])
                continue;
              std::fill(w.begin(), w.end(), 0);
              for (int t : rows[id].terms)
                w[std::abs(t) - 1] = t < 0 ? order - 1 : 1;
              w[n] = rows[id].first;
              w[n + 1] = rows[id].side;
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
                  break;
                }
                for (int c = v + 1; c < n + 2; c++)
                  if (basis[v][c]) {
                    int d = (w[c] - f * basis[v][c]) % order;
                    w[c] = d < 0 ? d + order : d;
                    ++ops;
                  }
              }
              if (!stored && (w[n] || w[n + 1])) {
                assert(w[n + 1] &&
                       (1LL * a * w[n] + 1LL * b * w[n + 1]) % order == 0);
                assert((order - w[n]) * inverse(w[n + 1], order) % order ==
                       b * inverse(a, order) % order);
                return true;
              }
            }
            return false;
          };
          bool ok = solve(false, s.ops);
          assert(solve(true, s.coreops) == ok);
          s.solved += ok;
          s.manyfail += !ok && nr > n;
          s.fewpass += ok && nr <= n;
          s.fewfail += !ok && nr <= n;
          s.empty += left == 0;
          s.rows += nr;
          s.core += left;
        }
    }
    for (int mode = 0; mode < 3; mode++)
      for (int i = 0; i < 4; i++) {
        auto &s = counts[mode][i];
        std::printf("mode%d ", mode);
        std::printf(
            "rank %d %d %d %lld %lld %lld %lld %lld %lld %lld %lld %lld\n", p,
            n, sizes[i], s.solved, s.manyfail, s.fewpass, s.fewfail, s.empty,
            s.rows, s.core, s.ops, s.coreops);
      }
  }
  if (n == limit) {
    constexpr int len = 36;
    long long total_y = 0, total_y2 = 0, first = 0, second = 0, shortfall = 0;
    int ymin = 100000, ymax = 0;
    for (int v : freq) {
      total_y += v;
      total_y2 += v * v;
    }
    for (int b = 1; b <= order / 2; b++) {
      int s = 0, head = 0, tail = 0;
      for (int j = 0; j < len; j++) {
        s += freq[tail];
        tail += b;
        if (tail >= order)
          tail -= order;
      }
      for (int a = 0; a < order; a++) {
        first += 2 * s;
        second += 2LL * s * s;
        shortfall += 2 * (s <= n);
        ymin = std::min(ymin, s);
        ymax = std::max(ymax, s);
        s += freq[tail] - freq[head];
        tail += b;
        head += b;
        if (tail >= order)
          tail -= order;
        if (head >= order)
          head -= order;
      }
    }
    assert(first == len * (order - 1LL) * total_y);
    assert(second ==
           len * (order - 1LL) * total_y2 +
               1LL * len * (len - 1) * (total_y * total_y - total_y2));
    std::printf("progressions %d %d %d %d %d %lld %lld\n", p, n, len, ymin,
                ymax, shortfall, 1LL * order * (order - 1));
  }
  if (n == limit) {
    struct Stat {
      long long ops = 0, rows = 0, censored = 0;
      std::vector<int> stop;
    };
    std::array<Stat, 4> stats{};
    std::vector<std::array<int, 4>> paired;
    std::mt19937_64 rng(2026111904ULL + p), randomization(2026111914ULL + p);
    std::uniform_int_distribution<int> sample(1, order - 1);
    for (int trial = 0; trial < 8192; trial++) {
      int a = sample(rng), b = sample(rng);
      std::array<int, 4> stops{};
      for (int mode = 0; mode < 4; mode++) {
        auto &s = stats[mode];
        std::vector<std::vector<int>> basis(n);
        std::vector<int> w(n + 2);
        int f0 = 1, f1 = 0, t0 = 0, t1 = 1, pow2 = 1;
        bool ok = false;
        int j = 0;
        for (; j < 96 && !ok; j++) {
          int g = 1, h = j;
          if (mode == 1) {
            g = f0;
            h = t0;
            int x = (f0 + f1) % order, y = (t0 + t1) % order;
            f0 = f1;
            f1 = x;
            t0 = t1;
            t1 = y;
          }
          if (mode == 2) {
            g = (j & 1) ? 0 : pow2;
            h = (j & 1) ? pow2 : 0;
            if (j & 1)
              pow2 = (2 * pow2) % order;
          }
          if (mode == 3) {
            g = sample(randomization);
            h = sample(randomization);
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
      std::printf("first %d %d %lld %lld %lld %lld %d %d %d %d %d %d\n", p,
                  mode, sum, s.rows, s.ops, s.censored, s.stop[4095],
                  s.stop[7782], s.stop.back(), wins, ties, loss);
    }
  }
  long long old = 0;
  for (int v : orig)
    old += v;
  assert(eligible - old == overlaps);
  assert(total == 8LL * n * (n - 1) * (n - 2) / 6);
  std::printf("%d %d %d %zu %lld %lld %lld %lld %d %d\n", p, n, order,
              subgroup.size(), total, eligible, old, overlaps, used,
              missed_classes);
}
int main() {
  {
    int d[] = {115, 121, 79, 20, 108, 67, 27, 67, 120, 42, 106, 54};
    for (int limit : {26})
      count(127, 4, 8161, d, limit);
  }
  {
    int d[] = {42, 167, 234, 222, 205, 108, 241, 83, 32, 187, 25, 101};
    for (int limit : {52})
      count(257, 4, 17783, d, limit);
  }
}
