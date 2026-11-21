# Exact normalized scaling and full closure under a target-row change

2026-11-14. Source `85c57a726a644a6d2138a5c97785ab1119a06aff`.
All claims unsigned [pcc:unverified].

The normalized target row has an exact inverse-pair pole-scaling law.
More decisively, the actual change of rows preserves the full iterated
degree-budgeted closure whenever the budget admits both input families.
This is a statement about the actual symbolic coefficient algebra, not just
equality of finite-field solution sets. It isolates what this normalization
can change and redirects the search for a closure improvement beyond a
constant row basis [pcc:unverified].

Central claims: normalized scaling `PCC-20261114-46dee9`, generic full closure
`PCC-20261114-5f67e7`, actual normalized row `PCC-20261114-044d98`, actual full
closure `PCC-20261114-d07690`. Gate/build receipt `PCC-20261114-f6eb11`.
[Scaling source](../lean/NormTargetGrading.lean),
[closure source](../lean/NormRowClosure.lean),
[prospective contract](../TARGET_GRADING_PROTOCOL.md),
[exact proof map](../target-grading-evidence/claim-pins.json),
[editable diagram](closure.dot), [SVG](closure.svg), [PDF](report.pdf).

![The exact row transformation preserves the full closure once both sets of inputs are admitted. The lower-budget and nonlinear routes remain separate. Evidence unsigned.](closure.pdf){width=88%}

## Exact normalized scaling

Keep the actual monic norm quotient, actual Dickson dividend and actual
membership quotient from the previous component. Write $d=2^{n+1}$ and
$R_e$ for the membership remainder. Its normalized target row is

$$
T_e=e^{d-3}(M_n(2)(r)-c)
 -(c_0+c_1e^2r)J_e Q_e(e^2r),
$$

where $Q_e$ is the computed monic quotient and

$$
J_e=2za_1+4e^2zra_2+6e^4zr^2
 -e(3r^2+a)(c_0+c_1e^2r)-2e^3z^2c_1.
$$

The prior actual division gives $R_e(e^2r)=e^3T_e$. Parameter one is
exactly $T_1=R_1(r)$. Evaluating the full-remainder transport at the scaled
target and cancelling only the explicit nonzero factor gives

$$
e^{d-3}T_1(a_1l^4,a_2l^2,c_0l^3,c_1l)
 =T_e(a_1,a_2,c_0,c_1),\qquad el=1.
$$

The curve premise $z^2=r^3+ar+b$ and $d\ge3$ remain explicit. The identity
holds at every stated depth over a field; it is not yet an identification
with a formal multivariate weighted homogeneous component. Thus weight29
at depth4 remains a candidate upper bound, not an established exact degree
[pcc:unverified].

## Full closure, rather than static input multiples

Let $A$ be a commutative algebra over a field $K$, and let
$\delta:A\to\mathbb N$ be an explicit degree function. The inductive
closure at budget $D$ allows:

1. an input $f$ only when $\delta(f)\le D$;
2. zero, addition and multiplication by a scalar in $K$;
3. multiplication of **any already derived row** $f$ by $a\in A$ when
   $\delta(a)+\delta(f)\le D$.

These are the literal full-closure rules of the existing ML source,
rebuilt at this study's pinned toolchain. Ordinary total degree and
weighted total degree can each instantiate the explicit function. No
static Macaulay-space surrogate, available GB or normal-form oracle is
assumed [pcc:unverified].

If input sets $F,G$ have equal $K$-linear spans and every member of both
sets is admitted at $D$, their complete closures at $D$ coincide. The proof
first puts each input span in the opposite closure, then transports every
derivation, including multiplication of derived rows. It needs no theorem
about the size of a solution set [pcc:unverified].

## Binding to the actual normalized target row

For an actual degree-below-five remainder $V\in A[X]$, public $q\in K$
and public $s\ne0$, the old inputs are its five coefficients. The new
inputs are $sV(q)$ and coefficients1 through4. Polynomial evaluation gives

$$
V(q)=V_0+qV_1+q^2V_2+q^3V_3+q^4V_4.
$$

This is an invertible constant row transformation: solve for $V_0$ using
$s^{-1}$. Hence the generator spans agree. For the actual membership
remainder the degree guard follows from monic norm/support division;
$q=e^2r$ and $s=(e^3)^{-1}$ give exactly $T_e$. Public curve and parameter
values are mapped into $A$ by its algebra map, while the retained
coefficients remain arbitrary elements of $A$. In particular, $A$ may be
the multivariate polynomial ring in those coefficients: this is a
symbolic identity, not equality after sampling [pcc:unverified].

