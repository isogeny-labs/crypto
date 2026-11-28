# Review and acceptance

The direct Fp² reference was authored before its worker inspected candidate
code. A separate mathematical reviewer independently derived the projective
transport, principal-divisor factorization, exact label counts and scope.
Both later inspected the candidate and evaluator read-only.

Before execution, review identified two API problems: public mutable frame
fields could invalidate cached maps, and zero-ordinate residuals were not
explicitly rejected despite the stated contract. The fields are now private
with copy getters; the ordinate guard and a valid M31 two-torsion rejection
control were added. The protocol amendment and both source registrations
preserve these corrections. No fixture or test outcome preceded them.

The source module with inner documentation was loaded through a path module
instead of `include!`; its original bytes were preserved. Reporting separates
random and controlled strata and marks every full-prefix case as a
constructed positive. The comparator uses the old symmetric x-Cayley result
only as a complete superset, then filters through separately evaluated
chord functions. No equivalence between the two label definitions is assumed.

Native acceptance passed on the first build: 12,288 full-set comparisons,
1,024 withheld positive recoveries, 1,536 complete supplied-set queries,
256 constructed principal-divisor/prefix checks, 52 malformed-input guards
and 16 matrix sanity checks. Deliberately incompatible labels triggered zero
square-root calls in all 12,288 rejection controls. Two executions of the
same immutable binary produced equal bytes and unchanged source manifests.

The reviewed theorem is per fixed residual pencil. Changing the prefix
changes the target, factors and transformation; no global fixed transform
or sublinear shared index follows. The actual implementation uses sorted
sets and charges inversion and setup in its stated cost model. Visit counts
are not complete arithmetic costs or timings.

This is producer evidence and separate sibling-worker review, not a signed
external validation. No Lean/kernel proof or PCC ledger claim is asserted.
