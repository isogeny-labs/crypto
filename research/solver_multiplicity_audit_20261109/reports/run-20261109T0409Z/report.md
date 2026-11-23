---
geometry: margin=16mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Exact multiplicity and solver controls

2026-11-09. All 16 registered msolve 0.6.5 calls completed with exit 0 and
passed the frozen native checks. Lean proves an actual two-variable ideal
has one distinct F5 point but quotient length 2. This refutes equating
support count with algebraic multiplicity; it does not refute Diem's bound.
Claim PCC-20261109-ee57d0, unsigned. [pcc:unverified]

![Recorded exact counts. Each fixture has two repetitions in each mode; both repetitions agree. No sampling interval or accepted clock metric. Run solver-multiplicity-20261109-v1.](support-vs-length.pdf){width=98%}

The formal family is I=(xy,x+y−c). Lean constructs
K[x,y]/I ≃ₐ[K] K[t]/(t²−ct), proves dimension2, proves equality with the
triangular ideal (x+y−c,y²−cy), and classifies every field solution. At c=0,
the unique point has total multiplicity 2. At c≠0, the two points are
distinct. Closed declaration:
BivariateMultiplicityAudit.support_is_not_multiplicity. Expanded transitive
gate: 93 declarations, 0 failing, 0 trusted-native exceptions; no custom axiom,
sorry or native_decide. [pcc:unverified]

msolve prints qdim 2 and eliminant degree 1 for the double point, 2 and 2 for
split points, 1 and 1 for the simple point and F2 degree control. Independent
native analysis checks normalized bases, input reductions, S-polynomials,
finite standard-monomial counts and every base-field point. The degree
control {x²+x,y²+y,xy+x+y} returns {x,y}; both GB runs select pair degree 3.
That logged degree is exploratory; the prior Lean lower/upper certificates
are separate mathematical statements. [pcc:unverified]

Flow: 16 planned, 16 attempted, 16 completed; 0 not-completed, 0 interrupted,
0 resumed, 0 excluded. Four fixtures × two modes × two repeats. Exact sparse
linear algebra (-l2), one thread, no saturation, 30-second timeout per call.
msolve source commit 5e72b9d9; binary SHA256 prefix 390a41bd8d8c.
Native analysis and frozen protocol committed before calls in f0bea543.
Protocol hash prefix cff556238d40; full identities are in protocol.json and
provenance.json. The canonical export and every referenced file hash validate.
[pcc:unverified]

Held-out c=2 preflight was seen before freeze and only calibrates format and
budget. Its wrong-constant/zero-polynomial controls pass. No A/A variance,
null population, significance or confidence claim: these are exact known
answers. Default genericity can add variables and use time-seeded random
forms; raw parameters and containing epoch intervals are retained. Counts,
not parameter bytes, are compared. One shared local host, zero paid spend,
approved local checkpoints, no accepted timing or independent-host claim.
[pcc:unverified]

Sources: pinned msolve main.c and msolve.c:123–181 document distinct qdim
and parameter fields; upstream nonradical_shape-31 already prints 11 and 8.
Mathlib AdjoinRoot gives the quotient basis machinery. Diem2011's general
n!dⁿ bound, C-to-F4Run refinement and the remaining14-item/paper backlog
remain open. Raw data:../../evidence/runs; exact checks:../../evidence/analysis.jsonl;
schema/hash-validated export:../../export.json; source/build/dependencies/host:
../../provenance.json. [pcc:unverified]
