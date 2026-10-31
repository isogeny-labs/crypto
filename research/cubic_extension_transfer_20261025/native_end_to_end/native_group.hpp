#pragma once
#include "native_inversion.hpp"
#include <dlfcn.h>
#include <memory>

namespace cubic_native {
struct NativeJacobian {
  void *library = nullptr;
  void *context = nullptr;
  void *(*create)(uint64_t, const uint64_t *, uint64_t) = nullptr;
  void (*destroy)(void *) = nullptr;
  uint64_t (*insert)(void *, const uint64_t *, const uint64_t *) = nullptr;
  uint64_t (*operation)(void *, uint32_t, uint64_t, uint64_t) = nullptr;
  int32_t (*compare)(const void *, uint64_t, uint64_t) = nullptr;
  int32_t (*export_divisor)(const void *, uint64_t, uint64_t *, uint64_t *,
                            uint64_t *) = nullptr;
  uint64_t (*certificate)(void *, uint64_t) = nullptr;
  int32_t (*solve)(void *, uint64_t, uint64_t, uint64_t, uint32_t, uint64_t,
                   uint64_t *, uint8_t *, size_t) = nullptr;
  template <class T> void symbol(T &function, const char *name) {
    function = reinterpret_cast<T>(dlsym(library, name));
    require(function != nullptr,
            std::string("missing Jacobian symbol: ") + name);
  }
  NativeJacobian(const Polynomial &g, const std::string &path, uint64_t seed) {
    library = dlopen(path.c_str(), RTLD_NOW | RTLD_LOCAL);
    if (!library) {
      const char *error = dlerror();
      throw std::runtime_error(std::string("native Jacobian load failed: ") +
                               (error ? error : "unknown"));
    }
    symbol(create, "cubic_jacobian_new");
    symbol(destroy, "cubic_jacobian_free");
    symbol(insert, "cubic_jacobian_divisor");
    symbol(operation, "cubic_jacobian_op");
    symbol(compare, "cubic_jacobian_equal");
    symbol(export_divisor, "cubic_jacobian_export");
    symbol(certificate, "cubic_jacobian_certify_order");
    symbol(solve, "cubic_jacobian_solve");
    std::array<uint64_t, 25> coefficients{};
    for (auto &[e, a] : g.terms) {
      require(e[0] + e[1] + e[2] == 4 && a.frob() == a,
              "invalid native Jacobian equation");
      coefficients[5 * e[0] + e[1]] = a.constant();
    }
    context = create(FP, coefficients.data(), seed);
    require(context != nullptr,
            "native Jacobian context rejected constructed model");
  }
  NativeJacobian(const NativeJacobian &) = delete;
  NativeJacobian &operator=(const NativeJacobian &) = delete;
  ~NativeJacobian() {
    if (context) {
      destroy(context);
    }
    if (library) {
      dlclose(library);
    }
  }
  uint64_t op(uint32_t which, uint64_t a, uint64_t b = 0) {
    uint64_t out = operation(context, which, a, b);
    require(out != 0,
            "native Jacobian arithmetic retry budget exhausted for operation " +
                std::to_string(which));
    return out;
  }
  bool equal(uint64_t a, uint64_t b) const {
    int32_t out = compare(context, a, b);
    require(out >= 0, "native divisor comparison failed");
    return out == 1;
  }
};
inline json divisor_certificate(const NativeJacobian &group, uint64_t handle) {
  std::array<uint64_t, 4> u{};
  std::array<uint64_t, 3> v{};
  std::array<uint64_t, 2> origin{};
  require(group.export_divisor(group.context, handle, u.data(), v.data(),
                               origin.data()) == 1,
          "native divisor certificate export failed");
  return {{"u", u}, {"v", v}, {"origin", origin}};
}
inline V3 transform_point(const M3 &M, const V3 &P) {
  auto out = column_times(M, P);
  require(bool(out[2]), "group projection meets a divisor at infinity");
  F unit = out[2];
  for (auto &c : out) {
    c = c / unit;
  }
  return out;
}
struct GroupProjection {
  Polynomial quartic;
  M3 coordinates;
  int attempts;
};
inline GroupProjection
group_projection(const Construction &c,
                 const std::vector<std::array<V3, 3>> &divisors) {
  std::map<std::vector<uint64_t>, V3> distinct;
  for (auto &D : divisors) {
    for (auto &P : D) {
      distinct.emplace(point_key(projective(P)), P);
    }
  }
  for (auto &contact : c.contacts.at(c.odd_index)) {
    distinct.emplace(point_key(contact.point), contact.point);
  }
  V3 origin{F(1), F(0), F(0)};
  distinct.emplace(point_key(origin), origin);
  for (int attempt = 1; attempt <= 512; attempt++) {
    M3 M{};
    for (auto &row : M) {
      for (auto &a : row) {
        a = F(long(n_randint(FR, FP)));
      }
    }
    if (!det(M)) {
      continue;
    }
    bool good = true;
    std::set<std::vector<uint64_t>> xs;
    for (auto &[key, P] : distinct) {
      (void)key;
      V3 Q = column_times(M, P);
      if (!Q[2]) {
        good = false;
        break;
      }
      F x = Q[0] / Q[2];
      if (!xs.insert(x.coeffs()).second) {
        good = false;
        break;
      }
    }
    if (!good) {
      continue;
    }
    Polynomial g = c.quartic.substitute(inverse(M)).normalized();
    if (!g.coefficient({0, 4, 0})) {
      continue;
    }
    for (auto &[key, P] : distinct) {
      (void)key;
      auto Q = transform_point(M, P);
      require(!g.eval(Q), "group projection violates quartic identity");
      if (!g.derivative(1).eval(Q)) {
        good = false;
        break;
      }
    }
    if (good) {
      g.prime_field();
      return {g, M, attempt};
    }
  }
  throw std::runtime_error(
      "common rational divisor projection budget exhausted");
}
inline ResidualDivisor effective_rur(const Polynomial &g, const M3 &M,
                                     const std::vector<Contact> &contacts) {
  EffectiveDivisor D;
  for (auto &C : contacts) {
    D.add(transform_point(M, C.point), C.multiplicity);
  }
  require(D.degree() == 3, "native group input divisor is not degree three");
  Univariate u{F(1)};
  M3 equations{};
  V3 rhs{};
  int row = 0;
  for (auto &[key, C] : D.support) {
    (void)key;
    auto P = C.point;
    require(bool(P[2]), "group divisor escaped its common affine projection");
    F affine_unit = P[2];
    for (auto &coordinate : P) {
      coordinate = coordinate / affine_unit;
    }
    require(bool(g.derivative(1).eval(P)),
            "nonprimitive divisor local parameter");
    // Here x is a uniformizer; unlike projective local_jets, its affine y
    // series must be used for Hermite interpolation of a repeated support.
    std::array<Series, 3> jet{Series(C.multiplicity), Series(C.multiplicity),
                              Series(C.multiplicity)};
    jet[0][0] = P[0];
    jet[1][0] = P[1];
    jet[2][0] = 1;
    if (C.multiplicity > 1) {
      jet[0][1] = 1;
    }
    F unit = g.derivative(1).eval(P);
    for (int n = 1; n < C.multiplicity; n++) {
      auto value = series_evaluate(g, jet);
      jet[1][n] = -value[n] / unit;
    }
    for (auto &value : series_evaluate(g, jet)) {
      require(!value, "Hermite curve jet identity failed");
    }
    for (int n = 0; n < C.multiplicity; n++) {
      for (int k = 0; k < 3; k++) {
        int binomial = n == 0 ? 1 : (n == 1 ? k : (k == 2 ? 1 : 0));
        if (k >= n) {
          equations[row][k] = F(binomial) * P[0].pow(k - n);
        }
      }
      rhs[row] = jet[1][n];
      row++;
      u = poly_mul(u, {-P[0], F(1)});
    }
  }
  V3 solution = column_times(inverse(equations), rhs);
  Univariate v{solution[0], solution[1], solution[2]};
  trim(v);
  for (auto &a : u) {
    require(a.frob() == a, "effective divisor x-polynomial is not rational");
  }
  for (auto &a : v) {
    require(a.frob() == a, "effective divisor interpolation is not rational");
  }
  require(eval_mod(g, {F(0), F(1)}, v, u).empty(),
          "Hermite divisor is off the target curve");
  return {u, v};
}
inline uint64_t insert_divisor(NativeJacobian &group,
                               const ResidualDivisor &D) {
  std::array<uint64_t, 4> u{};
  std::array<uint64_t, 3> v{};
  require(D.u.size() == 4 && D.v.size() <= 3, "invalid divisor RUR degree");
  for (size_t i = 0; i < D.u.size(); i++) {
    u[i] = D.u[i].constant();
  }
  for (size_t i = 0; i < D.v.size(); i++) {
    v[i] = D.v[i].constant();
  }
  uint64_t handle = group.insert(group.context, u.data(), v.data());
  require(handle != 0, "Rust backend rejected effective divisor certificate");
  return handle;
}
inline std::vector<Contact>
effective_contacts(const std::array<V3, 3> &points) {
  std::vector<Contact> out;
  for (auto &P : points) {
    out.push_back({P, 1});
  }
  return out;
}
} // namespace cubic_native
