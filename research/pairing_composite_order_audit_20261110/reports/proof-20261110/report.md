---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Composite-order Tate nondegeneracy: a Lean disproof

The literal nontrivial-root condition in Galbraith, Theorem 26.3.3, does
not suffice for composite n. Lean constructs two actual nonsingular curves
that satisfy the stated hypotheses, then proves that no map on the stated
domains can have both left bilinearity and left nondegeneracy. [pcc:unverified]

| Field / n | Actual curve | Actual point / order | Counts: E, left, right |
|:--|:--|:--|:--|
| F₃ / 4 | y²=x³+x+1 | P=(0,1), order 4 | 4, 4, 4 |
| F₅ / 6 | y²=x³+1 | P=(0,1), order 3 | 6, 6, 6 |

In both cases, −1 is a nonidentity nth root of exact order two, while a
primitive nth root is absent. Both actual target quotients are nontrivial
and have exponent two. Each left domain contains the nonzero double 2P.
[pcc:unverified]

![Actual quotient domains and the checked radical obstruction. [pcc:unverified]](proof.pdf){width=92%}

\newpage

# Exact obstruction, proof map and source boundary

Let S be an additive group, R any type and T a multiplicative monoid such
that z²=1 for every z. If e is additive in its left input, then for every Q,
e(P+P,Q)=e(P,Q)e(P,Q)=1. Left nondegeneracy would imply P+P=0.
A nonzero double therefore contradicts the simultaneous properties.
The generic Lean proof requires no axioms. [pcc:unverified]

The concrete domains are actual Mathlib objects: S=ker[n] on E(Fq),
R=E(Fq)/nE(Fq), and T=Fq*/(Fq*)ⁿ. The finite curve counts are connected
through the nonsingular-point equivalence. Actual point additions and
orders are kernel checked. Lagrange proves every rational point is killed
by n in these examples, giving the left and right counts in the table.
No canonical-pairing implementation or substitute lookup group is assumed;
the proof excludes every map with the asserted properties. [pcc:unverified]

| Module / declarations | Checked obligation |
|:--|:--|
| `BilinearObstruction`: `nonzero_double_obstructs` | Exponent-two target forces all doubles into the left radical |
| `CompositeTateThree`, `CompositeTateFive`: `source_hypotheses`, `P_order` | Actual nonsingularity, counts, point orders, coprimality, divisibility and nonidentity nth roots |
| Both: `target_square`, `target_nontrivial` | Actual field-unit quotient exponent and nontriviality |
| Both: `primitive_root_absent`, `primitive_two_control` | No primitive nth root; exhibited root has exact order two |
| Both: `left_cardinality`, `right_cardinality` | Actual n-torsion and curve-quotient counts |
| `CompositeTateCertificate` | Direct universal negation and combined certificate |

Main namespace: `CompositeTateCertificateAudit`. Main declaration:
`composite_tate_audit_certificate`; direct negation:
`literal_nontrivial_root_nondegeneracy_claim_false`.
Whole gate: 50 declarations, zero failing proofs and zero trusted-native
exceptions. Lean 4.35.0 / Mathlib 56a00585; only propext, Classical.choice
and Quot.sound. Failed quotient-coercion and record-syntax builds are
archived losslessly. Legacy-import advisories remain, without waivers or
changes to issued dependencies. No native experiment, timing or paid compute.
[pcc:unverified]

Source commit `0b81d51a` precedes packet `PCC-20261110-5e63e9`.
Actual local packet verification and signed standing are recorded in
`../../verification.json`; signed standing is unverified. Source pins and
archive inventories are in `../../sources.json`. [pcc:unverified]

The official chapter's printed pages 576–577 were visually inspected;
section 26.3 definitions and Theorem 26.3.3 were read. They use natural n,
coprimality, divisibility, actual quotient domains and the nontrivial-root
condition; the prime-order restriction occurs later. The official errata
list contains no chapter-26 entry in the bounded check. No novelty is claimed.
The original Frey–Rück paper remains unread after failed direct AMS access.
[pcc:unverified]

A primitive-root-qualified theorem and prime-order pairing theorem are
not refuted. Correctly qualified canonical Weil/Tate constructions and
MOV/Frey–Rück calculations remain open. The corpus node already requires
correct fields and domains; no blanket status promotion is made. The
polynomial identity-certificate schema cannot express these group quotients;
the Lean binding supplies the faithful proof. KB and Conductor unavailable;
primary-source/repository/Mathlib fallback used. [pcc:unverified]

[Official Chapter 26](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch26.pdf)
· [Official errata](https://www.math.auckland.ac.nz/~sgal018/crypto-book/crypto-book.html)
