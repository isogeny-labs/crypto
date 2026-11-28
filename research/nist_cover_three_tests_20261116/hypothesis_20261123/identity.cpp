// Elementary polynomial identity only; no curve experiment or performance result.
#include "identity_arithmetic.hpp"
int main(int argc, char** argv) {
    try {
        require(argc == 2, "usage: identity NEW-output-directory");
        const auto u = var("u");
        const auto c1 = var("c1"), c2 = var("c2"), c3 = var("c3"), c4 = var("c4");
        const auto lhs = add({mul({c1, u}), mul({c2, power(u, 2)}),
                              mul({c3, power(u, 3)}), mul({c4, power(u, 4)})});
        const auto rhs = mul({u, add({c1, mul({c2, u}), mul({c3, power(u, 2)}),
                                     mul({c4, power(u, 3)})})});
        issue(std::string(argv[1]) + "/root-removal.identity.json",
              "Generic zero-constant quartic root removal", {"u", "c1", "c2", "c3", "c4"},
              lhs, rhs, 5);
        return 0;
    } catch (const std::exception& error) {
        std::cerr << "REJECT " << error.what() << '\n';
        return 1;
    }
}
