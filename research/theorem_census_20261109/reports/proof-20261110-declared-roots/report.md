---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Fixed-size resultant roots and two Lean disproofs

Over an algebraically closed field, with positive declared sizes $m,n$
and actual degree bounds $\deg f\le m$, $\deg g\le n$, Lean proves
the following criterion. [pcc:unverified]

$$\operatorname{Res}_{m,n}(f,g)=0\iff
(\exists a:\ f(a)=g(a)=0)\ \lor\ (f_m=g_n=0).$$

The sizes remain fixed when actual degrees drop. A vanishing resultant
need not give a common finite root. The corrected criterion and both
negative controls are fully quantified in the combined Lean binding.
[pcc:unverified]

![Actual algebraic case split and the checked domain controls. [pcc:unverified]](proof.pdf){width=94%}

\newpage

# What the controls actually disprove

| Exact Lean instance | Actual degrees | Declared sizes | Common finite root |
|:--|:--|:--|:--|
| $f=g=1$, every field | 0,0 | 2,2 | None |
| $f=g=X^2+1$ over $\mathbb Q$ | 2,2 | 2,2 | None |

Both resultants are zero. The first has both declared top coefficients
zero and directly refutes the universal affine-only criterion, even over
algebraically closed fields. The second has leading coefficients 1,1 and
shows why the converse needs splitting or algebraic closure.
These are kernel proofs, not field samples. [pcc:unverified]

If one top coefficient is nonzero, that polynomial has its full declared
degree. Over an algebraically closed field it splits. Mathlib's actual
root-product formula expresses the resultant as a nonzero leading factor
times the product of evaluations at its roots. If both tops vanish,
both actual degrees are strictly smaller and the fixed-size Sylvester
determinant vanishes. Resultant commutation handles the opposite full-degree
side without assuming monicity or separability. [pcc:unverified]

Main binding: `declared_resultant_root_audit_certificate` in
DeclaredResultantRootsAudit. The proof map is `../../DECLARED_ROOTS.md`;
exact statement/source hashes are in `../../declared-root-sources.json`.
Whole census gate431passes, zero failures/zero trusted-native exceptions.
Local source commit `1029720b` precedes packet `PCC-20261110-2b8991`.
Actual local verdict and unsigned standing are recorded separately.
[pcc:unverified]

Lean 4.35.0 / Mathlib `56a00585` are pinned. Complete source 02 compiles with
no warnings and only propext, Classical.choice and Quot.sound. Attempt 01's
failed coefficient-one simplifications are retained with exact source,
raw output and exit status. No hole, custom axiom, native decision
exception or linter waiver. [pcc:unverified]

Sources reviewed: Mathlib Resultant.Basic root-product and degree-padding
lemmas, IsAlgClosed.Basic splitting definition, Polynomial.Roots, and the
existing actual GeneralS4 common-root/gluing and F5 constant-model counterexample.
Prior Semaev primary read scopes are unchanged. Original Semaev (2004)
remains unread after 403. No novelty or source-status promotion. [pcc:unverified]

This is the explicit finite-root/leading-zero polynomial theorem. It does
not construct projective space or finish the actual elliptic point-list
zero-sum recursion. Canonical evaluation and point-list induction remain
the next bridges. No native experiment, timing or paid compute.
[pcc:unverified]
