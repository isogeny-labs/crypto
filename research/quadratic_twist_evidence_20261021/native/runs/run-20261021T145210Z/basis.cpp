#define TWIST_FULL_NO_MAIN
#include "full.cpp"

// Independently encode the tower used by the frozen subfield fixture in a
// simple polynomial basis before issuing a representation identity for it.
int main(int argc, char **argv) {
  try {
    require(argc == 2, "usage: basis OUTPUT_RECEIPT");
    auto K = std::make_shared<Field>(7, 3);
    Field tower(K, 8), simple(7, std::vector<I>{5, 0, 5, 0, 1});
    // u^2=3 and v^2=1+u.  Thus w=v, u=w^2-1.
    auto convert = [&](I x) {
      I a = x % 7, b = (x / 7) % 7, c = (x / 49) % 7, d = x / 343;
      return simple.encode({a - b, c - d, b, d});
    };
    std::set<I> image_set;
    I field_checks = 0;
    for (I x = 0; x < tower.size; ++x) {
      image_set.insert(convert(x));
      for (I e : {1, 7, 49, 343}) {
        require(convert(tower.mul(x, e)) == simple.mul(convert(x), convert(e)),
                "basis multiplication");
        require(convert(tower.add(x, e)) == simple.add(convert(x), convert(e)),
                "basis addition");
        ++field_checks;
      }
    }
    require(image_set.size() == 2401, "basis bijectivity");
    Curve A(tower, 1, 1), B(simple, 1, 1), source(*K, 18, 45);
    auto point_map = [&](Point P) {
      return P.infinity() ? P : Point{convert(P.x), convert(P.y)};
    };
    I point_checks = 0, group_checks = 0;
    for (auto P : A.points()) {
      require(B.contains(point_map(P)), "basis point equation");
      ++point_checks;
    }
    require(point_checks == 2475 && B.points().size() == 2475,
            "basis point count");
    I ix = tower.inv(8), iy = tower.inv(tower.mul(8, 49));
    auto psi = [&](Point P) {
      return P.infinity() ? P
                          : point_map({tower.mul(P.x, ix), tower.mul(P.y, iy)});
    };
    for (auto P : source.points())
      for (auto Q : source.points()) {
        require(psi(source.add(P, Q)) == B.add(psi(P), psi(Q)),
                "subfield simple-basis group map");
        ++group_checks;
      }
    Point P{3, 17}; // (3,3+2u), as independently found by the full replay.
    require(source.contains(P) && source.scalar(3, P).infinity(),
            "source order-three fixture");
    auto mapped = psi(P);
    require(!mapped.infinity() && B.scalar(3, mapped).infinity(),
            "mapped order-three fixture");
    auto receipt = obj({{"schema", str("quadratic-twist-basis-certificate/v1")},
                        {"status", str("passed")},
                        {"p", num(7)},
                        {"monic_modulus", nums({5, 0, 5, 0, 1})},
                        {"field_elements_bijected", num(2401)},
                        {"basis_field_checks", num(field_checks)},
                        {"point_equations_checked", num(point_checks)},
                        {"group_law_checks", num(group_checks)},
                        {"subgroup_order", num(3)},
                        {"source_generator", nums({P.x, P.y})},
                        {"mapped_generator", arr()}});
    receipt.fields["mapped_generator"].items = {nums(simple.digits(mapped.x)),
                                                nums(simple.digits(mapped.y))};
    std::ofstream out(argv[1]);
    require(bool(out), "basis receipt output");
    out << dump(receipt) << '\n';
    std::cout << "PASS: tower/simple basis; 2401 elements, 2475 points, 2025 "
                 "group-law checks\n";
    return 0;
  } catch (const std::exception &e) {
    std::cerr << "FAIL: " << e.what() << '\n';
    return 1;
  }
}
