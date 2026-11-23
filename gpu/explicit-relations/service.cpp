// Persistent line protocol, intentionally separate from ecbench's counted CPU
// methods. CUDA operation counts are not relabelled as GroupOps or GAE.
#ifdef ER_CUDA
#include "device.cuh"
#else
#include "host.hpp"
#endif
#include <iostream>
#include <set>
#include <sstream>

using namespace explicit_relations;
#ifdef ER_CUDA
using Backend = DeviceBackend;
#else
using Backend = HostBackend;
#endif

static U number(std::istream &in) {
  std::string token;
  if (!(in >> token) || token.empty() || token[0] == '-')
    throw std::runtime_error("missing unsigned integer");
  size_t used = 0;
  U v = std::stoull(token, &used, token.rfind("0x", 0) == 0 ? 16 : 10);
  if (used != token.size())
    throw std::runtime_error("invalid unsigned integer");
  return v;
}
static void end(std::istream &in) {
  std::string extra;
  if (in >> extra)
    throw std::runtime_error("unexpected trailing input");
}
static Point point(std::istream &in) {
  U x = number(in), y = number(in);
  return {x, y, false};
}
static void print_base(const std::vector<Point> &ps) {
  std::cout << "BASE " << ps.size();
  for (Point p : ps)
    std::cout << ' ' << p.x << ' ' << p.y;
  std::cout << std::endl;
}
static void require(bool ok, const char *message) {
  if (!ok)
    throw std::runtime_error(message);
}
// Canonical affine reference: deliberately does not use Montgomery/Jacobian.
static Point independent_add(Point a, Point b, U p, U ca) {
  if (a.inf)
    return b;
  if (b.inf)
    return a;
  if (a.x == b.x && (a.y + b.y) % p == 0)
    return {};
  U den = a.x == b.x ? 2 * a.y % p : (b.x + p - a.x) % p;
  U num = a.x == b.x ? U((3 * (unsigned __int128)modmul(a.x, a.x, p) + ca) % p) : (b.y + p - a.y) % p;
  U slope = modmul(num, modpow(den, p - 2, p), p);
  U x = U(((unsigned __int128)modmul(slope, slope, p) + 2 * (unsigned __int128)p - a.x - b.x) % p);
  return {x, (modmul(slope, (a.x + p - x) % p, p) + p - a.y) % p, false};
}
static void validate_caps(U cap, U chunk) {
  if (!cap || cap > (U(1) << 32) || !chunk || chunk > cap)
    throw std::runtime_error("row caps must be in 1..2^32, chunk <= cap");
}
static void selftest() {
  validate_caps(1, 1);
  validate_caps(U(1) << 32, U(1) << 20);
  validate_caps(U(1) << 32, U(1) << 32);
  for (auto caps : {std::pair<U, U>{0, 1}, {U(1) << 32 | 1, 1},
                    {1, 0}, {1, 2}}) {
    bool rejected = false;
    try {
      validate_caps(caps.first, caps.second);
    } catch (const std::runtime_error &) {
      rejected = true;
    }
    require(rejected, "invalid row/chunk cap accepted");
  }
  for (U modulus : {U(1) << 62, (U(1) << 62) + 57,
                    (U(1) << 63) - 25, (U(1) << 63) + 29, UINT64_MAX - 58}) {
    for (U a : {U(0), U(1), modulus / 2, modulus - 2, modulus - 1})
      for (U b : {U(0), U(1), modulus / 2, modulus - 2, modulus - 1})
        require((modulus < (U(1) << 63) ? hash_mul63(a, b, modulus) : hash_mul64(a, b, modulus)) == modmul(a, b, modulus),
                "wide hash multiply disagrees with u128");
  }
  // Public arithmetic fixtures, not certified group-order fixtures.
  for (U modulus : {(U(1) << 61) - 1, (U(1) << 62) - 57, (U(1) << 63) - (U(1) << 34) - 1}) {
    while (!prime(modulus))
      modulus -= 2;
    Curve c = curve(modulus, modulus - 3, 7);
    Backend device(c, 100000, 257);
    HostBackend host(c, 100000, 257);
    for (U initial : {2 * modulus, (U(1) << 63) - 25, (U(1) << 63) + 29, UINT64_MAX - 58}) {
      U s = initial;
      while (!prime(s))
        ++s;
      if (s < 2 * modulus || (unsigned __int128)s > 4 * (unsigned __int128)modulus)
        continue;
      for (U alpha : {U(1), U(17), s - 1})
        for (U beta : {U(0), s / 2, s - 1}) {
          hash_parameters(c, s, alpha, beta, 1024, 100000);
          auto got = device.hash(s, alpha, beta, 1024);
          auto expected = host.hash(s, alpha, beta, 1024);
          require(got.size() == expected.size(), "wide hash size mismatch");
          for (size_t j = 0; j < got.size(); ++j)
            require(equal(got[j], expected[j]), "wide hash point mismatch");
        }
    }
    for (U bad_s : {U(1) << 63, U(-1)}) {
      bool rejected = false;
      try { hash_parameters(c, bad_s, 17, 0, 1, 100000); }
      catch (const std::runtime_error &) { rejected = true; }
      require(rejected, "unsupported hash modulus accepted");
    }
  }
  for (U modulus : {97, 211}) {
    U ca = modulus == 97 ? 2 : 0, cb = modulus == 97 ? 3 : 7;
    Curve c = curve(modulus, ca, cb);
    Backend device(c, 100000, 257);
    HostBackend host(c, 100000, 257);
    std::vector<Point> all;
    for (U x = 0; x < modulus; ++x)
      for (U y = 0; y < modulus; ++y)
        if (modmul(y, y, modulus) ==
            (modmul(modmul(x, x, modulus), x, modulus) + ca * x + cb) % modulus)
          all.push_back({x, y, false});
    U s = 2 * modulus;
    while (!prime(s))
      ++s;
    for (U tau : {U(1), s / 3, s}) {
      auto got = device.hash(s, 17, 23, tau);
      std::vector<Point> expected;
      for (Point p : all)
        if ((17 * (2 * p.x + (p.y & 1)) + 23) % s < tau)
          expected.push_back(p);
      require(got.size() == expected.size(), "hash size mismatch");
      for (size_t j = 0; j < got.size(); ++j)
        require(equal(got[j], expected[j]), "hash point mismatch");
    }
    std::vector<U> xs;
    for (Point p : all)
      if (xs.empty() || xs.back() != p.x)
        xs.push_back(p.x);
    auto lifts = device.lift(xs);
    require(lifts.size() == all.size(), "explicit lift size mismatch");
    for (size_t j = 0; j < all.size(); ++j)
      require(equal(lifts[j], all[j]), "explicit lift mismatch");
    bool missing_rejected = false;
    for (U x = 0; x < modulus; ++x)
      if (!std::binary_search(xs.begin(), xs.end(), x)) {
        try {
          device.lift({x});
        } catch (const std::runtime_error &) {
          missing_rejected = true;
        }
        break;
      }
    require(missing_rejected, "nonlifting x accepted");
    bool excess_slots_rejected = false;
    try {
      plan(c, std::vector<std::vector<Point>>(MAX_SLOTS + 1, {all[0]}), 100000);
    } catch (const std::runtime_error &e) {
      excess_slots_rejected = std::string(e.what()) == "invalid base slots or row cap";
    }
    require(excess_slots_rejected, "excess slot count accepted");
    for (unsigned slots = 2; slots <= MAX_SLOTS; ++slots) {
      std::vector<std::vector<Point>> bases(slots);
      for (unsigned j = 0; j < slots; ++j) {
        // Same points in separate slots, both signs, duplicated sums.
        bases[j] = slots <= 8
            ? std::vector<Point>{all[0], all[1], all[2], all[3], all[4], all[5], all[6]}
            : std::vector<Point>{all[0], all[1]};
      }
      Plan p = plan(c, bases, 100000);
      device.prepare(p);
      host.prepare(p);
      auto targets = all;
      targets.push_back({});
      for (Point target : targets) {
        Result a = device.find(target, 100000), b = host.find(target, 100000);
        require(a.found == b.found && a.indices == b.indices,
                "GPU/reference deterministic witness differs");
        if (a.found) {
          Point sum;
          for (unsigned j = 0; j < slots; ++j)
            sum = independent_add(sum, bases[j][a.indices[j]], modulus, ca);
          require(equal(sum, target), "independent affine replay failed");
        }
      }
      bool limited = false;
      try {
        device.find({}, 0);
      } catch (const std::runtime_error &) {
        limited = true;
      }
      require(limited, "zero work budget accepted");
      bases[0].clear();
      p = plan(c, bases, 100000);
      device.prepare(p);
      require(!device.find({}, 0).found, "empty base has witness");
    }
    // Exhaustive misses force multiple chunks even if larger bases happen
    // to cover every target in their first chunk.
    Backend small(c, 100000, 3);
    std::vector<std::vector<Point>> narrow = {
        {all[0]}, {all[0], all[1], all[2], all[3], all[4], all[5], all[6]}};
    Plan p = plan(c, narrow, 100000);
    small.prepare(p);
    host.prepare(p);
    bool exhausted = false;
    for (Point target : all) {
      Result got = small.find(target, 100000),
             expected = host.find(target, 100000);
      require(got.found == expected.found && got.indices == expected.indices,
              "chunk boundary changed witness");
      if (!got.found) {
        require(got.visited == p.right, "negative result before exhaustion");
        exhausted = true;
      }
    }
    require(exhausted, "missing exhaustive negative fixture");
  }
  std::cout << "PASS arithmetic, explicit lifts, signed hashes, all-target "
               "relations, deterministic witnesses, empty bases, limits"
            << std::endl;
}

