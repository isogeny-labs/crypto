# Conditional pairing positive control, version 1

This is a separate known supersingular control, not an ordinary/NIST curve
candidate. On y²=x³+x over F43, independently enumerate #E=44 and #E(F43²)=1936
in the native small-field code. Use r=11, k=2 and F43²=F43[w]/(w²+1). Select the
first enumerated nonidentity rational point of order 11; use the distortion
point Q=(-x(P),w*y(P)). Verify Q has order 11 and is outside the cyclic source
subgroup. Register both exact models before citing the execution.

The fixture-preparation run records the subgroup generator, field polynomial,
distortion point, native reduced Tate-pairing generator and ten known-scalar
target controls. Implement native Miller line/vertical functions, exponent 168,
nondegeneracy, order and all ten bilinearity checks. This is a correctness
control with deliberately known scalars, not a DLP performance measurement.

After freezing the prepared fixture and source, independently use PARI's native
elltatepairing and fflog on the public target coordinates. The GP script gets
no list of expected logarithms. Check each recovered scalar against its public
point in GP and then against the source group and native Miller pairing in
Rust. Keep the exact GP script, all stdout/stderr, status and native result.
Each GP child has a 15-second limit; the enclosing process has 4 GiB and the
whole control stops after two minutes. Preserve a failed parse/computation as
unresolved and amend source/protocol before any fresh retry.

This validates the positive small-embedding-degree branch of E16. It does not
construct a nonzero homomorphism to a rational twist subgroup on the NIST
parameters, supply a NIST pairing reduction, or establish any new algorithm,
novelty, full-cost speed ratio or ordinary-curve asymptotic result.
