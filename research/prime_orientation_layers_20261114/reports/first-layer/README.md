# Actual first crossing-jet factors and base-chart integrality

The completed orientation kernel's first crossing-jet matrix now has an
explicit weighted approximation with `2^u` rational 0/1 factors. Its
residual is one bit above the actual diagonal weight. The same source derives
base-chart integrality through a literal polynomial bridge, eliminating the
earlier exposed base-content premise. Central claims
PCC-20261114-800c08/f93c33/ab220a remain unsigned [pcc:unverified].

## Exact construction and domain

Use the original rational/2-adic Cauchy tensor model, `L=2^a`,
`S={-L,...,L-1}`, `C(T)=product_(s in S)(T-s)`, `kappa=C'(0)`,
`alpha=2^(a+1)`, and `delta=y-x`, for actual ports `0<=x,y<L`.
The preceding orientation construction uses

```
M_xy = { j<u : bit_j(x)=bit_j(y) }
B_delta(T) = product_(s in S, s!=delta)(T-s) / kappa
F_xy(T) = product_(j in M_xy) ((2^j-T)/(2^j-delta))^m_j
W_xy(T) = B_delta(T) F_xy(T)
G_i = m_i - sum_(i<j<u) (j-i)m_j
```

All preceding node values, origin identity and degree statements refer to
this same polynomial W and remain unchanged. This is not a different
level-specific deformation or the separate elliptic norm chart.
For the layer theorem require `i<u<=a` and every retained `m_j>0`.
Global chart integrality instead uses feasible `G_j>=0` at every retained
level. Helper declarations retain their exact, sometimes weaker hypotheses
in the proof map [pcc:unverified].

## The actual base-content bridge

The established normalized-square/common-scalar source already constructs
`LocalCharts.localBase` over the concrete subring of rational q satisfying
`padicNorm 2 q<=1`. Its normalizer is derived from actual punctured products,
Lucas parity and factorial identities. Those results are reused, not claimed
as new.

The new bridge proves the literal grid bijection `k -> k-L`, identifies the
punctured polynomials and actual derivative normalization with the old center
product, and proves an identity of rational polynomials [pcc:unverified]:

```
map integralRing.subtype (LocalCharts.localBase a m s)
 = B_(node s).comp (C(node m)+C(alpha)*X).
```

Polynomial equality follows from evaluation at every rational argument over
an infinite field, not agreement on a finite sample. Mapping actual
coefficients transports the old concrete subring property to the new
coefficientwise `Content 0` predicate. This derives base integrality for every
literal signed source chart, including zero and -L. Multiplying by the
already constructed correction gives actual whole-kernel chart integrality;
no favorable base-content witness is supplied. PCC-20261114-bbdb2c/ab220a
[pcc:unverified].

## Exact diagonal normalization

Set `t=-2^i`, and let `J_xy` be coefficient one of the actual polynomial
`W_xy(t+alpha*Z)`. Define the actual diagonal correction value

```
D_i = F_00(t)
    = product_(j<u) ((2^j+2^i)/2^j)^m_j.
```

It is positive and nonzero. Exact valuation of the numerator is i+1 when
j=i and min(i,j) otherwise. Thus `v_2(D_i)=G_i`, not merely a lower bound.
This exact equality is needed when dividing the strict margin by D_i
[pcc:unverified].

Differentiating the actual punctured polynomial identity at the actual source
node gives

```
Gamma = alpha*C'(t)/kappa
J_xy = Gamma/(t-delta) * F_xy(t)
c = J_00 = Gamma/t * D_i
J_xy/c = [t/(t-delta)] * [F_xy(t)/D_i].
```

Crossing bit equality forces `delta!=t`, so these denominators are nonzero.
The actual nodal derivative ratio equals the reused actual `cauchyRho`, whose
valuation is zero; therefore Gamma is nonzero with valuation a+1. Consequently
c is nonzero and has the exact derived weight

```
w = v_2(c) = a+1-i+G_i.
```

No desired scalar, derivative unit or valuation is assumed.
PCC-20261114-e9e4c2/bdaedb [pcc:unverified].

## Matching and nonmatching retained classes

If `x mod2^u=y mod2^u`, the actual displacement is divisible by `2^u`, all
retained bits agree, and the actual mask is the full retained range. Hence

```
F_xy(t)/D_i = product_(j<u) (2^j/(2^j-delta))^m_j.
```

Each ratio is one modulo two. The base ratio `t/(t-delta)` is also one modulo
two because i<u. Products and powers retain that residue. Thus `J_xy/c` is
one modulo two on a matching class [pcc:unverified].

