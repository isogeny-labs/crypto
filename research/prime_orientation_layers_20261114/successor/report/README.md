# Actual first-jet precision factors and a necessary factor count

Frozen source `8455c485411b19c3b6a99cfc6c95f32276867b9b` contains40 authored
statements. The pinned shared gate audits44 declarations, including4 generated
congruence lemmas, with0 extra trusted axioms, exit0. Import-inclusive fresh
OrientationRank exits0 with default imports. These are unsigned producer checks, gate data
PCC-20261114-341743; no signed standing is asserted [pcc:unverified].

## Actual objects and scope

Use the same orientation-completed polynomial, literal dyadic grid and bit
mask from the frozen82-statement parent. Let R be its actual normalized first
crossing-jet matrix, with noncrossing entries explicitly zero. Its actual
nonzero diagonal c has exact valuation w=a+1-i+G_i. Assume i<u<=a and every
retained multiplicity positive. The parent derives these objects and the
lowest-layer factorization from the actual polynomial, not a rank oracle.
The origin/node values and source degree are unchanged [pcc:unverified].

## Constructed precision upper control

Choose u<=v<=a. If rows x,z agree modulo2^v, their retained masks agree.
For every selected anchor q=2^j, the actual denominator valuation is at most
j<u; the negative pole denominator has valuation at most i<u. Therefore the
actual ratios (q-delta(z,y))/(q-delta(x,y)) differ from1 by valuation at least
v-u+1. Finite products and multiplicities retain this precision. Exact
polynomial derivative/first-jet identities give [pcc:unverified]

    R[x,y] = R[z,y] * comparisonRatio(x,z,y),
    VBound(v-u+1, R[x,y]-R[z,y]).

Prototype integrality is derived from the actual leading-layer proof. It is
not a premise asserting the desired comparison. Noncrossing zeros are handled
explicitly. Choose the literal valid row z=x mod2^v. Define [pcc:unverified]

    A[x,r] = indicator(x mod2^v=r),
    B[r,y] = R[r,y],       r in Fin(2^v).

The exact finite product (AB)[x,y] is R[z,y]. Both factors are integral.
Scaling A by the actual c gives row weight w and unnormalized residual
weight w+v-u+1. Feasibility G_i>=0 makes scaled row factors integral. There
are exactly2^v labels. At v=a the factorization is exact, retaining every
original row. PCC-20261114-9fb7c1/127309 [pcc:unverified].

For k>=1 with u+k-1<=a, this particular construction guarantees k bits using
2^(u+k-1) labels. This is its charged upper count, not a universal lower bound
and not an improved asymptotic or complete mode profile [pcc:unverified].

## Necessary count at the lowest layer

Take one actual row and column for every retained residue r=0,...,2^u-1.
All are literal source ports because u<=a. The parent's explicit actual
leading layer gives the resulting normalized minor M congruent to identity
modulo2 entrywise. The new proof transports finite-product congruence through
the determinant's signed permutation sum [pcc:unverified]

    VBound(1, det(M)-1),  det(M) != 0,  rank(M)=2^u.

This is the rank of that minor. It is not a claim that the full rational
matrix has rank exactly2^u. PCC-20261114-4d783b [pcc:unverified].

For ANY rational factor pair A,B with k inner columns satisfying
VBound(1,R[x,y]-(AB)[x,y]) at every actual port pair, its restricted product
is likewise identity modulo2, hence nonsingular. Standard rational matrix
rank inequalities imply [pcc:unverified]

    2^u <= k.

No factor-integrality assumption is needed. Dividing an absolute weighted
approximation by the derived nonzero c gives the SAME necessary count when
its residual has valuation at least w+1. PCC-20261114-1bce48/543d60
[pcc:unverified].

The parent's2^u-factor construction and this lower bound match at the lowest
normalized bit. Neither theorem makes2^v minimal at higher precision. These
bounds apply to this kernel/jet/approximation contract, not arbitrary
completions, non-generic algorithms, or ECDLP complexity [pcc:unverified].

## Evidence and remaining requirements

| Requirement | Status |
| --- | --- |
| Actual finite-precision factors | constructed,2^v labels; exact at v=a |
| Actual first-layer necessary count |2^u for any rational approximation factors |
| Absolute weighted normalization | actual c,w used, no supplied favorable scalar |
| Source axiom audit |44 declarations/0 extra trusted axioms, gate exit0 |
| Fresh replay | exit0/default imports; final data429114 |
| Literal native artifact | IDC1hbc3887c49b3c9ce8, ring telescope only,32+16points |
| New literal adapter checks |3tests/format/strictClippy exit0 |
| All chart jets and complete weighted mode profile | open |
| Full recursive/public-input ECDLP expected bit cost | open |
| Integration | protected concurrent ledger conflict; auto-merge off |

No cryptographic fixture, empirical scaling/timing panel, paid/cloud/native
search/F4/msolve campaign was launched. No previous checker/Lean source,
claim body, raw evidence or canonical ledger history was rewritten. Failed
drafts and exact logs are retained. All new records remain unsigned.

[Protocol](../PROTOCOL.md), [source/parent pins](../evidence/source-freeze.json),
[gate receipt](../evidence/gate-validation-001.json),
[all40 declaration/claim bindings](../evidence/claim-pins.json),
[versioned final report](report-final.tex), [final PDF](report-final.pdf).
The earlier pending [checkpoint report](report.tex)/[PDF](report.pdf) retain
their exact pinned bytes. Final replay receipt
[validation-001.json](../evidence/validation-001.json), unsigned data
PCC-20261114-429114 [pcc:unverified].

Next: bind the existing reciprocal/geometric-series moment method to this
exact masked family with constructed coefficients and charged costs. A
separate moment draft is outside this40/44 project. Every higher chart jet,
weighted density, degree/precision/recovery/entry/recursion and public
finder/extraction/expected bit cost remains an original obligation.

Sources checked(KB): unavailable, wrapper has no executable. Prior normalizer,
LocalCharts, orientation and repeated-anchor primary sources plus pinned
Mathlib determinant/rank APIs were read. Standard facts are reused; no
priority claim follows from bounded fallback searches. No performance point
is added to the canonical dashboard by these symbolic component results.

## Separate successor checkpoint

A43-statement isolated moment draft locally elaborates actual integral
2^u*k first-jet factors with residual w+k and additive row weights w+b.
It is outside the frozen40/44 audit/fresh scope, unsigned elaboration data
PCC-20261114-75aea8. Its symbolic count comparison is against the row
refinement control, not a runtime comparison with the prior linear-moment
method; all higher-chart and complete bit-cost obligations remain open
[pcc:unverified].
