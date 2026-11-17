#pragma once
#include "host.hpp"

namespace explicit_relations {
struct AffineCase { Point target, right, expected; };
// Independent ordinary-coordinate affine law with u128 intermediates.
inline Point reference_add(Point p, Point q, U mod, U a) {
  using W = unsigned __int128;
  if (p.inf) return q;
  if (q.inf) return p;
  if (p.x == q.x && U((W(p.y) + q.y) % mod) == 0) return {};
  U den = p.x == q.x ? U(2 * W(p.y) % mod) : U((W(q.x) + mod - p.x) % mod);
  U num = p.x == q.x ? U((3 * W(modmul(p.x, p.x, mod)) + a) % mod)
                          : U((W(q.y) + mod - p.y) % mod);
  U slope = modmul(num, modpow(den, mod - 2, mod), mod);
  U x = U((W(modmul(slope, slope, mod)) + 2 * W(mod) - p.x - q.x) % mod);
  U y = U((W(modmul(slope, U((W(p.x) + mod - x) % mod), mod)) + mod - p.y) % mod);
  return {x, y, false};
}
inline std::vector<AffineCase> affine_cases(Curve c) {
  std::vector<AffineCase> cases;
  U a = fp64_from_mont(c.f, c.a);
  auto add_case = [&](Point target, Point right) {
    Point expected = reference_add(target, c.neg(right), c.f.p, a);
    Point got = c.affine(c.affine_sub(target, right));
    Jac old = c.jac(right);
    if (old.y) old.y = c.f.p - old.y;
    Point baseline = c.affine(c.mixed(old, target));
    if (!equal(got, expected) || !equal(got, baseline) || !c.on_curve(got))
      throw std::runtime_error("affine subtraction host differential failed");
    cases.push_back({target, right, expected});
  };
  std::vector<Point> points{Point{}};
  if (c.f.p == 101) {
    for (U x = 0; x < c.f.p; ++x) {
      Point p;
      if (c.lift(x, 0, p)) {
        points.push_back(p);
        if (p.y) points.push_back(c.neg(p));
      }
    }
    for (Point p : points) for (Point q : points) add_case(p, q);
  } else {
    U seed = 0x123456789abcdefULL;
    auto next = [&]() { seed ^= seed << 13; seed ^= seed >> 7; seed ^= seed << 17; return seed; };
    for (unsigned i = 0; i < 256; ++i) {
      Point p;
      while (!c.lift(next() % c.f.p, 0, p)) {}
      points.push_back(p);
      points.push_back(c.neg(p));
    }
    for (Point p : points) {
      add_case(p, Point{}); add_case(Point{}, p);
      add_case(p, p); add_case(c.neg(p), p);
    }
    for (unsigned i = 0; i < 4096; ++i)
      add_case(points[next() % points.size()], points[next() % points.size()]);
  }
  return cases;
}
inline std::vector<Curve> affine_curves() {
  return {curve(101, 0, 7), curve(101, 98, 0),
          curve((U(1) << 63) - 25, 0, 7),
          curve((U(1) << 63) - 25, (U(1) << 63) - 28, 7)};
}
} // namespace explicit_relations
