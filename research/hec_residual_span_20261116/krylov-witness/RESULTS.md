# Callback-only target witness finder qualifies

Pushed freeze5ff034f1672, single invocation exits0. Frozen hashes verified.
All18retained Schur panels produce an exactly verified target witness on
first prescribed attempt. For each, S^T w=e_k,w^T rhs agrees with original
Sparse/SchurSparse/nativeFast, and the scalar reconstructs the group target.
All previous252basiscolumn/54adjoint/RHS/original-equation checks pass.

Five fixed synthetic tests:zerooperator and target outside rowspace return
unknown after3attempts(no witness returned);solvable rank-deficient,F5isotropic-
kernel and fullcolumn-rank rectangular cases return verified witnesses on
first attempt. None is promoted from recurrence alone. No bounded-success
probability or infeasibility certificate is inferred from unknown.

Application counts per actual panel range7to205each for S and S^T, including
verification. Largest68seed case uses degree68recurrence and205ofeach.
Raw attempt/degree/zero-constant/rejected-witness counters in CONTROL.stderr;
all18actual-panel rejected/zero-constant counts are0. These are callbacks,
not counted field instructions, CPU, wall or a speedup measurement.

Finder takes only field,dimensions,callback pair and deterministic seed.
It constructs no dense matrix or Krylov-vector table and keeps O(n+r)
field words beyond graph/output. Diagnostic driver DOES materialize dense
references outside finder, as declared; no whole-process memory claim.
This implements classical verified Wiedemann with bounded preconditioning,
not a mathematical breakthrough or an attack/genus3/4qualification.

Next obligation: compare charged solver work with incremental sparse
elimination at matched completing prefixes. Applying every redundant closure
for3nsteps may be expensive, so smaller dimension/storage cannot be assumed
faster. A scaled producer and unknown-action component handling still need
integration; retained-support/known-field inputs are free oracle data here.
Full genus2+10xobjective remains unresolved, with failures/cost boundaries
from earlier studies unchanged.
