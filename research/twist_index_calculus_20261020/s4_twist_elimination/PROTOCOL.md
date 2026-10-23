# Exact S4 structure and quartic-twist elimination

Frozen 23 October 2026 before implementing this structural probe.

The original objective remains a verified improvement in elliptic-curve
index calculus using twist symmetry. The previous native k=1 collector
peels a signed line-2 factor and solves four degree-four equations in
three elementary symmetric line-0 variables. Its same-target arithmetic
cost remains well above twist24 rho. Exact-orbit caching identified at
most 790 redundant solves in 142,248 full-stream peels. This round tests
a different mechanism: sparse algebraic structure *inside each solve*.

## Requirements and frozen inputs

| Requirement | Evidence to obtain |
| --- | --- |
| Derive the exact j=0 S4 polynomial | Native integer polynomial resultant and symmetric reduction, zero exact remainder; explicit coefficient record; identity certificate and feasible Lean proof |
| Identify which structure is supplied by the twist | Substitute `B=b0*t^3`, `t^4=d` and compare four descended components with the existing interpolated S4 on the same inputs |
| Test a complete elimination path if the structure permits one | Complete symmetric roots and signed relations on the frozen panel; explicit treatment of every zero denominator and exceptional locus |
| Compare the actual relation finder | Preserve the old native source/binary, identical factor bases, residual order, roots and answer checks; count every phase; no oracle hits added to candidate output |
| Preserve scope and limits | All failures, exceptional systems and misses retained; whole-method pricing and external-solver comparison remain separate gates |

Worktree baseline: `ab7bbcaed1` on `codex/twist-ic-20261020`.
The exact curve models and generator/target data are those in
`../quartic_twist_mixed_v1.ndjson` and the native p=37/73/97 scaling
records. The source is `examples/quartic_twist_tagged_s5.rs`, especially
`SymS4` and `collect_mixed_pattern`; no historical file is overwritten.
All targets are public synthetic `[271]G` fixtures.

## Structural derivation, before any performance comparison

Implement sparse polynomials over the integers in native Rust. Expand
the quadratic resultant

`(a2*b0-a0*b2)^2-(a2*b1-a1*b2)*(a1*b0-a0*b1)`

for `S3(x1,x2,Z)` and `S3(x3,X,Z)` on `y^2=x^3+B`, then reduce the
first three variables into `e1,e2,e3` by lexicographic symmetric
reduction. Require the entire coefficient remainder to be exactly zero,
not merely zero on selected finite-field points. Emit the resulting
integer coefficients and degree profile. Check the independent direct
resultant and the compact formula with the repository identity
certificate, including checker-derived extra points and a deliberately
corrupted negative control. Attempt the central identity in pinned Lean
using the same resultant and elementary-symmetric definitions.

After deriving the exact formula, inspect its four components on
`Fp[t]/(t^4-d)` with `B=b0*t^3` and the line-0 variables. The structural
pilot uses all finite signed line-2 peels of the first eight frozen p=37
seed-zero residuals, all full-base processed prefixes of the six p=73/97
scaling runs, and specifically includes the p=73 seed-one failed peel.
Match the component polynomial coefficients exactly to the old
interpolated implementation. Retain degenerate and zero components.
Use native field/group arithmetic throughout. The reference oracle and
known scalar are confined to post-run correctness checks.

An explicit solver suggested by the structure receives its own frozen
complete comparison protocol before running performance measurements.
Algebraic structure or a low-degree formula alone does not establish a
relation-yield, rank, total-cost, asymptotic or novelty claim. Ordinary
symmetrization and invariant coordinates are prior work; novelty needs
a concrete additional mechanism and literature comparison.

## Execution and reporting

Use 8 GiB address-space and 1,200 s process watchdog bounds for the
structural probe and any independent replay; preserve exit codes and
stderr. Counted arithmetic is primary. Any native-time comparison must
use the repository isolation tool, fixed thread counts, A/A and at least
five interleaved rounds on the same inputs. msolve/Magma comparisons,
complete native memory/CPU accounting, matched strongest rho and the
0/0.1/1 GAE band remain necessary before a priced whole-method claim.
Keep source-linked reports, figures and inspected PDFs for all outcomes.

## Structural supplement after the first exact derivation

The first standard-library native job produced 49 terms in the three
elementary variables and a zero integer coefficient remainder. Before
running a further derivation, also freeze the full four-abscissa symmetric
reduction: elementary variables `s1,s2,s3,s4` of `x1,x2,x3,X`, with `B`
left independent. Require exact reconstruction over the integers and a
faithful commutative-ring Lean theorem. This checks for a smaller formula
whose specializations could expose an elimination pattern. It makes no
novelty or relation-finder performance claim by itself.
