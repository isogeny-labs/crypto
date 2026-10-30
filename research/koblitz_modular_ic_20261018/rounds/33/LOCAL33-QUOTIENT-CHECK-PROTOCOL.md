# Source-bound quotient specialization, prospective

Only consume a completed quotient producer accepted by exit/marker/error
gates. Never use a partial live export. Read all four quotient/lift objects,
verify their identities against the same four source generators again, and
collect every coefficient denominator of every source, quotient and lift.
Export a deduplicated native list of those polynomials in u, with short=0.
Native formal roundtrip checks the small specialization function (all quotient
generators, source Jacobian, source values, H and old D). Python only filters,
concatenates and lexically renames serialized expressions; it does no field
or verifier arithmetic.

At both fixed padding points, and then every fixed degree83 twist point,
evaluate all denominator polynomials FIRST and require each nonzero. Check
plane inverse, all four source equations, old D/H units, and all four maximal
Jacobian minors. Form the product family (quotient coefficient)*(matching
minor), now including omission4. Require exact unit-ideal lift in the AS
residue algebra and preserve all slots via matrix multiplication. A corrupt
zero product family must be rejected. The generic lift identities specialize
because every one of their denominators has been checked; no enormous lift
needs re-evaluation for that implication. Both the original generic identity
and source-bound specialized coefficients are retained.

Success gives local equality with a three-equation standard-etale
presentation on a cover of every branch, provided the preceding original
Tate/plane change identities have the checked units. Proper global polarized
embedding and quotient saturation remain separate. A failure is recorded,
not waived to run jets. All native invocations4GiB/1200s; fixed complete
point list, no statistical or timing comparison, no cap increase.
