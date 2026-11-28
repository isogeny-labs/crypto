---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# What the trace classification actually implies

Lean proves a conditional bound for an actual Weierstrass curve over any
finite field. Put $q=|K|$, $N=\#E(K)$ and $t=q+1-N$. If prime $r$ divides
$N$ and $t^2\in\{0,q,2q,3q,4q\}$, then
$\operatorname{ord}_r(q)\in\{1,2,3,4,6\}$, hence at most 6. The same theorem
proves $\gcd(r,q)=1$ and existence of an actual curve point of additive
order $r$. The trace classification is an explicit premise.
[pcc:unverified]

| Exact integer trace case | A power forced to be one modulo prime $r$ |
|:--|:--|
| $t^2=0$ | $q^2$ |
| $t^2=q$ | $q^3$ |
| $t^2=2q$ | $q^4$ |
| $t^2=3q$ | $q^6$ |
| $t^2=4q$ | $q^2$ (indeed $q=1$ modulo $r$) |

These are symbolic implications, not empirical field samples. Divisibility
of the actual point count gives $q+1=t$ modulo $r$. The five polynomial
identities and a small divisor table yield the listed orders.
[pcc:unverified]

![Proof obligations: the trace classification stays an explicit assumption. Actual point existence is obtained separately by Cauchy's theorem.](proof.pdf){width=96%}

\newpage

# A negative control that stays within its scope

The combined Lean certificate also checks the arithmetic instance
$q=289$, $N=324$, $r=81$. Here $r\mid N$, $\gcd(r,q)=1$,
$t=-34$ and $t^2=4q$, while $\operatorname{ord}_{81}(289)=9>6$.
Thus primality cannot simply be dropped from the count-only argument.
No actual curve or point of order 81 is asserted by this control.
[pcc:unverified]

The actual prime-order point theorem is different: the affine point
equivalence gives a finite point type, and Mathlib's additive Cauchy theorem
supplies a point from prime divisibility of its group cardinality. Local
classical decidability supplies the group-law instances. The main theorem
quantifies the actual field, curve and prime divisor rather than assuming
a modular-order conclusion. [pcc:unverified]

The main binding is `trace_classification_audit_certificate` in
SupersingularTraceClassesAudit. The proof map is
`../../SUPERSINGULAR_TRACE.md`; exact normalized declaration and source
hashes are in `../../supersingular-trace-sources.json`. Lean4.35.0 and
Mathlib `56a00585` are pinned. Attempt06 compiles using only propext,
Classical.choice and Quot.sound; failed attempts01/04/05 are retained.
No custom axiom, hole, native decision exception or linter waiver.
[pcc:unverified]

The whole-project gate passes331declarations with zero failures and zero
trusted-native exceptions. Source commit `c60c141e` precedes packet
`PCC-20261110-9c7d48`; its actual local-check record accompanies this report.
Producer-local checks do not create signed verification standing. [pcc:unverified]

Freeman–Scott–Teske (2010), Definition2.1/Remark2.2 pp230–231 and the
trace-square list p237 were read. Original Waterhouse Theorem4.1and proof
beginning pp536–537were also read and visually inspected after a successful
resumed fetch. Its parity/characteristic/congruence restrictions give the
necessary square-list premise. Exact read scopes and PDF hashes are in the
source plan. No novelty claim or proof of the classification is inferred. [pcc:unverified]

Geometric supersingularity implies the classification, and modular order
equals the actual cyclotomic embedding-field degree: these bridges remain
separate proof obligations. The older actual-field $r=2$ disproof remains
unchanged. This conditional bound and its numerical control do not finish
the general supersingular theorem or the fourteen-item audit. No new native
experiment, timing or paid compute. [pcc:unverified]
