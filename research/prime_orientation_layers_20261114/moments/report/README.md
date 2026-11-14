# Actual first-jet factors with additive moment weights

Frozen source `6e97fcc1fffc1dcbdc25770aac701c2e252905c5` contains43 authored
statements. The pinned shared gate audits44 declarations, including generated
momentRight.congr_simp, with0 extra trusted axioms, exit0. Import-inclusive
fresh OrientationMoment is running. All records are unsigned; gate data
PCC-20261114-24441c [pcc:unverified].

## Exact construction

Keep the SAME actual completed orientation polynomial, its retained bit mask,
source ports Fin(2^a), actual first crossing coefficient J and actual diagonal
c!=0 with derived valuation w=a+1-i+G_i. Let R=J/c at crossings and0 elsewhere.
For i<u<=a, positive retained multiplicities, all actual ports and k>=0, the
new construction uses labels (r,b) in Fin(2^u) times Fin(k) [pcc:unverified].

Set r=x mod2^u and h=x div2^u. The actual integer identity x=r+2^u*h fixes
the row coordinate. Matching retained row residues have the same actual mask.
For each actual prototype denominator d=q-delta(r,y), set e=2^u/d. The
selected anchor denominator has valuation<=j<u and the pole denominator
has valuation<=i<u; hence every relevant e has valuation>=1, derived from
actual bits and denominators. Exact polynomial first-jet comparison yields
[pcc:unverified]

    R[x,y] = R[r,y] / D_(r,y)(h),
    D_(r,y)(H) = (1+e_pole*H) * product_selected_j (1+e_j*H)^m_j.

The actual finite polynomial D depends on r,y, not h. Its constant is1 and
coefficient b has valuation>=b. No favorable denominator or approximate
polynomial is supplied. PCC-20261114-84a6cb [pcc:unverified].

## Constructed inverse and finite error

Construct F=invOfUnit(D,1) with the pinned standard power-series API. Its
coefficient recurrence expresses coefficient n through earlier coefficients
and the actual D. Strong induction and finite-product/convolution valuation
bounds give v2(F_b)>=b. The actual identity D*F=1 follows from D_0=1.
Define Q_k=trunc_<k F [pcc:unverified].

The finite polynomial D*Q_k-1 has zero coefficients below k; coefficient b
retains valuation>=b. Evaluating at actual integer h therefore has valuation
at least k. D(h) is1 modulo2, hence nonzero with valuation0. Division derives
[pcc:unverified]

    VBound(k, D(h)^(-1)-Q_k(h)).

This is a constructed truncation with an actual evaluation-error proof,
not an assumed good approximation. PCC-20261114-a88b83 [pcc:unverified].

## Explicit finite matrix factors

Write f_b(r,y) for the actual inverse-series coefficient of D_(r,y). Define
[pcc:unverified]

    A[x,(r,b)] = indicator(x mod2^u=r) * 2^b * (x div2^u)^b,
    B[(r,b),y] = R[r,y] * f_b(r,y) / 2^b,       b<k.

If prototype r,y is noncrossing, B is zero. Otherwise actual prototype R
is integral, v2(f_b)>=b, and the derived factors are integral. A has weight
at least b. The exact finite matrix product is R[r,y]*Q_k(h), yielding
normalized residual k. The actual weighted row factor cA has weight w+b,
with absolute residual w+k. Feasibility G_i>=0 makes cA integral as well.
No desired factorization/content premise is supplied. Central records
PCC-20261114-3be17e/15d101/e3550d/f04bf6 [pcc:unverified].

The exact label count is2^u*k, including k=0. For k>=1, it is at most the
previous row-refinement upper count2^(u+k-1). This is a comparison of symbolic
factor counts for the stated first jet. It is not a timing/bit-cost comparison
with the prior source's existing linear-moment method. At k=1 it matches
the separately proved necessary count2^u. PCC-20261114-f6a09d
[pcc:unverified].

## Evidence and requirement status

| Requirement | Status |
| --- | --- |
| Same actual polynomial/first jet/scalar | bound through exact parent identities |
| Actual denominator, even slopes, row-class invariance | derived in source |
| Actual inverse/truncation/error | constructed, not assumed |
| Integral finite factors and exact count |2^u*k, normalized residual k |
| Actual additive weighted profile | row w+b, residual w+k |
| Pinned source audit |44 declarations/0 extra trusted axioms, exit0 |
| Fresh replay | running; final exit not claimed yet |
| All required higher chart jets | open; separate7-lemma integrality draft only |
| Coefficient construction/bit/entry/recovery/recursion costs | open |
| Public finder/extraction/complete expected ECDLP bit cost | open |
| PR integration | protected concurrent ledger conflict; auto-merge off |

A separate7-statement draft derives actual retained crossing base-chart content
a+1-i, whole completed crossing-chart content w and all-coefficient integrality
after division by actual c. It locally elaborates only, outside this43/44
project, data PCC-20261114-310194. It supplies no higher-chart factorization
[pcc:unverified].

No new native crypto fixture, search, timing/scaling experiment, paid/cloud
campaign or old native tests were launched. Parent fixed literal scaling
certificate IDC1h57849461da3c8854 applies only to its stated ring rearrangement;
it is not a parameterized inverse/valuation/matrix/count/complexity certificate.
The universal statements use the Lean project. All failed drafts, raw outputs,
source/dependency pins and exact declaration bindings are retained.

[Protocol](../PROTOCOL.md), [proof map](../PROOF_MAP.md),
[source pins](../evidence/source-freeze.json),
[declaration/claim pins](../evidence/claim-pins.json),
[gate receipt](../evidence/gate-validation-001.json),
[editable report](report.tex), [PDF](report.pdf).

Sources checked(KB): unavailable, wrapper has no executable. Read prior actual
LocalCharts/LocalIntegral and repeated-anchor linear moments, pinned Mathlib
PowerSeries.Inverse/Trunc and polynomial evaluation, and actual parent first-
jet proofs. Standard inverse-series moments are reused without a priority,
fastest-method or complete-algorithm novelty claim. The2^u mask multiplicity
may exceed old2^(i+1) tiles. Every original higher-jet/mode/degree/precision/
entry/recovery/recursive/public-input/expected-bit-cost obligation remains.
No performance or solve point is added to the canonical scoreboard.
