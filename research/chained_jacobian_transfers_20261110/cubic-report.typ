#set page(paper:"a4",margin:17mm)
#set text(font:"DejaVu Sans",size:10pt)
#set par(justify:true)
= Cubic covers: the arbitrary-class norm prerequisite
2026-11-11 · Chained ECDLP Transfers · crypto PR3877 / queue3904

*Standing: unsigned and unverified.* This implements a bounded missing norm
operation on the existing prime-field cover family. It claims no new cover,
solver speedup or cross-characteristic descent. [pcc:unverified]

#image("cubic.svg",width:100%)

== Exact operation and proof boundary
On E/F101:y²=x³+x+46, the full rational group has prime order109, with
G=(2,37). The cubic covers have genera4 and13. For canonical (u,v), whole
pullback substitutes X³ in both polynomials; degree≤3g fits the target
genus3g+1. Coefficient deflation is a checked inverse on the exact image.
It is not a global genus13 norm. [pcc:unverified]

The actual genus4 norm factors every support polynomial of degree≤4 and
traces the projected elliptic point in Fp[X]/q over all Frobenius conjugates,
with support multiplicity. Factor reconstruction, orbit closure and descent
are checked. All canonical genus4 inputs are within the algorithm's domain;
the test set is130 fixed translated classes, not exhaustive enumeration of
that Jacobian. The prototype restricts checked primes to5..101.
[pcc:unverified]

Norm-after-first-pullback is[3]. The restricted two-hop composition gives[9],
invertible by97 modulo109; first recovery uses73. These recover source
points, not unknown scalars. Four Lean lemmas establish coefficient
left-inverse/injectivity and closed-orbit trace invariance. The12-declaration
gate passes. Finite-field and Picard geometry remain written arguments.
PCC-e1a827 remains unsigned. [pcc:unverified]

== Correctness evidence
All109 source scalars pass both pullbacks and first norms. A predeclared
E/F17:y²=x³+2x+2,order19 control checks361 ordered subgroup pairs and
ramification, reaching degree12 representatives on genus13. The fixed130
norm witnesses include irreducible support degrees3 and4. [pcc:unverified]

The independent root replay imports no new norm code. Existing genus4/13
arithmetic reconstructs all classes, existing affine elliptic arithmetic
reconstructs all expected norms, and a distinct Frobenius-gcd criterion
confirms cubic/quartic support. Receipt cubic-independent-01.txt and witness
TSV are bound by PCC-2d5c08. Both Jacobian engines use Cantor; producer-authored
agreement is not a signed independent verdict. [pcc:unverified]

#pagebreak()
= Restrictions, retained negatives and next task

== What the quadratic obstruction does establish
Odd full elliptic group order implies an irreducible Weierstrass cubic.
If α is a root of f(X²+s), α²+s has degree3, so the tower law makes the
degree of α divisible by3. Thus there are no rational linear or quadratic
factors. The existing three-individually-rational-factor Richelot constructor
and rational-Weierstrass chart cannot start here. This is checked directly
on the E109 sextic. [pcc:unverified]

An unordered triple of nonrational quadratic factors can nevertheless be
Frobenius-stable. No universal Richelot obstruction follows. Next: implement
balanced genus2 arithmetic with two infinities, validate it against the
existing odd-chart engine, then assess explicit non-split kernel descent.
This remains prospective work. [pcc:unverified]

== Prior art and unresolved computational cost
The existing Enge20261024 study already constructs the C(d,s) family, analyzes its
published generic bound and tests rational-point collectors. Its open problem
prime-cover-projected-relation-cost-missing asks for closed primes, norm,
replay and total-cost evidence. This work addresses a bounded arithmetic
prerequisite. The original relation-yield and linear-algebra gaps remain.
An upper-bound comparison is not a lower bound excluding specialized methods.

Increasing genus alone gives no advantage on the embedded order109 subgroup.
The image is recognizable by coefficient deflation. No relation collector,
DLP solver, benchmark, cryptographic-size field or prime-field descent has
been demonstrated. [pcc:unverified]

== Integrity and reproducibility
Producer: cryptanalysis/experiments/chained-transfer-reference/src/cubic_transfer.rs.
Independent replay: crypto/examples/chained_cubic_norm_reference.rs.
Run the consumer check.sh with Rust1.99.1 on PATH; run cargo test --release
--example chained_cubic_norm_reference from crypto. Core Lean4.35.0 audits
lean/CubicPullback.lean using tools/lean/proof-gate.sh. The checkpoint manifest
binds the producer revision and witnesses. First compile/PATH failures remain
retained separately from subsequent passes. [pcc:unverified]

The full crypto library result remains6184 pass/13 fail/133 ignored.
Restoring tracked fixtures fixes6 failing tests;5 still need archived raw data
and2 need unavailable kernel clock metadata. The failed full run remains
failed. PCC-8ffeb3 preserves it. Crypto's frozen ledger fork blocks merging
PR3877. No history, test or evidence was rewritten. [pcc:unverified]

S3 admission and live IsoLab budget/capacity are unavailable, so no experiment
or paid job was launched. KB access is absent and crypto-autoresearcher's
vault remains locked; its integration handoff stays in crypto.

Sources: Galbraith corrected ch.8 §8.3 and ch.10 §§10.3–10.4;
research/enge_primefield_cover_20261024/REPORT.md and its native source;
problems/prime-cover-projected-relation-cost-missing.md. Full source-linked
account: CUBIC_NORM_20261111.md. Canonical transfer/leaderboard graphs remain
unchanged because the evidence is unsigned and contains no performance edge.
