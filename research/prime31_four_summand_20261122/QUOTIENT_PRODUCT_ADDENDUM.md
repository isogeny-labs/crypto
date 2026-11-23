# Compact quotient-product follow-up

Frozen before the follow-up candidate calls, 2026-11-22 Pacific. Tracking PR:
#4949, opened after #4942 merged. This addendum
retains every earlier protocol, input and failure. It is a second bounded Rust
correctness qualification, not a benchmark or an increased-resource rerun.

## Observed obstruction and exact change

The expanded B8 remainder gates each returned `Inconclusive`: no algebraic
univariate was available, and the trace sink refused enumeration. The fixed
positive had one independently enumerated signed row; the generator control
had none. Each input had four polynomials, 319 terms and maximum degree 10.
The recorded native matrix maxima were 208 rows and 3,752 columns. These
observations are retained in `evidence/b8-positive-01` and `evidence/b8-empty-01`.

For base sizes 5 through 8, retain the same RR5 monic quartic U in its three
coefficient parameters. Introduce a monic polynomial W of degree B-4 with B-4
unknown coefficients. Equate all B lower coefficients of U*W-h to zero. The
leading coefficient is identically zero because U, W and h are monic. At B8
this is eight equations of degree at most three in seven unknowns, rather than
four expanded degree-ten equations in three unknowns. Use graded reverse
lexicographic order for this presentation. B4 retains its existing equations
and lexicographic traversal.

Over the coefficient field, monic divisibility U|h is equivalent to the
existence of this W, which is unique. This algebraic rewriting does not bound
F4 cost or prove that a useful eliminant will be found. All original chart
preconditions, finite-field root extraction, terminal equation checks and
signed group replay remain mandatory. Only the first three parameters are
needed by the signed lift; auxiliary assignments cannot introduce a second
cofactor for a fixed U.

The lower input degree does not transfer the B4 degree-five Macaulay argument.
Homogenized product generators can have auxiliary directions at infinity, and
a complete Grevlex basis need not display a pure univariate. Either practical
obstruction can leave this root-only traversal incomplete without contradicting
the affine equivalence.

## Fixed controls and unchanged acceptance

Rebuild from a new frozen source inventory and execute the same independent
positive and generator-empty B8 gates. Do not alter their oracle or assertions.
The candidate still receives no witness, scalar, tuple list or base logarithm.
The test-only oracle retains exactly 1,120 signed four-subset evaluations.

Keep degree cap 10, 30 seconds per candidate call, 20,000 symbolic terms,
4,096 assignments, one Rayon/test worker, a 180-second outer timeout and
2,097,152 KiB whole-process address-space limit covering F4 allocation. Retain
each case in its own process so a failed case cannot hide the other one. A
timeout, allocation failure, degree limit, missing eliminant or partial traversal
fails the complete gate. No limit or acceptance criterion has been relaxed.

Rerun the 16 unchanged B4/adapter checks and the new scalar long-division
construction control on the revised source. The already
qualified public example and identity entry point are independently bound to
their own receipts. No full-library pass or requested B677 result follows from
any successful compact B8 control.
