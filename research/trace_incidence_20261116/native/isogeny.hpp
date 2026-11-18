#pragma once
#include "paper.hpp"

namespace paper_ec {
// Classical degree-two Velu map, with explicit rational kernel {O,T}.
// This changes an auxiliary model, not its rational point count or smoothness.
struct TwoIsogeny {
    Curve domain, codomain;
    Point kernel;
    Word kernel_coefficient = 0; // 3*x(T)^2+a
};
TwoIsogeny two_isogeny(const Curve&, Point rational_two_torsion, Stats* = nullptr);
Point isogeny_image(const TwoIsogeny&, Point, Stats* = nullptr);

// Exhaustive, explicitly counted SMALL-FIELD fixture construction only.
// Refuses p>257. No general polynomial-time datum construction is asserted.
Datum construct_toy_datum(const Curve&, Word nonsquare, Stats* = nullptr);
} // namespace paper_ec
