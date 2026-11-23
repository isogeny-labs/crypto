# Prime-field PDP arity and solver-model correction

**Status:** corrected analysis, hardened 2026-11-01; no scientific run.

**Decision provenance:** `DEC-20261031-b51282` records the immutable arity
correction. `DEC-20261101-26f1fa` adds the setup, population, representation,
proof and encoding gates below. `DEC-20261031-cd6460` remains an unrelated
checker-defect decision.

## Arity and complete-cost bridge

Every accepted theorem and completed measurement in
[`prime_factor_base_degree_20261031`](../prime_factor_base_degree_20261031/README.md)
is a two-summand result. Let

\[
  s=\frac{\log_2 p}{m},\qquad |V|\asymp 2^s.
\]

If reusable target-independent setup costs `S(s)=2^(a s)` and one natural
point-decomposition query costs `Q(s)=2^(e s)`, collecting `Theta(2^s)`
relations costs

\[
  S(s)+2^sQ(s)=2^{a s}+2^{(e+1)s}
\]

up to polynomial factors. Beating rho's `p^(1/2)` exponent therefore requires
both

\[
  a<m/2\quad\text{and}\quad e<m/2-1.
\]

At `m=2`, the second condition is `e<0`, impossible for a nontrivial query;
the exact pair-enumeration reference is optimal at that arity up to polynomial
factors. The first discriminating fixed arity is `m=3`, where the necessary
gates are `a<1.5` and `e<0.5`.

## Solver and model semantics

The decision candidate is hybrid CDCL(T): CaDiCaL owns the Boolean selector
skeleton and a source-bound `F_p` propagator owns field arithmetic, inversion
guards, chained group-law constraints, exceptional charts and signed lift
semantics. Every propagation/conflict needs an independently replayed Boolean
explanation; UNSAT needs both Boolean and admitted theory proof steps checked.

Pure CNF is an encoding-cost control. The quotient relations `z^2-z` avoid
algebraic carries, but the selected schoolbook multiplier has
`Theta((log p)^2)` gates by construction. That is not a lower bound for all
Boolean multiplication. The exact gate basis and modular-reduction circuit
must be frozen and counted.

The semantic object is a rational **point**, not an abscissa alone. Chained
`S3` relations are sign-blind. Each selected abscissa is therefore bound to a
rational lift sign, and every accepted model is independently replayed as a
signed group equation.

At inspected cvc5 source head
`02f77af402b4be82f27e5cc00b86b265a90dea3d`, finite-field theory returns no
proof checker. QF_FF is consequently a SAT/model diagnostic, not an
UNSAT-eligible slope candidate, unless a later frozen source or external
certificate supplies independently checked finite-field UNSAT evidence.

## Retained algebra

For nonexceptional displacement `t`, the lift equation

\[
  (2t\lambda-s_0)^2=f(x)
\]

is quadratic in `lambda` with discriminant

\[
  (-4ts_0)^2-4(4t^2)(s_0^2-f(x))=16t^2f(x).
\]

Thus for `t!=0`, a rational lambda exists exactly when `x` has a rational
curve lift. The `t=0` case remains an explicit exceptional branch.

For three disjoint abscissa windows, odd prime whole-group order and a rational
non-2-torsion target, Frobenius rules out a nonrational subset of one, two or
three summands: one cannot sum rationally; two would be inverses and repeat an
abscissa; three would make the target 2-torsion. Hence guarded arity-three
models have rational lifts without an extra per-summand circuit. At arity four
or higher, explicit rationality circuits remain required and charged.

Field/index membership is the compact predicate `0 <= word < domain`, emitted
as an unsigned comparator or native theory-domain check. One clause per
forbidden word is permitted only below a frozen enumeration cap; it is not the
default representation.

## Initial ladder and continuation admission

The initial development target is fixed `m=3`, every integer `s=6..14`, with
three disjoint selector windows and chained `S3` through the quadratic lift;
no expanded `S4` polynomial. Exact two-window enumeration is the model-set
oracle where it completes and the `e=2` cost control.

The selected Sinz sequential exactly-one encoding has, at `s=27`, exactly
402,653,184 selector variables, 402,653,181 auxiliaries, 805,306,365 Boolean
variables before theory auxiliaries and 1,207,959,543 exactly-one clauses.
Therefore the one-hot `s=15..27` continuation is not admitted. A prospective
amendment must either pass a measured resource budget for the exact
representation or establish small-cell model-set equivalence and fresh cost
preflights for a compact/lazy selector.

## Population, censoring and evidence boundary

The primary `e` slope uses natural instances only. Planted and exact-UNSAT
instances are separately reported correctness/control strata; they are never
pooled into the natural slope. The setup exponent `a` is fit from the reusable
target-independent stage. The regression coordinate is actual recorded
`log2(p)/3`, not merely the nominal rung.

Advance requires every preregistered natural row at every rung to complete with
a verified exact status, all controls to pass, and corrected upper intervals
strictly below both `a=1.5` and `e=0.5`. Any natural timeout, OOM, unknown,
unproved UNSAT or incomplete rung makes the decision inconclusive. No
completed-row subset or imputation can satisfy the gate.

The native development oracle merged in
[`cryptanalysis#1347`](https://github.com/isogeny-labs/cryptanalysis/pull/1347)
at `3c769002f0f68d63a5055f6f3e24a9500393180e`. It validates a disclosed
small-field fixture, the lift identity, signed semantics, compact range check,
Sinz encoding and exact representation counts. It supplies no held-out rows,
solver exponent or production `F_p` trail theory.
