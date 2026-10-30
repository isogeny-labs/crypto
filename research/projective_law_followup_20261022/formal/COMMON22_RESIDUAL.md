# Common 2+2 returned-ordinate congruence

Schema: `projective-law-common22-residual/v1`. Lean proves the exact cleared
polynomial identity `C⁵N + S C²X + T = U[-C⁴X + C³(-Cq₁-A) + C²Q]` over any
commutative ring. This is the cleared congruence between the source-shaped
ordinate numerators and cubic numerator modulo the residual quadratic; it
uses no field division. In the unit-weight affine specialization, nonzero
`B,C` allow the usual negative-interpolant interpretation. Normalization of
arbitrary weighted outputs remains unproved here. The [build log](common22-residual-build.log) records the kernel
check and foundational axiom output.

The source checker extracts the common Rust 2+2 branch, verifies its return
tuple, proves that the source `r` has a `C²` factor, and compares the source
`qdelta`, `rdelta`, `s`, and `t` with the theorem's definitions after exact
polynomial substitution. All five obligations match. Mutating the `s` sign
on either side is rejected. The [certificate](common22-residual.json) binds
both source hashes and five polynomial digests; the [summary CSV](common22-residual-summary.csv)
and [graph](common22-residual.svg) retain the exact monomial counts. The
[PDF](COMMON22_RESIDUAL.pdf) and 3.3 MB [term-level CSV](common22-residual-terms.csv)
are stored through the repository artifact manifest in this directory; use
`python3 tools/artifacts/artifacts.py pull research/projective_law_followup_20261022/formal`
from the repository root before checking it.

This proves the common branch's cleared ordinate congruence and its stated
source obligations. It does not yet prove that `U` is the residual support
for a valid smooth curve, that the interpolation function has the required
principal divisor, or that every exceptional runtime branch returns the
Jacobian sum. The full 1,260-target operation-price ratios are unchanged.
