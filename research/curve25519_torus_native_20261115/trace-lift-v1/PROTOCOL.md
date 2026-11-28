# Exact trace projection: pre-code development protocol

The original request remains a scaling advantage for Curve25519 index
calculus. This component tests an exact elimination interface for the next
direct solver; it does not substitute a representation count for solving cost.
The current public-only reference explicitly enumerates signed triples.

For an odd prime field, write z=x+b*sqrt(2), norm(z)=1, and omega=c+d*sqrt(2)
with norm(omega)=1 and d nonzero. The classical Chebyshev identities give
z^q=T_q(x)+b*U_(q-1)(x)*sqrt(2) and
T_q(x)^2-(x^2-1)*U_(q-1)(x)^2=1. Hypothesis: the single equation T_q(x)=c
has the unique original-coordinate lift
b=(x^2-1)*U_(q-1)(x)/(2*d). No independent square-root branch is selected.
For this implementation require q>0 and q nonzero in the field, and compute
U from T'_q/q by automatic differentiation of the binary Chebyshev ladder.

## Sources and novelty boundary

KB unavailable: no loaded MCP, sparse wrapper absent, backend executable
absent; the current-main tracked wrapper was read and cannot supply a
missing backend. Direct sources read: historical-native-v1.rs and current
torus-s4 Chebyshev implementation; frozen root-reference-v1/NEXT.md;
consumer ROUTE_CRITERIA_V3.md at merged commit2dee2662; DLMF18.5.E1/E2 and
18.9.T1; pinned Mathlib RingTheory/Polynomial/Chebyshev.lean at
cfc55fc58714a59dc06e6e79edc6d1d95140d3c2, including natDegree_T,
T_derivative_eq_U and recurrence declarations. The repository already uses
Chebyshev membership. No novelty is claimed for that coordinate, its
recurrences, Pell identity, differentiation or degree theorem.

## Frozen fixed controls

New isolated Rust crate tools/torus-trace-lift; no edits to the frozen
membership/aggregation/reference sources. Original equations are checked by
their existing Fp2 arithmetic, with every accepted original p101/q3 member
also replayed through Circuit::compile and witness. Fixed controls only:

1. Existing p101/h(17,12), q3, j1..8. Inspect every x,b in Fp101 and compare
   norm-one plus z^q=omega with the proposed unique lift. Retain complete
   equality trace and lifted members, including all original24 members.
2. Same p101/h, q1/5/17, j1,2: all field pairs. These are algebraic core
   controls; no assertion that each altered q defines the original base.
3. Existing p59/h(54,37), q3, j1,2: all field pairs. Replay against original
   core equations; no use of the p=5 mod8 curve square-root routine here.
4. Actual original q213156431/a60824497/k20633: the existing18 fixed inputs
   j in {1,10316,20633}, t in {0,1,q-1}, each sign of b. Only the correct
   signed omega is used for each lift; preserve conjugate distinction.
5. Refuse d=0, nonunit omega, noncanonical coordinates, q=0, q divisible
   by the field characteristic and noncanonical x. The fixed q3/x50,
   omega=1 control over Fp101 exhibits the excluded ramified chart.
6. Issue native evaluated identity records for the q3 Pell identity and
   cleared-denominator lift residuals, plus plus-one corruptions. These are
   evaluated identities under the maintained checker, not formal proofs.

Acceptance: zero finite-equivalence mismatches, exact original lifts and
replay, refusal distinctions, preserved conjugate/ramification controls,
source/binary qualification and retained failures. Any mismatch rejects
this implementation or the exact stated lemma, as appropriate. Resource
failure is censored, not a route exclusion. No parameter/target sampling,
timing, paid execution, full-base enumeration or solver campaign is planned.

## Degree and solver gate

The projected equation still has degree q. A split squarefree q-root fibre
has a q-dimensional coordinate algebra; materializing a dense q-by-q
multiplication operator is a different, potentially expensive operation
from evaluating a logarithmic-size recurrence. State splitting and field
hypotheses explicitly. Do not claim a lower bound on every solver.

No representation-only output passes the root-discovery gate. The successor
must act on the projected relation constraints without triple enumeration,
lift returned roots through the unchanged original system, and charge
construction, failed attempts, yield, indexing, selection, kernel, recovery
and memory to the one-target cold cost. Keep the original criteria, private
query tags, distinct raw columns and one-retained-row contract.

Clarification before qualification: control4 uses the nine earlier raw
torus parameters and each one's conjugate,18 algebraic chart cases total.
Earlier18 signed point-image controls varied the curve ordinate, a different
sign. The conjugate uses public omega^-1, which is outside the original
positive-j label set; it is a separate chart control, not another original
base member. Inputs, count and acceptance criteria here are unchanged.
