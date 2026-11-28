# Count rho's executed native tower arithmetic

The existing rho walk uses its own deferred Barrett tower arithmetic, while
measure_fq3_prims calibrates the IC harness. In particular, their inversion
algorithms differ. This patch adds a separately named native calibration
and opt-in comparator price. It preserves the legacy default, TUNED credits,
walk parameters, RNG consumption and original recorded experiments.
[pcc:unverified]

Arith is const-generic. COUNT=false is the production instantiation and
removes the new hooks; COUNT=true calibrates the same native source. Each
source prime-field product and sum, including deferred reductions, emits
the corresponding existing modular hook. Subtraction/negation emits one
field-add hook; inv_mod already emits one inverse and is not charged again.
Barrett reduction internals do not emit additional field multiplications:
the unit prices model prime-field algebra, not physical circuits or native
instruction counts. Current calibration is bounded to the existing small
public-prime domain. [pcc:unverified]

| Source algebra | Expected source-derived primitive counts |
| --- | --- |
| Quadratic tower product | 5 M, 2 A |
| Cubic-over-quadratic product | 11 quadratic products plus18accumulations and4final additions:55 M,44 A |
| Cubic inverse | 15 quadratic products plus a quadratic inverse and10adjugate/norm additions:80 M,42 A,1 I |

These are prospective source criteria until native tests and the frozen
reference validation complete. The hand control checks
(2+3t)(5+7t)=52+29t and (2+3t)^−1=72+94t over p101,t²=2, with exact
5 M/2 A and5 M/2 A/1 I respectively. Basis/random/zero operands over
p101,251,1009 compare counted and uninstrumented native values with the
independent harness algorithm, including square/inverse counts. Existing
strong-walk tests and the public worker check unchanged actual scalars,
semantic operation traces and per-phase inverse accounting.
[pcc:unverified]

The comparator's opt-in native mode retains its calibration vectors,
marks exactly this resolved coverage item, and leaves total/comparative
flags false. The reconstructed rho tally remains the sole source for its
explicit walk costs; only separately measured outer elliptic replay is
added. Whole process hooks are never merged into that tally. Nested public
point answer fields are now rejected as well. [pcc:unverified]

The standalone Rust qualifier reconstructs explicit setup/walk/collision/
verification operation vectors from original high-level counters, checks
native calibration and phase repricing, and requires identical residual
outer-replay vectors between harness/native modes. It binds actual scalar,
public fixture, held-out answer, source/argv/flow/status and exact semantic
walk equality. Native hand/tamper controls test multiplication/inversion,
RNG words, canonicalization, collisions and double-inverse rejection.
No independent signed verdict follows. [pcc:unverified]

Sources checked (KB): retained status/exact/mismatch rho receipts show the
wrapper unavailable. Read native Arith, both calibration paths, rho_tally,
the existing controls and docs/units/BITOPS.md. Amendment27 and
rho-native-plan.json specify the prospective fresh p101seed317 reference
validation, conditional on native source checks and publication. No new
IC comparison, algorithm speed result or parity claim. [pcc:unverified]

At the final source freeze,11native library checks,1public worker check,
16standalone qualifier checks, production builds and strict Clippy pass.
Two root-source hashes stayed fixed through native compilation. The
native capture service's authorization/single-dispatch test and warnings-
denied compilation pass. The user selected Runpod before measurement:
the CPU-only runner, immutable Rust image, credential-free bundle and
relative custody hashes are specified before the sole scientific call.
No32measurement result is asserted at source freeze. [pcc:unverified]
