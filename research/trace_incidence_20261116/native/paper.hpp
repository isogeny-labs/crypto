#pragma once
#include "api.hpp"

namespace paper_ec {
struct PrimePower { Word prime = 0; unsigned exponent = 0; };
struct GroupBasis {
    Point first, second;
    Word first_order = 1, second_order = 1;
};
struct Datum {
    Word beta = 0;
    Curve curve, twist;
    Word order = 0, twist_order = 0;
    std::vector<PrimePower> factors, twist_factors;
    GroupBasis basis, twist_basis;
};
// Native domain: prime 3 < p < 2^63. Validation is separately chargeable
// preprocessing. Throws invalid_argument; no group/field enumeration is used.
void validate_datum(const Datum&, Stats* = nullptr);
class PreparedDatum {
public:
    explicit PreparedDatum(const Datum& datum, Stats* setup = nullptr);
    const Datum& get() const { return datum_; }
private:
    Datum datum_;
};
struct Embedding { Coefficients u0, u1, v0, v1; };
// Assumes monic split squarefree f, coprime to X^3+aX+b.
Embedding finite_field_embedding(const Curve&, const Coefficients& f,
                                Word nonsquare, Stats* = nullptr);
// Division-free normalized recurrence. No factor/root enumeration is used.
Coefficients division_polynomial_mod(const Curve&, Word index,
    const Coefficients& x, const Coefficients& y,
    const Coefficients& modulus, Stats* = nullptr);
// Input must be monic, split and squarefree; this is checked algebraically.
// Online code never factors or enumerates the field/group.
std::vector<Word> root_find(const PreparedDatum&, const Coefficients&,
                           TranslationMethod, Stats* = nullptr);
Datum paper_f83_datum();
} // namespace paper_ec