Consequently, once both original and normalized input families fit $D$,
the same rows are derivable in their full closures. The theorem keeps
both admission hypotheses explicit; it does not assert a numerical
admission threshold for an unproved weighted grading [pcc:unverified].

## What the test decides

Constant row recombination alone cannot lower the full closure above that
input-admission threshold. This tests the proposed mechanism before using
a degree fall as evidence of a faster solver. It leaves open effects below
the threshold, implementation changes, different degree functions,
nonlinear charts, different factor bases and other non-generic algorithms.
It does not show that the original closure is large: the transformed row
may still expose short derivations already present in it. It is not a
generic-group lower bound or a global impossibility result
[pcc:unverified].

The exact transformation and scaling remain useful algebraic information.
A subsequent candidate needs to change something beyond this constant
input span, or supply actual bounded derivations and a complete solver
cost. The original nonscalar-origin/jet family still needs its own faithful
map; this norm normalization is not a map to that family [pcc:unverified].

## Sources, validation and retained failures

Sources checked (KB): unavailable as previously documented, no unchanged
retry. The [bounded receipt](../target-grading-evidence/prior-search-receipt.json)
records three fallback queries and actual primary sources. Read ML commit
`8020e131a1a3806531d695b6bc7586473802dada`
`formal/ICFallDegrees/{Closure,VariableClosure,FiniteQuotient,PointIdeal}.lean`.
The finite-quotient result needs an available GB/reduced-ideal subsystem;
the point-ideal result needs budgeted coordinate derivations. Neither
discharges those premises from a small zero set. Prior weighted-GB source
review and Mathlib's weighted homogeneous definitions remain relevant.
This bounded review establishes no novelty or exhaustive absence
[pcc:unverified].

Final normalized build002 and closure build005 exit0. There are15 authored
theorem bindings. The whole-study gate audits369 declarations, including
the new generated `InClosure.brecOn`, and reports0 additional trusted uses,
exit0. Standard foundations only. Exact source/log hashes are in the
[gate receipt](../target-grading-evidence/gate-build-001.json), claim
`PCC-20261114-f6eb11`; theorem claims are unsigned/default not_covered
(exit2). Import-inclusive fresh replay exits0, including both new modules;
validation receipt PCC-20261114-6643d4 binds the exact hashes
[pcc:unverified].

Retain normalized draft001's parameter-one wrapper error, closure drafts001
through003's predicate, field, notation, recursive parameter and coercion errors,
and draft004's unnecessary tactic-sequencing warning. The final source
fixes these without changing statements, suppressing checks or adding
trusted axioms. No Rust/native mathematical source changed and no prior
native test was counted again. The inherited canonical SHA256 clippy and
benchrec diagnostics remain untouched [pcc:unverified].

No fixture, unknown-root, F4/msolve, paid/cloud/attack or timing experiment
was attempted. The earlier native fixture remains1 planned,0 attempted,
0 completed,0 excluded,1 admission-blocked by checkpoint upload transport.
Local Lean proof construction is separate. Complete growing-family
closure, an efficient public-input finder, extraction and expected bit
cost remain open [pcc:unverified].

Global performance/curve/attack-route graphs are unchanged: this component
adds no measured speedup or group-transport edge. All previous sources,
evidence, claims and canonical ledger prefix are preserved. The new visual
records the exact mechanism and its scope [pcc:unverified].


## Exact proof map

All entries unsigned [pcc:unverified]. Declaration/context hashes and source
freeze are in claim-pins.json.

| Declaration | Claim |
|---|---|
| `NormTargetGrading.actualTargetFactor` | `PCC-20261114-17b15a` |
| `NormTargetGrading.normalizedOne` | `PCC-20261114-4f506d` |
| `NormTargetGrading.membershipOne` | `PCC-20261114-d8ecad` |
| `NormTargetGrading.targetValueScale` | `PCC-20261114-f41fe3` |
| `NormTargetGrading.normalizedScale` | `PCC-20261114-46dee9` |
| `NormRowClosure.transport` | `PCC-20261114-7326e0` |
| `NormRowClosure.inputSpan` | `PCC-20261114-d0640f` |
| `NormRowClosure.closureSpanInvariant` | `PCC-20261114-5f67e7` |
| `NormRowClosure.evaluationRows` | `PCC-20261114-a28623` |
| `NormRowClosure.rowsMembership` | `PCC-20261114-a2c68b` |
| `NormRowClosure.rowSpans` | `PCC-20261114-bcec52` |
| `NormRowClosure.rowClosure` | `PCC-20261114-1f8f28` |
| `NormRowClosure.actualMembershipWindow` | `PCC-20261114-87bdd8` |
| `NormRowClosure.actualNormalizedRow` | `PCC-20261114-044d98` |
| `NormRowClosure.actualRowClosure` | `PCC-20261114-d07690` |
