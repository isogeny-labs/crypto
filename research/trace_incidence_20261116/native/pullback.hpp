#pragma once
#include "paper.hpp"

namespace paper_ec {
// Keep the original root coordinate as the quotient-ring parameter throughout
// Sylow descent. No image-polynomial or inverse-composition reconstruction.
// Datum construction/validation is charged separately through PreparedDatum.
std::vector<Word> root_find_pullback(const PreparedDatum&,
                                   const Coefficients&, Stats* = nullptr);
} // namespace paper_ec
