# Orientation-guarded anchors: prospective proof contract

2026-11-14. Objective remains a novel classical non-generic sub-exponential
algorithm for general prime-field ECDLP, with complete expected bit cost.
This candidate addresses the original rational/2-adic tensor W model;
it is not a map from the separate elliptic norm coefficient chart.
No algorithm or priority claim is established by this protocol.

## Actual candidate

Let L=2^a, grid S={-L,...,L-1}, C its monic nodal polynomial, alpha=2L,
and B_delta(T)=C(T)/(C'(0)(T-delta)), with actual puncture cancellation.
For row/column indices 0<=x,y<L put delta=y-x. Use u<=a retained bits,
nonnegative multiplicities m_j and

    M_xy = {j<u : bit_j(x)=bit_j(y)},
    F_xy(T)=product_{j in M_xy} ((2^j-T)/(2^j-delta))^m_j,
    W_new[x,y](T)=B_delta(T)*F_xy(T).

Every included denominator must be proved nonzero from the actual bit
condition. F_xy(delta)=1. Prove the actual quotient [pcc:unverified]

    H_xy(T)=((F_xy(T)-1) divMonic(T-delta))/C'(0)

satisfies W_new-B_delta=C*H_xy, rather than supplying H or a successful
factorization. Node values including origin identity remain exact. With
S_m=sum_j m_j, target degree bounds are W_new<=2L-1+S_m and
H_xy.degree<S_m (zero polynomial handled with WithBot degree).
This is inside the earlier H-degree budget when S_m<=2L [pcc:unverified].

## Distinguishing prediction

The old guard 2^(j+1)|delta omitted valid crossings whose lower bits differ.
The actual bit-equality guard includes j=i for every level-i crossing,
but omits the first differing bit s, where v2(delta)=s. For j!=s,

    v2(2^j-delta)=min(j,s).

For the actual scaled chart T=t+alpha*z, v2(t)=i<a, the numerator has
coefficientwise valuation >=min(i,j) for j!=i and >=i+1 for j=i.
For delta=0 use denominator valuation j. Target uniform inequality:

    correction coefficient gain >= G_i,
    G_i=m_i-sum_{j>i}(j-i)m_j

on every crossing entry, not just the diagonal. If every G_i>=g>=0,
prove the whole corrected chart remains integral and every crossing gains
g bits relative to the actual base-chart bound. Noncrossing active nodes,
inactive nodes, origin and the -L endpoint must all be checked [pcc:unverified].

The diagonal is unchanged, so the old necessary mass g*F_(2u) remains;
this candidate does not evade that degree cost. The older all-a obstruction
cannot be substituted for the actual retained-u cost. The special
(0,1), i=1 pair must be covered analytically rather than silently excluded.
All bounds are prospective hypotheses until kernel checked [pcc:unverified].

## What would and would not establish progress

Acceptance: exact actual bit/valuation bridge and all-node polynomial
identities/budgets, no supplied desired rank, source-pinned Lean proofs
without holes/new trusted axioms, independent gate and import-inclusive
fresh replay, unsigned PCC records, editable report and inspected PDF.
Preserve failed attempts. Generic arithmetic lemmas alone do not discharge
the actual model binding.

Uniform content gain is not a weighted rank factorization. The masks at
j>i vary inside an old source tile; any tile/Walsh/moment refinement must
be charged. A useful next step needs actual integral U,V factors and
precision-adjusted/degree-adjusted counts, all child-entry and recursive
costs. No lower rank, improved exponent, norm-chart bridge or complete
ECDLP conclusion follows from this protocol [pcc:unverified].

No native fixture, solver, search/scaling panel, paid/cloud/attack/F4/msolve,
Python cryptomath, vault, signing, force push, ledger rewrite or gate bypass.
The draft XP-20261101 bounded search is not executed by this proof work.
Any future experiment needs its own reviewed/frozen contract and live
admission; the earlier norm fixture remains independently unlaunched.

Sources checked(KB): unavailable. Primary files and bounded GitHub queries
are pinned in the session journal. Read the repeated-anchor source before
selection: its explicit missing-mask counterexample and retained-level
qualification motivate the candidate. The code/packing source limits exact
pair filtering; this entrywise completion does not assume that template.
No exhaustive novelty claim. Conductor executable unavailable. Scope this
new study/project, current journal and append-only claim records.
