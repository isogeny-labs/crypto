#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Shared progression setup inverse
T-1889 · Prime-field coefficient construction · 2026-11-08

Progression setup now obtains the grid-step and factorial inverses from one product inverse. Both existing verification multiplications remain. This applies at each setup reached by the interval engine, including the coefficient-stream path.

#image("flow.svg", width: 100%)

*Identity.* For prime r, nonzero step s and 1 ≤ d < r, F=d! is nonzero. Set u=(sF)^(r−2). Then Fu=s⁻¹ and su=F⁻¹. Substitution preserves the samples, inverse-factorial recurrence and Lagrange weights. No new vector or retained cache field is introduced.

*Source-derived arithmetic change.* The binary exponentiation helper uses B(n)=bit_length(n)+popcount(n) multiplication calls, including its final square. Replacing two inverses by one adds three multiplications, yielding B(r−2)−3 fewer scalar multiplication calls per setup. This is 2 at r=7, 10 at r=97, 13 at r=257 and 29 at r=65537. These are derived code counts, not runtime observations.

*Qualification.* The new controls cover 234 setups, 2,556 sample/weight identities and 151,020 evaluation values, with 45 setups at degree r−1. They require one Fermat-inverse entry per setup and retain five invalid-domain controls. Zero polynomials and all first points in the selected small fields are included. Native receipt records the inherited regression result separately.

*Evidence.* Reversible source edits and compiler dependency hashes bind the implementation. The inherited recovery checker is compared byte for byte with preserved output. Producer assertions remain unsigned pending independent signed verification.

*Scope and next step.* Prime-field preconditions remain caller obligations. No comparative width panel or cloud/timing workload ran. Future width comparisons must bind this same progression implementation in every arm. The larger source/coefficient cost accounting remains separate from this local arithmetic reduction.
