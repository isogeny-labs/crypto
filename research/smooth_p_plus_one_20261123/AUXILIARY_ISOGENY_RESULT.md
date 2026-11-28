# Auxiliary elliptic-curve chains and dense fiber polynomials

2026-11-25 · `auxiliary_isogeny_v1` · Native software qualification · PR #5142

A separate [Rust library](auxiliary-isogeny/src/lib.rs) now constructs actual
auxiliary elliptic-curve two-isogeny chains and the composed rational map.
It returns a regular coset's fiber polynomial and separately lifts its roots
to a target curve. **Nine unit tests pass, and the release library builds.**
The executed fixtures are F7 and F31. This implements the published
auxiliary-curve construction; no scientific solver panel, relation recovery
or performance comparison is included. [pcc:unverified]

The existing Dickson implementation and all 20 qualified source/input pins
remain unchanged. The new module is not yet connected to its RR5 collector.
The allowed-prime API includes 7, 31, 127 and 8191; the latter two have no new
auxiliary-base selection panel in this component. Depth is capped at six,
and the image generator must retain order at least four.

| Unit control | Checked behavior |
|:--|:--|
| F7, kernel (0,0) | (3,3) maps to (1,5); kernel and infinity map to infinity |
| F7 group law | The map respects all 64 pairs of the eight known group points |
| F31 chain | G=(6,6) has order 32; the second kernel has x=2 |
| F31, three steps | Numerator degree 8, denominator degree 7; chained/dense values agree at all 31 x, including poles |
| Regular F31 coset | The degree-eight fiber polynomial equals the product of its eight distinct roots |
| Negative/lifting controls | Corrupted coefficient, invalid inputs, bad fiber images and target two-torsion are handled explicitly |

![Two representations of the same auxiliary-curve fiber. Numerical degrees label the F31 unit fixture, not a timing experiment. Target lifting remains a separate operation.](auxiliary-isogeny-flow.pdf){width=95% height=3.5in}

\newpage

## Construction, evidence and costs

For E': y²=x³+x and the allowed primes, p is 3 modulo 4 and p+1 is a power
of two. The character-pairing argument and the single nonidentity rational
two-torsion point give a cyclic group of order p+1. The implementation also
checks the supplied generator's full order. Its deterministic search exposes
candidate counts; those are setup counters, not CPU-work measurements.

After translating the current order-two point to z=0, each map uses

\[
u=z+a+b/z,\quad v=y(1-b/z^2),\qquad
v^2=u^3-2au^2+(a^2-4b)u.
\]

The next kernel is selected from the current image of the original generator;
it need not have x=0. For a composed N/D, put Z=N-rD and replace the pair by
\(Z^2+aZD+bD^2,\ ZD\). At depth d the degrees are \(2^d,2^d-1\).
For a nonzero, non-two-torsion image with x-coordinate chi, the regular fiber
is the monic polynomial N-chi D. The protocol explains the standard coset
and denominator arguments. Infinity is represented explicitly, and excluded
image cases return errors instead of affine artifacts. No checked Lean
formalization is claimed.

The primary [PKC 2016 paper, Section 3.4](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf)
already supplies isogeny-defined factor bases; the official
[Sage reference](https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_curve_isogeny.html)
describes the pointed-morphism isogeny contract. The formulas and elementary
derivation here instantiate that established construction. KB status and two
queries were unavailable (wrapper exit 127), with the receipt retained.

```text
source: 2cb1f120c5e5bd3665a844cdaa1fec46d2b3bcd7
```

[PCC-20261125-583348](../../pcc/claims/PCC-20261125-583348.json) pins the
test/build logs and [build identity](evidence/auxiliary_isogeny_v1/build_identity.json).
It remains unsigned `not_covered`; local verification finds no unexpected
proof reasons. The build identity binds seven source/evidence files and the
library/test artifacts. The 1,911-entry PCC ledger chain passes locally.
This is producer-local software evidence. [pcc:unverified]

The first test invocation exited 101 before compilation because its lockfile
was absent after a preceding prose-lint failure. That log remains intact.
After explicit lock generation, the actual nine-test suite passed once;
zero doc tests ran. The release build passed. No failing assertion or
passing-suite rerun is hidden. Exact commands, the
[frozen qualification protocol](AUXILIARY_ISOGENY.md) and
[reproduction guide](AUXILIARY_ISOGENY_REPRODUCE.md) preserve the scope.

A later comparison must charge generator search, chain construction, dense
composition, coset enumeration, target lifting and serialization/reference
setup. It must then price all relation attempts, validation, deduplication,
rank/linear algebra and scalar replay. Neither successful map construction
nor small polynomial degree establishes that full-cost gain. No comparison
with Sage/Magma runtime has been made. The current
[corrected-output plan](CORRECTED_OUTPUT_PLAN.md) and its execution access,
canonical tooling, checkpoint and live-budget requirements remain unchanged.

PR #5142 stays draft pending required artifact upload/byte replay for eleven
older raw files. The latest inspected migration 45541809165/job138961871754
logged six HeadObject HTTP 403 failures before upload/replay; no logged
outcome establishes the other five files' access state. General CI at source
2cb1f120c5 had two successes, 21 skips, two running gates and this failure;
no final CI pass is inferred. The Cloudflare receiver is still unconfigured;
713 records remain local and no sender is running.
