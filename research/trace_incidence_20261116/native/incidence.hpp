#pragma once
#include "pullback.hpp"
#include <array>

namespace paper_ec {
struct LabelledPoint { Point point; Word alpha = 0, beta = 0; };
struct LabelledPools {
    Curve target;
    Word subgroup_order = 0;
    Point generator, public_target;
    std::array<std::vector<LabelledPoint>, 3> pools;
};
enum class RootBackend { KnownCoordinates, PaperTrace, PaperDense, Pullback };
struct IncidenceAnswer {
    bool coordinate_relation = false, informative_relation = false;
    Coefficients incidence_gcd;
    std::array<std::size_t, 3> positions{};
    std::array<int, 3> signs{};
    Word recovered_scalar = 0;
};
// Inputs are finite, correctly mixed-labelled subgroup points, with globally
// distinct x coordinates. Generation/rejection costs are separate caller work.
// All four backends share the same incidence and witness code. The auxiliary
// datum belongs to the same FIELD; it need not be isogenous to the target.
IncidenceAnswer solve_incidence(const LabelledPools&, RootBackend,
                               const PreparedDatum* auxiliary, Stats* = nullptr);
} // namespace paper_ec
