---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# A complete all-smooth F5 control

For $y^2=x^5-x$ over $\mathbf F_5$, Lean proves there are exactly
16 actual reduced polynomial pairs. Every $u$ splits and every $v=0$.
The curve polynomial is monic, degree five and separable. Identity is
included. This supplies the fixed all-smooth native control before data.
[pcc:unverified]

| Exact control | Complete pairs | Split $u$ | Smooth fraction |
|:--|--:|--:|--:|
| Same equation over F3 | 8 | 7 | $7/8$ |
| Same equation over F5 | 16 | 16 | $1$ |

These are exact polynomial-pair models, with complete coefficient-shape
proofs. The formal identification with the geometric Picard/Jacobian
group remains separate. Two small fields do not refute a fixed-genus
asymptotic limit or a qualified large-prime/typical-subgroup heuristic.
[pcc:unverified]

![Actual polynomial completeness and splitting give a pre-data calibration control. Dashed obligations remain open.](proof.pdf){width=95%}

\newpage

# Proof and execution scope

The model includes every monic $u$ with $\deg u\le2$, every $v$ with
$\deg v<\deg u$, and requires $u\mid v^2-f$. An explicit bijection
with the complete coefficient shapes proves the cardinality. Distinct
shapes give distinct polynomial pairs. [pcc:unverified]

An exact monic quotient/remainder identity proves the actual division
test equivalent to finite scalar equations. Kernel `decide` classifies
651 candidates and finds 16 valid pairs. Every valid quadratic $u$ has
a root $r$ over F5. The actual factorization
$(X-r)(X+a+r)=X^2+aX-r^2-ar$ proves splitting. This is the
`Polynomial.Splits` predicate. [pcc:unverified]

The curve derivative is $-1$, giving separability by an explicit Bezout
certificate. Scalar v-zero conditions are proved for every candidate and
transferred to every actual pair. The proof adapts the earlier F3 shape
method while rechecking characteristic-five equations. A recursion bound
4096 is a finite-reduction resource limit, with no trust relaxation.
[pcc:unverified]

The combined binding is:

`MumfordFiveControlAudit.mumford_five_control_certificate`.

The whole tracked project gate passes 240 declarations, 0 failures and
0 trusted-native exceptions, including generated declarations. Only
propext, Classical.choice and Quot.sound are accepted. Lean 4.35.0 and
Mathlib revision `56a005855ea17e0b0958f73ec801ae140b0a1679` are pinned.
The failed first build and passing second build are retained. Lake's
module-system advisories remain visible. [pcc:unverified]

Source commit `11100a85` precedes packet `PCC-20261109-e81718` and scalar
supplement `IDC1ha387b9e22b14b9cf`. The full local packet check passes at
21:56:51Z on producer host `762f5fc120c9`; signed standing remains unverified. The supplement accepts
32 derived and 11 extra seeded points over $2^{61}-1$ for the supplied-root
quadratic factorization only: three variables, degree two. It does not prove
the cardinality, completeness, splitting or geometry. The finite control
rests on Lean. [pcc:unverified]

The reused native Rust driver and separate C++ arithmetic implementation
build successfully. They have not been invoked on curves. Calibration
protocol f2572b4d was frozen at 21:52:34Z and must be committed before
affected calls. It requires exact F3/F5 counts and unequal/equal detector
controls, all actual addition/group/subgroup checks, source and binary pins,
failure retention and no paid compute. Its elapsed intervals are scheduling
diagnostics, with no scientific timing claim. [pcc:unverified]

Stoll2014's representation/addition sections remain read at the scopes in
../../cantor-sources.json. Example 4.19 (pp. 25--26) was also read in full
for a source-known nonzero-v/order-five control of the later full F3 census.
That is a pre-data prediction and still needs a faithful Lean extension.
Cantor1987's original publisher PDF remains unread after 403. No source is
promoted merely because a related note was read. [pcc:unverified]

Exact source/binding hashes and reproduction are in ../../FIVE_CONTROL.md
and ../../five-control-sources.json. The original fourteen-item audit
remains partial. Geometry, scaling families, qualified asymptotics,
independent-machine validation and the other original bounds remain open.
No performance or curve-comparison metric changed. [pcc:unverified]
