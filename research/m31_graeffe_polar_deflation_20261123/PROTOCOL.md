# M31 Graeffe polar deflation: fixed development protocol

Frozen design, 2026-11-23 Pacific / 2026-11-24 UTC. This is a native
correctness component and research proposal, not a solver benchmark.
Standing: unsigned producer development evidence. [pcc:unverified]

## Question and acceptance criteria

Can six total second partial derivatives of the centered third elementary
coefficient remove the local multiplicity of the fully collapsed RR5
Graeffe presentation, while retaining every solution in its original
regular chart?

The written argument is in `MATHEMATICS.md`. Its finite-field check must:

1. Retain the original M31 curve, depth-10 Dickson fiber, and all sixteen
   fixed cases below. Reject an inadmissible case; do not replace it.
2. Reconstruct each RR5 quartic from its signed divisor, check all original
   point equations and exact norm coefficients, and retain the target.
3. Propagate all total Taylor coefficients through degree three in the three
   original RR5 parameters, through ten Graeffe stages. Compare against a
   separate root-jet construction and scalar Dickson composition.
4. Check that every original centered residual and all six Hessian entries
   of the third centered residual vanish, and that the six-by-three
   Jacobian of these entries has rank three over M31.
5. Exercise the cheaper first-derivative-of-e2 alternative on a synthetic
   tangent hyperplane where it is singular, while the e3 Hessian succeeds.
6. Retain a control showing that differentiation with respect to independent
   terminal graph coordinates is not total differentiation in the original
   parameters. Reject a meaningful wrong centered-coefficient formula.

The field checks concern four leaves in the pure Dickson B509 component.
They do not apply the construction to the mixed Y7 sectors of B677.

The observations qualify only the fixed development controls. They do not
measure relation discovery, F4 cost, root extraction, or speed.

## Exact field, curve, and fixture

- Coordinate prime: p = 2147483647 = 2^31 - 1.
- Curve: y^2 = x^3 + 847514846 x + 2087444724.
- Recorded prime group order: 2147446831; cofactor one.
- Recorded generator: (625037715, 2100627251).
- Reuse the existing native `PRIME31`, `Arithmetic`, and `Base::build` from
  `research/prime31_ic_20261115/native/lib.rs` through the unchanged
  dependency metadata adapter. Fixture acquisition is setup, not discovery.
- `Base::build(PRIME31, 10, ...)` must return 1024 distinct raw selector
  abscissae and 509 curve-liftable canonical points. Retain the actual
  terminal tau and confirm tau is neither 2 nor -2.

For k = 0,...,15, use the four canonical base entries with indices
`[4*k, 4*k+1, 4*k+2, 4*k+3]`. Negate ordinate j exactly when bit j of k
is one. The target is their exact elliptic sum. Require a finite target
outside the signed base, distinct original abscissae, nonzero ordinates,
nonzero selector derivatives, and the regular RR5 normalization.

These are deterministic known-answer controls. They use no random stream,
target search, or favorable-case replacement. The synthetic exceptional
normal is n=(1,5,c,-6-c), with c=(-6+sqrt(-88))/2 and the smaller canonical
square root. Require c^2+6c+31=0, all normal entries nonzero, sum(n)=0 and
sum(n_i^2)=0. Failure is retained; no alternative normal is selected. This is
a separately labeled algebraic control, not an elliptic witness or a sample
from the M31 relation population. The controls n=(1,1,1,1), n=(1,2,3,4),
and the deliberately out-of-scope n=(1,0,0,1) are also fixed.

## Execution and preservation

Use Rust/Cargo 1.100.0, an offline dependency cache, one build job, and one
test thread. Freeze a source copy and source SHA-256 manifest before native
execution. Build and execute that copy, with separate build, pre-call, and
post-call receipts. Preserve exact commands, stdout, stderr, exits and
source hashes. One build and one check attempt have separate 300-second
outer limits. A source defect may be corrected in the working template;
retain the failed snapshot and use a newly named snapshot for a new attempt.
Do not overwrite a pre-call record or reinterpret a failed attempt as absent.

No F4 calls, relation extraction, randomized comparison, paid compute or
scientific cloud execution is part of this protocol. The native checker
must not receive unknown target logs. Exact operation timings are not a
comparison result. Root-crate qualification is separate from this standalone
component.

## Next solver gate, not executed here

A later admission must freeze matched native discovery implementations for
the full remainder, collapsed Graeffe, six-Hessian Graeffe, and distinct-image
cutoff presentations. Use identical curves, targets, seeds, machine limits,
and total budgets; report the cutoff's lost support explicitly. Charge
automatic differentiation, all lifts, construction, failed solves, root
extraction, signed replay, useful row rank and retained memory. Keep all
original limits and use independent holdouts. Smaller local multiplicity or
a full-rank Jacobian alone is not a performance result.

## Infrastructure and prior-art receipt

No research-KB tool was exposed. After populating the sparse worktree,
`scripts/research-kb status` exited 127 with `no executable found`.
Conductor, AWS CLI and kubectl were absent from PATH. The fallback was
bounded source review in crypto, crypto-autoresearcher and cryptanalysis,
plus primary Dickson and deflation papers. Sources and rediscovered leads
are retained in `SOURCE_REVIEW.md` and the report. No absence-of-prior-art
claim follows from this search.