static void serve() {
  std::string line, cmd;
  if (!std::getline(std::cin, line))
    throw std::runtime_error("missing curve header");
  std::istringstream header(line);
  header >> cmd;
  if (cmd != "curve")
    throw std::runtime_error("expected curve");
  U modulus = number(header), a = number(header), b = number(header),
    cap = number(header), chunk = number(header);
  end(header);
  validate_caps(cap, chunk);
  Curve c = curve(modulus, a, b);
  Backend backend(c, cap, chunk);
  Plan p;
  bool prepared = false;
  std::cout << "READY 1" << std::endl;
  while (std::getline(std::cin, line)) {
    std::istringstream in(line);
    in >> cmd;
    if (cmd == "quit") {
      end(in);
      return;
    }
    if (cmd == "bases") {
      U m = number(in);
      if (m < 2 || m > MAX_SLOTS)
        throw std::runtime_error("slot count must be 2..12");
      std::vector<std::vector<Point>> bases(m);
      for (auto &base : bases) {
        U n = number(in);
        if (n > cap)
          throw std::runtime_error("LIMIT base size");
        base.reserve(n);
        for (U i = 0; i < n; ++i)
          base.push_back(point(in));
      }
      end(in);
      p = plan(c, bases, cap);
      backend.prepare(p);
      prepared = true;
      std::cout << "PREPARED " << p.split << ' ' << p.left << ' ' << p.right
                << std::endl;
    } else if (cmd == "find") {
      if (!prepared)
        throw std::runtime_error("prepare bases first");
      U inf = number(in);
      Point target = point(in);
      U work = number(in);
      end(in);
      if (inf > 1 || (inf && (target.x || target.y)))
        throw std::runtime_error("noncanonical infinity");
      target.inf = inf;
      Result r = backend.find(target, work);
      if (r.found) {
        std::cout << "FOUND " << r.indices.size();
        for (U i : r.indices)
          std::cout << ' ' << i;
      } else
        std::cout << "NONE";
      std::cout << std::endl;
      std::cerr << "{\"stage\":\"relation\",\"left_rows\":" << p.left
                << ",\"right_rows_visited\":" << r.visited << "}\n";
    } else if (cmd == "xs") {
      U n = number(in);
      if (!n || n > cap / 2)
        throw std::runtime_error("LIMIT explicit x size");
      std::vector<U> xs;
      std::set<U> seen;
      for (U i = 0; i < n; ++i) {
        U x = number(in);
        if (x >= modulus || !seen.insert(x).second)
          throw std::runtime_error("invalid or duplicate x");
        xs.push_back(x);
      }
      end(in);
      print_base(backend.lift(xs));
    } else if (cmd == "hash") {
      U s = number(in), alpha = number(in), beta = number(in), tau = number(in);
      end(in);
      hash_parameters(c, s, alpha, beta, tau, cap);
      print_base(backend.hash(s, alpha, beta, tau));
    } else
      throw std::runtime_error("unknown command");
  }
}
int main(int argc, char **argv) {
  try {
    if (argc == 2 && std::string(argv[1]) == "--self-test")
      selftest();
    else if (argc == 1)
      serve();
    else
      throw std::runtime_error("usage: explicit-relations [--self-test]");
    return 0;
  } catch (const std::exception &e) {
    // Error/limit terminates the process and invalidates its cache. Never
    // print NONE after allocation failure, CUDA error, or incomplete work.
    std::cout << "ERROR " << e.what() << std::endl;
    std::cerr << e.what() << '\n';
    return 1;
  }
}
