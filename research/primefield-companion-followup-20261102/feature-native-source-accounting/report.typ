#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
= Public source accounting and preparation reuse
T-1831 · 2026-11-07 · Prime-field source construction

Public source construction now computes the dyadic exponent once and lazily caches immutable public base strides. Every window on that source reuses the preparation, including concurrent callers. Raw/sign-only and zero-length-window usage avoid that preparation. This is an implementation change with exact saved checks and derived operation accounting.

#image("flow.svg", width: 100%)

*Accounting convention.* Every field product, square and constant product counts as one M; field Euclid as one I. Integer arithmetic, reductions, comparisons, additions, allocation and copying are additional costs. Prime p, odd prime r, coprimality and the retained word-size domain apply. See DERIVATION.md for exact accepted-path equations and branch costs.

*Complete symbolic composition.* Raw-point evaluation comprises grid geometry, a compiled division schedule, multiplicative-class arithmetic and a polynomial trace term. Source construction adds curve validation, schedule compilation, integer modular inversion and calibration. Windows charge initialization, actual addition branches, joint conversion, and residual arithmetic for each finite shift. Polynomial degree/remainder traces remain unevaluated numerically; no dominant phase is inferred.

*Derived saving and memory.* For J nonempty windows, preparation changes from J passes to one, removing J−1 field inversions and the corresponding public grid products. First-window arithmetic is unchanged. Retained labels, projective/affine strides and curve/base context cost O(g) storage. Window snapshots, mappings, copied grids and polynomial arrays remain additional. The exponent inversion is modulo the dyadic order, not a prime-field inversion.

#table(columns: (2fr, 1fr, 2fr), inset: 4pt,
 [Saved check], [Count], [Outcome],
 [Preparation controls], [8 sources], [Lazy zero window; repeated/concurrent reuse],
 [Source points / windows], [1,108 / 24], [Direct products, covariance, signs],
 [Division equalities / plans], [160,660 / 4,100], [Inherited mode and storage checks],
 [Additional recoveries], [617], [5,929 calls; zero unresolved],
 [Inherited public comparisons], [63], [511 calls],
 [Normal library build], [7 pins], [Audit instrumentation disabled])

*Evidence and scope.* Saved validation-01, recovery-01 and normal-build receipts pass and their input/output hashes were checked after migration. Original 4,563-case cohort unchanged. No timing/cloud run or full Cargo-suite rerun. PCC-20261107-7f1bb8 is unsigned producer evidence, not independent verification. The source still forms public index squares for each scaled term.

*Sources and next step.* The prior split-lifetime/joint-window proofs and primary EFD Jacobian formulas informed the accounting; EFD uses different operation weights. KB unavailable. Next instrument the polynomial call trace under this convention to locate remaining compression opportunities. No other canonical graph or historical report is changed.
