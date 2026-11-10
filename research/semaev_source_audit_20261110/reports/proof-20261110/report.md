---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# A missing plus in the original binary S4 display

Lean disproves the literal product parsing of the continuation lines in
Semaev 2004, page 5, Remark 1. Inserting the missing addition gives an expression
that Lean proves equal to the actual Sylvester resultant over every field
of characteristic two, every a and every coordinate tuple. This concerns
the printed display; the recursive construction is retained. [pcc:unverified]

Write s1=x+y+z+w, s3=xyz+xyw+xzw+yzw, s4=xyzw. The repair is:

```text
s1^4 + s3^4 + s4*(s3+s1)^2 + s4^2*s1^2 + s3^2
```

The literal display multiplies its third and fourth terms. Over F₄ with
alpha²+alpha+1=0, the tuple (1,1,alpha,alpha²) has s1=s3=s4=1.
The actual resultant is zero; the literal product is one. All 16 tuples over
F₂ agree with both expressions, so prime-field-only checks miss the issue.
[pcc:unverified]

![Universal repair, prime-field control and actual F₄ point witness. [pcc:unverified]](proof.pdf){width=96%}

\newpage

# Actual points, proof map and manuscript read receipt

On the nonsingular actual curve E: y²+xy=x³+1 over F₄, Lean constructs
P=(1,0), T=(0,1), R=(alpha,0) and S=(alpha²,alpha²). Actual Mathlib
addition formulas give P+P=T, R+S=T and T+T=O. Hence P+P+R+S=O,
while the literal display evaluates to one on their x-coordinates.
The proof directly negates both universal formula equality and literal
vanishing on actual four-point zero sums. It does not assume a lookup group
or an algebraic-closure existence bridge in place of these rational points.
[pcc:unverified]

| Module / declarations | Exact checked obligation |
|:--|:--|
| `BinaryFourth`: `binary_resultant_coefficients` | Actual curve invariants and actual quadratic resultant coefficients |
| `BinaryFourth`: `actual_resultant_corrected` | Universal repaired formula, all characteristic-two fields, a and tuples |
| `BinaryFourthControl` | Actual F₄ cardinality/root relation; fixed symmetric quantities all one; resultant0/literal1 |
| `BinaryFourthPoints` | Actual nonsingularity, four point equations, three actual additions and zero sum |
| `BinaryFourthCertificate`: `all_F2_tuples_agree` | Kernel check of all 16 prime-field tuples |
| Certificate: `literal_printed_recursion_claim_false`, `literal_printed_vanishing_claim_false` | Direct universal negations over actual F₄ |
| Certificate: `binary_fourth_source_audit_certificate` | Universal repair plus actual counterexample and prime-field control |

The main namespace is `SemaevBinaryFourthCertificateAudit`. The full gate
checks 28 declarations, with zero failures and zero trusted-native exceptions.
Lean 4.35.0 / Mathlib 56a00585 are pinned; only propext, Classical.choice and
Quot.sound. No holes, custom axioms, native_decide, linter waivers or added
elaboration settings. All 31 attempt files, including failed recursion,
normalization and missing-import builds, are archived losslessly.
Source commit `44b32e67` precedes theorem packet `PCC-20261110-95607f`.
Actual local verdict and signed standing are retained in verification.json;
signed standing remains unverified. [pcc:unverified]

The public author-uploaded original is dated 5 February 2004: six pages,
131064 bytes, SHA-256 1cab0ae675996cdba6393b1e369a22113fd2e74bcf1223595743dc140631c7d6.
The entire manuscript, theorem proof, remarks and references were read;
pages 2 and 5 were visually inspected. This is an additive current read receipt
for S-SEMAEV-2004. Historical frozen plans still record their former failed
IACR access. The typography claim is scoped to this exact PDF version, with
no assertion about every later source or novelty. [pcc:unverified]

The original theorem uses algebraic-closure points; section 3 explicitly
permits quadratic-extension y-coordinates. The earlier rational-only
converse counterexample concerns an additional restriction. Existing
all-arity geometric equivalence and individual-degree certificates stay
unchanged. All-arity polynomial permutation invariance, absolute
irreducibility, arbitrary-split exact normalization and qualified attack
costs remain open obligations. No native experiment or paid compute.
[pcc:unverified]

The byte-identical prior supplement IDC1hbf1f44625ee306dc covers the scalar
six-coefficient resultant identity only. Its fixed 61-bit-prime evaluations
cannot prove characteristic-two or actual-point claims; the new Lean proofs
do so. KB tools/wrapper and Conductor unavailable; bounded primary,
repository, Mathlib and formula/errata searches were used. No accepted
correction source was found in that search scope. [pcc:unverified]

Primary source: [author-uploaded manuscript](https://www.researchgate.net/profile/Igor-Semaev-3/publication/2869464_Summation_Polynomials_and_the_Discrete_Logarithm_Problem_on_Elliptic_Curves/links/5440f69f0cf228087b6a11ba/Summation-Polynomials-and-the-Discrete-Logarithm-Problem-on-Elliptic-Curves.pdf).
Source pins, proof map and archives: study README.md, sources.json,
coverage.json and verification.json. [pcc:unverified]
