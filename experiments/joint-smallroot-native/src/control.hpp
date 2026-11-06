// SPDX-License-Identifier: LGPL-3.0-or-later
#pragma once
#include "pipeline.hpp"
namespace jsr {
inline Q evaluate_planted(const Poly &f) {
  Q value = 0;
  for (auto &t : f.t) {
    bool zero = false;
    for (size_t i = 1; i < t.first.size(); ++i)
      if (t.first[i])
        zero = true;
    if (!zero) {
      Z p;
      mpz_ui_pow_ui(p.get_mpz_t(), 101, static_cast<unsigned>(t.first[0]));
      value += t.second * p;
    }
  }
  return value;
}
inline void control_construction(int r, int mu, const fs::path &output,
                                 const fs::path &singular) {
  Point Qtarget = add(Point{41, 15, false}, Point{41, 15, false});
  Cell c =
      cell(r, mu, "joint_A", {1, 3, 1, 3, 41, 41}, static_cast<int>(Qtarget.x));
  require(fs::create_directory(output), "fresh control construction output");
  Archive a{output, c.order};
  a.object("canonical-input", canonical_input(c));
  a.object("input.sha256", sha256(canonical_input(c)) + "\n");
  Backend b{singular, output / "singular", c.order, c.names, 0, {}};
  for (auto &f : c.F) {
    Q q = evaluate_planted(f);
    require(q.get_den() == 1 &&
                mpz_divisible_ui_p(q.get_num().get_mpz_t(), 101),
            "planted original equation failed");
  }
  auto generators = c.F;
  generators.push_back(Poly::variable(7, 0));
  auto powers = a.phase("control-ideal-power",
                        [&]() { return ideal_power(generators, mu); });
  a.polys("ideal-power-generators", powers);
  auto G = a.phase("control-groebner", [&]() { return b.groebner(powers); });
  a.polys("ordered-basis", G);
  auto domain = a.phase("control-domain", [&]() {
    return hull(initial_vertices(c), [&](const Domain &d) { a.domain(d); });
  });
  auto closure = a.phase("control-closure-echelon", [&]() {
    return optimal(
        domain.points, G, c.order,
        [&](const Poly &f) { return b.normal_form(f); },
        [&](const Closure &x) { a.closure(x); });
  });
  a.closure(closure);
  Z modulus;
  mpz_ui_pow_ui(modulus.get_mpz_t(), 101, static_cast<unsigned>(mu));
  std::string receipt;
  for (size_t i = 0; i < closure.S.size(); ++i) {
    auto &f = closure.S[i];
    Z denom = 1;
    for (auto &t : f.t)
      mpz_lcm(denom.get_mpz_t(), denom.get_mpz_t(),
              t.second.get_den().get_mpz_t());
    Z gcd;
    mpz_gcd_ui(gcd.get_mpz_t(), denom.get_mpz_t(), 101);
    require(gcd == 1, "control denominator noninvertible modulo101");
    Q value = evaluate_planted(scale(f, Q(denom)));
    require(value.get_den() == 1, "control denominator clearing failed");
    bool divisible =
        mpz_divisible_p(value.get_num().get_mpz_t(), modulus.get_mpz_t());
    receipt += "row=" + std::to_string(i) + " denominator=" + denom.get_str() +
               " cleared_value=" + value.get_num().get_str() +
               " modulus=" + modulus.get_str() +
               " divisible=" + (divisible ? "true" : "false") + "\n";
    a.object("divisibility", receipt);
    require(divisible, "control constructed shift not divisible by101^mu");
  }
  a.object("status", "complete-control-only; no bound interpolation; "
                     "extraction not_requested\n");
}
inline void control_panel(const fs::path &output, const fs::path &singular) {
  require(fs::create_directory(output), "fresh control output");
  write(output / "sign-tree-control", prepared_control());
  for (int r : {2, 4})
    for (int mu : {1, 2})
      control_construction(
          r, mu,
          output / ("r" + std::to_string(r) + "-mu" + std::to_string(mu)),
          singular);
  write(output / "status", "complete-control-only\n");
}
} // namespace jsr
