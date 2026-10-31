# N19 signed-label fallback: frozen protocol

Frozen before the first timed cohort run. This is a distinct arm of the
uncached N11 paired collector on the same 1,260 public synthetic source
targets, seven bit slices, seeds, split rule, caps, and negation-rho control.
It does not replace or alter the frozen N11 rows.

The state is `(K(D), K(D+G), a, b)` with the invariant `D=aG+bQ` in the
prime-order Jacobian subgroup. The chosen branch is the original partition
of `K(D)`. Its successor labels are `a'=2a+choice (mod r)` and `b'=2b
(mod r)`. The three original uncached, torsion-repaired xADD attempts are
charged on every step. If either of the two outputs needed by the selected
branch is zero or undefined, reconstruct `D'=a'G+b'Q` by two native
Jacobian scalar multiplications and one addition; form `D'+G` by another
addition; project both by the published C1 image of the classical
degree-0/1/2 Kummer chart. Every executed scalar multiplication, addition,
projection, repair, and `Jacobian::valid` check is counted. A zero vector after direct projection is a fatal
`fallback_projection_fail`, retained in the raw row. The unused third xADD
output may be zero without invoking fallback. A separate validation mode
checks each ordinary next-state pair by signed reconstruction and exact
projective equality, charging its native and chart arithmetic. The finite
gate enables it with `KUMMER_VALIDATE_TRANSITIONS=1`; the full 1,260-target
timed cohort uses the default disabled mode and records this flag per row.
This preserves the original quotient-walk cost boundary while the finite
gate tests the signed invariant. These diagnostic checks are excluded from
primary cost because they are not executed there. Independent final scalar
replay on the original source curve is required for every primary row. The
fallback's all-state chart is also checked in the finite gate against the
independently implemented C2 oracle, including a valid `a0=b0=0` divisor.

Initial state and restarts retain the baseline random `(a,b)` and signed
reconstruction. Collector relation admission, graph operations, Recover,
source scalar verification, and target cap remain as in the uncached arm.
The source transfer, model/root/theta/torsion setup, anchor, all online
work, and final verification are included in full-cold cost per target.
The 21 model setups are charged 60 times each. Rho runs in the same
process with the same source addition unit and all verification charged.
Pricing uses all 36 cells of the original grid: `S/M={0,0.5,1}`,
`I/M={1,10,100,1000}`, native logical charges `{0,0.1,1}` GAE. The
baseline candidate, new arm, and rho are joined by workload ID. All wins,
losses, failed rows, and null ratios are retained. No mean or stage-only
ratio substitutes for per-target or full-cohort ratios.

Gate sequence: (1) source-level label invariant; (2) direct projection
cross-check on degree 0, degree 1 and degree 2 classes across all frozen
models; (3) force a zero xADD output and check the charged signed fallback;
(4) finite target transition and scalar replay; (5) only then run the full
1,260 paired cohort. If any gate fails, retain its evidence and do not
interpret a partial cohort as a full result.
