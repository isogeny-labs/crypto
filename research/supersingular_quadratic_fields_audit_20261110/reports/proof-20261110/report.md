---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Every general model over actual F4 and F9

Lean proves point-count classifications and modular embedding-degree
bounds for every nonsingular general Weierstrass model over explicit
actual fields with 4 and 9 elements. The source criterion is exactly
gcd(t,q)≠1 with q the proved field cardinality and t=q+1−#E the actual
point-group trace. No general Hasse or Waterhouse classification premise
is assumed for these checked fields. [pcc:unverified]

| Field | General coefficient tuples covered | Source-supersingular point counts |
|:--|--:|:--|
| F4 | 1,024 | 1, 3, 5, 7, 9 |
| F9 | 59,049 | 4, 7, 10, 13, 16 |

Every prime divisor r has modular degree ordᵣ(q)≤6, coprimality with q
and an actual point of order r. Concrete prime-order-7/13 curves over
F4/F9 both have exact modular degree 3. [pcc:unverified]

![Complete model coverage through actual fields, bijections and kernel checks. [pcc:unverified]](proof.pdf){width=96%}

\newpage

# Proof obligations, controls and assurance

F4 is QuadraticAlgebra (ZMod 2) 1 1, with α²=α+1. F9 is
QuadraticAlgebra (ZMod 3) (−1) 0, with β²=−1. Root-free base polynomials,
field axioms, computable arithmetic, characteristics and cardinalities are
proved. Composite ZMod 4/9 rings are not used as fields. [pcc:unverified]

For F4, a monic quadratic fibre has at most two roots. An explicit
sigma-fibre equivalence gives counted(E)≤2q+1=9. Positivity and the
source trace-gcd criterion yield every count case. This is a structural
all-model proof, not a claimed 1,024-row native enumeration.
[pcc:unverified]

For F9, Lean proves characteristic-3 square completion on every general
model, with y′=y−a₁x−a₃ and coefficients a₂′=a₂+a₁²,
a₄′=a₄−a₁a₃, a₆′=a₆+a₃². Equation equivalence, an explicit coordinate
map and inverse, and preservation of the counted set are checked. All
729 normalized models are classified by nine separate 81-model kernel
decisions and explicit a₂ exhaustion. The bijection transfers this to
all 59,049 general tuples. [pcc:unverified]

Nonsingularity binds the counted equation set to actual Mathlib points.
The checked trace lists imply the prime-divisor modular bound and
coprimality. Cauchy's theorem produces an actual prime-order point with
the caller's equality decision and group instance kept explicit.
The actual controls are y²+αy=x³+α²x over F4, of order 7, and
y²=x³+βx+1 over F9, of order 13. Both are nonsingular, satisfy the exact
source criterion and have modular degree 3. [pcc:unverified]

| Module group | Checked obligation |
|:--|:--|
| QuadraticModels / SourceTrace | Actual fields, cardinalities, populations and exact source criterion |
| FibreBound / CompleteSquare | Generic quadratic root bound and characteristic-3 count bijection |
| NormalizedCase00–22 / NormalizedAll | All 729 normalized models with exact kernel decisions |
| QuaternaryClassification / NonaryClassification | Every general model, actual counts and prime-divisor conclusions |
| ExtensionControls / QuadraticCertificate | Actual degree-3 controls and combined certificate |

The whole audit passes 42 declarations, zero failures and zero trusted-native
exceptions. Lean 4.35.0 / Mathlib 56a00585; only propext, Classical.choice
and Quot.sound. No holes, custom axioms, native_decide, linter waivers or
increased elaboration limits. All 57 attempt/source/resource files and five
gate files have byte-equal archive checks. The first monolithic reduction
was resource-interrupted with SIGTERM/143; its source/log/resource snapshot
and all rejected attempts are preserved. The case import chain serializes
large kernel reductions in normal Lake builds. [pcc:unverified]

FST 2010, printed pages 230–231, and its page-237 trace list, plus
Waterhouse 1969 Theorem 4.1, printed pages 536–537, supply the checked
source definitions and known classification context. Exact URLs, primary
hashes and limited read scopes are retained. No new full-paper read or
novelty claim. Higher extensions, geometric supersingularity equivalence
and full torsion-field interpretation remain separate obligations.
[pcc:unverified]

No native experiment, fresh sampled certificate, timing claim or paid
compute. KB wrapper status 127 / MCP unavailable; bounded primary,
repository and Mathlib fallback. Conductor unavailable. Source commit,
packet and actual verdict are in verification.json; signed standing remains
unverified. Proof map and pins: README.md and sources.json. [pcc:unverified]
