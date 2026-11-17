# Frozen GF(16) cubic Hess closure audit

Date: 2026-10-27. The audit follows the literature check in
`../ghs_hess_c_parameter_audit_20261026/EXACT_CUBIC_FAMILY.md` and
Menezes--Teske (2004), Sections 7 and 9.3.2. The previous GF(16)
quartic isogeny and catalog receipts are inputs, not outcomes to
reinterpret after the screen.

## Cohort and tests

Enumerate every normalized ordinary elliptic curve
`E_(a,b): y^2+xy=x^3+a*x^2+b` over Conway `K=GF(4096)`, with
`a in {0,32}` and `b=1,...,4095`. Use Sage's elliptic cardinality for
all 8,190 models and compare their complete order groups with the
independently computed full-cohort certificate. Record every exact order.

For `k=GF(16)`, set `U=ker Tr_(K/k)` and
`P={gamma*beta: gamma,beta in U*}`. For each source, let
`delta=sqrt(b)`. The complete Hess genus-three criterion is:

- absolute trace `Tr_(K/F2)(a)=0`: `delta in P`;
- absolute trace `Tr_(K/F2)(a)=1`: `delta in U`.

The second case follows Hess's regularity condition: both trace-zero
factors are inadmissible when absolute trace of `a` is one. In cubic
degree, the only genus-three factor-order pairs are `(1,2)` and
`(2,2)`. The `(1,2)` product set is `U*`, and the `(2,2)` set is `P`.
The former lies in the latter. This criterion is within Hess's
specified construction, not a test for every possible genus-three
cover or isogeny.

Group curves by exact `#E(K)`. An isogeny class belongs to the Hess
genus-three closure iff any normalized curve at that order passes the
criterion. The order census is complete for ordinary curves, so the
negative closure label covers arbitrary `K`-isogenies, not a bounded
walk. Join the order groups to the frozen LMFDB class-status snapshot
and its separate Sage catalog audit; label unknown Jacobian status
unknown. Recheck the preceding 300-source witness cohort, including
the order-4126 class, as a focused control.

## Outputs and claim gate

Retain a machine-readable certificate with exact b lists per order,
hashes of prior snapshots, full count partitions, and explicit
Hess-eligible and Hess-ineligible counts. Replay representative source
cardinalities and the coefficient criterion independently. The result
may identify a structural residual transfer family. It is a measured
vulnerability only after an explicit nonzero subgroup map, a complete
target-dependent solve, and a same-source attack cost below rho.
