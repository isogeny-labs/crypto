# Retained prime33555419 crossing: local execution protocol

Original objective remains an evaluated ordinary isogeny crossing a minimum
80-bit prime End-conductor ratio, with matched endpoint ECDLP measurements.
This separately named case is a26-bit intermediate milestone, preserving its
original53-bit prime subgroup5629830654216971. A field bit count or Frobenius
conductor alone does not satisfy either crossing criterion.

Native evaluator: separately versioned `native/complete_twisted_pipeline.cpp`,
compiled against the same native FLINT/PARI/GMP libraries and existing optimized
object. Exact argv/input/binary identities are frozen before execution. The
original evaluator is unchanged. Bounded qualification:18 direct PARI maps,
470 point-input evaluations, three default byte matches, eight rejections;
PCC-20261119-3b9f6e, unsigned/not_covered. [pcc:unverified]

The source auxiliary curve T has a rational prime-order kernel. The actual
curve E is its nonsquare twist d=2. Its normalized map is
`(x,y) -> (d*f(x/d), y*f'(x/d))`; auxiliary kernels are not asserted to be
points on E. The result equation is the d²/d³-scaled quotient of T. At infinity
the map returns infinity; kernel abscissa collisions return infinity. Small
direct kernel-polynomial references exercise those cases.

`prepare-case0.gp` checks exact endpoint group counts, primality, original
generator order, twist coefficients, ordinary trace/discriminant, class number
one and distinct endpoint j values. It freezes32 known scalars from PARI seed
20261119 plus the originalG. The separate auxiliary probe has prime order ell
and nontrivial Weil pairing with K, so it lies outside the first kernel. It is
used solely for constructing the dual; it never replaces the benchmark subgroup.

After preparation, freeze all case files, scripts, native source/binary hashes
and actual host metadata. Run main twisted map and auxiliary probe separately
under the shared lock, at most120seconds each,12GiB address-space limit,
max_window4096,block4096,projective_batch64,prepared norm. Stop on timeout/OOM,
nonzero exit, stderr (except the explicitly saved resource report), incomplete
coverage, equation mismatch or point validation failure. Preserve failed runs.
No build/test/audit or other benchmark overlaps measured execution. Local L0
only; no reserved-core/PMU admission or performance ranking is asserted.

Independently verify returned quotient against retained equation, all source/
image memberships, nonzero transportedG, subgroup orders and32 scalar identities
using PARI. Then freeze a dual-construction input using the mapped independent
auxiliary probe. Its normalized double quotient must equal the ell⁴/ell⁶-scaled
original equation; composed point images must equal ell²/ell³-scaled [ell]P.
These are finite implementation checks; the separable Vélu/kernel theorem and
CM classification argument carry the mathematical interpretation.

Only after map checks prepare matched native ECDLP experiments with transported
generator and targets, explicitly frozen budgets/arms/repeats and preserved
failure outcomes. Keep setup, construction, transport, solver and verification
costs separate. No measured ECDLP result is asserted by this protocol.

Sources checked: retained `conductor_gap_cm_orbits_20261015/prepared/`
kernel/catalog and `native/CONSTRUCTOR.md`; actual complete evaluator and native
PARI APIs; previously reviewed primary ordinary-order/typed-transfer references.
KB unavailable (no loaded tools, wrapper executable absent). No novelty claim.