If retained residues differ, the actual first differing bit s is below u
and omitted from the mask. At a crossing s!=i. For s<i, anchor i contributes
an extra `(i-s)m_i`; for i<s<u, omitting anchor s avoids the charged loss
`(s-i)m_s`. Positivity of retained multiplicities gives at least one extra
bit. The actual correction chart has content at least `G_i+1`; its constant
coefficient therefore does too. After division by D_i it has content1.
The actual crossing base ratio is integral since
`v_2(t-delta)<=i`. Thus `J_xy/c` is zero modulo two outside matching classes
[pcc:unverified].

## Explicit factors and absolute residual

Zero out noncrossing entries and call the resulting normalized first-jet
matrix R. Use actual labels `r in Fin(2^u)` and rational 0/1 entries

```
U[x,r] = indicator(x mod2^u=r)
V[r,y] = indicator(y mod2^u=r)
```

The finite matrix product is exactly
`(U*V)[x,y]=indicator(x mod2^u=y mod2^u)`. Both matrices are integral.
The preceding actual-coefficient analysis gives

```
VBound 1 (R[x,y]-(U*V)[x,y]).
```

With actual weighted row factors `A=c*U` and actual unnormalized crossing
first-jet matrix J_cross,

```
VBound w A[x,r]
VBound (w+1) (J_cross[x,y]-(A*V)[x,y]).
```

Feasibility `G_i>=0` makes the row factors integral as well. Every factor,
normalization and residual is explicitly bound to the actual same kernel.
There is no desired-rank or supplied-factor premise.
PCC-20261114-800c08/f93c33/9ba2ee [pcc:unverified].

The `2^u` count is for the full port crossing matrix, both bit classes. It is
not the rank of the entire higher-precision or higher-jet deformation. This
count may exceed the old `2^(i+1)` tile count, so no asymptotic improvement
follows from the layer alone. Additive weights must remain additive; all
higher layers, actual mode density, degree/precision/recovery, child entry
costs and recursive enumeration remain to be charged [pcc:unverified].

## Evidence and requirement status

Frozen six-module source `e422291022ee713b7097438f322baa4a0bf794f2`.
The pinned axiom gate audits82 declarations with0 extra trusted axioms;
import-inclusive fresh OrientationLeading exits0 with default imports.
All82 new theorem records are unsigned. Exact source/legacy dependency hashes,
failed drafts and raw logs are retained [pcc:unverified].

| Requirement | Status and evidence |
| --- | --- |
| Literal base-content binding | implemented; actual polynomial map, bbdb2c/ab220a |
| Actual diagonal coefficient and weight | implemented; e9e4c2/bdaedb |
| Actual first-jet factors/residual | implemented; 800c08/f93c33/9ba2ee |
| Independent source audit | pinned gate82/0; unsigned evidence |
| Fresh replay | exit0 with default imports; fresh-001.log/exit |
| All higher precision/jet layers | open |
| Complete expected ECDLP bit cost/existence or nonexistence | open |
| Publication merge | protected concurrent-ledger conflict, report-only |

[Protocol](../../PROTOCOL.md), [proof map](../../PROOF_MAP.md),
[declaration pins](../../evidence/claim-pins.json),
[source/legacy pins](../../evidence/source-freeze.json),
[gate](../../evidence/gate-001.log), [editable diagram](layers.dot),
[SVG](layers.svg), [PDF](report.pdf), [editable report](report.tex).

Literal certificate `IDC1h57849461da3c8854` checks only the ring substep
`(N-U*V)c=cN-c(UV)`. It cannot certify masks, derivative/grid binding,
valuation, matrix sums, rank or expected cost; those quantified statements
use the Lean sources. The3 new certificate-library tests, formatting and
strict Clippy pass; old checker/native fixtures were not rerun. No native cryptographic fixture, solve, timing/scaling
panel, paid/cloud campaign or previously blocked norm fixture is reported.

Sources checked(KB): unavailable, fallback queries and primary openings in the
journal/protocol. Actual prior LocalCharts/LocalIntegral and normalizer source
were read before implementation; their Git blob IDs match the reused files.
Pinned Oct25 D.4.3/D.4.5 provides the original kernel/mode interface. The
preceding repeated-anchor and common-kernel problem preserve prior finite
counterexamples and the retained-u Fibonacci qualification. No exhaustive
novelty claim or inference from an unsuccessful search.

Canonical scoreboard/timeline and previous study charts are unchanged: this
round contains no performance or solve value. The new source-linked diagram
shows the exact component and open algorithmic dependencies.

Separate [27-statement precision control](../../evidence/REFINEMENT_SCOPE.md)
locally elaborates outside this82-statement audit/fresh scope, unsigned
PCC-20261114-ee0465. It retains2^v labels for precision v-u+1; no higher-jet
or improved density result is included [pcc:unverified].
