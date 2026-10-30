# Finite one-level ternary contraction unit

The complete frozen panel passed: 128 pinned matching-network cases at root
precisions 1 and 2, with exact child factorization, extension-ring interpolation
and rational Hermite recovery. This is a bounded development correctness result.
It does not certify a general algorithm, asymptotic parameter admission,
performance, ECDLP recovery, improved exponent or novelty.

## What is implemented

`onelevel.cpp` uses native C++17 and GMP integers/rationals. The fixed packet has
three row ports and three column ports with the F5 slot labels in `PROTOCOL.md`.
Its coefficient ring is `(Z/3^q Z)[U]/(U^3-U-1)`, at child precisions q=121 and
122. The residue extension supplies 27 elements; the first 25 canonical lifts
are the interpolation points for charts of degree at most 24.

At each of six active nodes, each row has one local column and two crossing
columns. The implementation assigns a separate formal mode to every ordered
crossing (source column, target row group). This produces six modes. A tile
contains one column and its tile coordinate is j=0, so the source linear form
for b>0 is exactly `0^b X=0`. For b=0 the convention `0^0=1` gives source form X.
Thus all higher-b modes can be removed identically, independent of precision.
Each row factor is affine in its two incoming modes, so its only choices are
the constant term or one of the two modes. The 27 row-choice patterns are the
complete support of the formal product; there is no heuristic monomial cutoff.

For every pattern, a nonempty group child retains its two original unary pin
tensors and has a packet tensor given by target coefficient extraction times
the source-bit factors. Its contraction explicitly sums all four assignments
of its row and column edges. All three nonempty children are evaluated this
way; the two empty groups contribute 1. The product of the group contractions
is summed over patterns. No source/target mode is merged and no pin
substitution replaces these direct leaf contractions.

The resulting values are compared with a separately written 64-assignment
deformed sum and an exact rational polynomial assembled in the global variable
T. Newton divided differences recover all 25 chart coefficients over the
extension ring, and every coefficient is compared with the exact rational
chart. The six retained jets at every active node feed exact rational Hermite
formula D.9, applied coordinatewise to the canonical representatives of their
residues. Reduction of the final three-integral rational coordinates gives the
original matching contraction. A second Hermite check uses the exact rational
jets directly and must equal the integer answer exactly.

All rational polynomial kernels are formed after cancellation of the Cauchy
factor. Three-adic powers are cancelled in their rational coefficients before
unit denominator inversion; `C'(0)=-720` is never inverted modulo a power of 3.
Ring inverses are lifted by Newton iteration from an exhaustive F27 inverse and
checked at full precision. The exact Hermite functional uses normalized
factors `E_t(T)=product_(v != t)((T-v)/(t-v))^6` over the six active nodes.

## Coverage and controls

| Check | Complete panel |
| --- | ---: |
| Root pin/precision cases | 128 |
| Root answers equal to 1 / 0 | 16 / 112 |
| Mode/child evaluations compared with two controls | 19,200 |
| Row-choice patterns evaluated | 518,400 |
| Actual two-edge child contractions | 1,555,200 |
| Explicit child assignments | 6,220,800 |
| Independent six-bit direct assignments | 1,228,800 |
| Full chart coefficient comparisons | 19,200 ring elements |
| Exact and lifted Hermite recovery cases | 128 |

The controls also check all 26 nonzero F27 residues at both precisions, all 300
pairs of interpolation points at both precisions, all 351 pairs among the full
27 residue lifts, nonunit and nonintegral rejection, both slot actions, the
wrong component-2 direction counterexample, W(0)=I and crossing locality at all
six active nodes. The kernel integrality control counts 540 stored polynomial
coordinates: 486 coefficients in the intrinsic degree-8 charts plus 54 trailing
zero coordinates produced by the untrimmed Horner chart representation.

The mandatory target-split regression uses t=1, zeta=1 and X=Y=(1,0,0) modulo
27. The correct factorization and independent direct sum both give 9; the
deliberately wrong factorization that imposes one source exponent separately
at both targets gives 0. This is a detected structural failure, not a passing
variant.

## Artifacts and replay

- `PROTOCOL.md` and `PROTOCOL.sha256`: frozen before implementation.
- `onelevel.cpp`, `replay.sh`: final native source and exact build/run commands.
- `run-002/`, `run-003/`: final-source independent rebuilds and complete outputs.
  Their controls, all 128 case rows, summary and run log must be byte-identical.
- `run-001/`, `onelevel.run-001.cpp`, `onelevel.run-001`: first successful run
  and its source/binary, retained before a reporting-only control-row split.
- `build-001.log`: retained failed strict build; an unused comparison helper
  triggered `-Werror=unused-function`. `build-002.log` records its correction.
  Rational constructor canonicalization was also made explicit before the first
  execution after reviewing GMP's constructor semantics. No case was dropped.
- `REPLAY-CHECK.txt`, `TOOLCHAIN.txt`, `MANIFEST.sha256`: final evidence checks,
  native compiler information and artifact hashes.

Run `./replay.sh new-output-directory`; then compare its `controls.tsv`,
`cases.tsv`, `summary.tsv` and `run.log` with the saved final outputs. The
program fails on the first arithmetic mismatch and flushes completed case rows.
No timers or timing metrics are used. Research arithmetic is native C++/GMP;
no Python arithmetic or cryptographic library is used.

The fixed source precision, instance and checks are those of `PROTOCOL.md`.
The tiny M=3 packet is forced to exercise identities. It is not an admissible
active call under the source's large-size parameter regime. There is no deeper
recursion, randomized tensor family, arbitrary nilpotent coefficient extension,
general packet packer, or complete contraction solver in this unit. This work
adapts the cited Appendix D.4 mechanism; it supplies no novelty certificate.
