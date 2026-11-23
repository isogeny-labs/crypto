#set page(paper:"a4",margin:17mm)
#set text(font:"Libertinus Serif",size:10pt)
#set par(justify:false)
#set document(title:"Linear-denominator rational index calculus: complete cold screen")
= Linear-denominator rational relations
*Complete cold same-target screen · 15 November 2026*

The specialized full solver improves its generic rational reference by
1.46–3.02× in fixture medians, with exact deterministic work equality.
Primary median ratios against the fastest existing same-target arm range
from 0.70× to 1.43×. The registered 10× pilot criterion fails. These are local
observational wall times; the evidence supports the arithmetic improvement
and retains the complete-pipeline losses.

#image("LINEAR-COLD.svg",width:100%)

#table(columns:(1fr,1fr,1fr,1fr),inset:5pt,
 [Genus, prime],[Existing / linear],[Generic / linear],[Targets],
 [2, 1009],[1.434×],[2.099×],[3],
 [2, 16381],[0.750×],[1.456×],[3],
 [3, 127],[0.839×],[2.156×],[3],
 [3, 1009],[0.696×],[2.597×],[3],
 [4, 31],[0.905×],[2.388×],[3],
 [4, 127],[0.755×],[3.020×],[3])

*What ran.* Six reused dense curves, family0, with three newly measured
target seeds5,6,7 each. Sixteen fresh-child arms per target: twelve native/table
controls, compact, generic rational, specialized rational and a duplicate
specialized A/A arm. The primary denominator is specialized cold total;
the numerator is the minimum of the fourteen existing arms, including compact
and generic. A/A is excluded from choosing the baseline. Every arm has the
same curve, subgroup, generator, target and walk seed. No target amortization.

*Raw evidence.* #raw("linear-cold-evidence/summary.json") and the complete
stdout/stderr/receipt/fixture archive #raw("LINEAR-COLD-RAW.tar.gz"). Closed
record #raw("20261115T0930Z-hec-linear-cold"); source #raw("ed0fb2c5f982").

#pagebreak()
= Algebra, correctness and complete costs
For $d=2$, let $M=g+4$, $A=product_i(x-x_i)$, $u_i=sigma_i y_i/(A'(x_i))$,
$H=sum_i u_i A/(x-x_i)$, $S_0=sum_i u_i$, $S_1=sum_i u_i x_i$. Then the
subset pair has $b=S_0 x-S_1$ and $a=b H-S_0^2 A$. At a root $r$ of $b$,
$a(r)=-S_0^2 A(r)$. This replaces generic polynomial GCD by checking the
linear denominator at the anchors, including zero/constant-denominator cases.
Gray updates compute all anchor probes by additions/subtractions while storing
them in the original integer sign order. A monic quotient recurrence computes
only required upper norm coefficients. Canceled anchors are removed explicitly.

*Exactness.* All 2688 curve/sign cases match the subset reference coefficient
for coefficient and match generic primitive numerator/denominator/residual and
anchor masks. The panel covers 118 common factors and 13 constant denominators,
with additional forced zero/constant controls. Twelve earlier complete solver
runs preserve every deterministic count and scalar. All 25 CI-target tests pass
locally. Three evaluated identity certificates cover the barycentric/subset
numerator equality: #raw("LINEAR-IDENTITIES.json"), including
#raw("IDC1h0ee3a91b91e99243"). Each has32derived and16extra producer recheck
points; formal-proof pointers are null. No independent issuer or novelty claim.

*E6 passes.* All 18 full group-gate children pass, checking 17064 raw principal
and 600 projected anchor identities. All 288 timed cells exit successfully and
recover the expected scalar. Generic/specialized and A/A target identities and
deterministic counts agree on all 18 targets. No experimental timeout, cap,
missing cell or failed answer. Changed-input and timeout runner self-tests are
separate intentional diagnostics. Total driver wall8.18s, summed child wall7.976s.

*E7 fails.* Every fixture median is below the pre-registered10× threshold.
A/A spread reaches2.55×, so the small positive primary result has no controlled
confidence claim. Failure concerns this implementation/policy and panel; it
does not prove a universal limitation of rational interpolation.

*Cold boundary.* Method totals include catalogue/tables, descents and target
support, anchors, whole signed batches including unused tails, normalization,
factorization/orientation, deduplication, forest, algebra and final scalar
verification. Input parsing, common fixture validation/target construction and
extra harness checks are excluded equally. Full group gating is separate.
Single-target fresh children run sequentially on local ARM, L0 with no isolation
assertion or cloud spend. Source/arguments/input SHA are checked before reduction.

*Remaining cost.* Median per-solve relation fractions are69.6/70.7% at genus2,
66.4/69.5% at genus3 and75.8/78.3% at genus4, in increasing field-size order.
The next structural candidate should change useful-relation supply or selection.
Another equivalent arithmetic rewrite is not justified by the10× cost gap.
Do not multiply these ratios by earlier engineering or rank-count gains.

An exploratory fixed-work diagnostic removes compact's harvesting time per
target before aggregation. Its maximum best-existing / remaining-cost ratio
is3.62×. A harvesting-only same-work replacement therefore misses10× under
that assumption; the next cost case needs joint downstream changes. This is
not a universal bound. Source: #raw("HARVEST-ONLY-BOUND.json").

*Provenance.* Protocol commits #raw("e38d00ce0e"), #raw("ce6ebaa97b") precede
scientific source #raw("ed0fb2c5f982") and the pre-run S3 checkpoint at09:34UTC.
Existing fifteen-arm cohort/receipts remain unchanged. Native Rust executes and
collects; native jq reduces; Rust/Typst render. Shared registry tools handle
generic ledger/custody/progress metadata separately. Primary interpolation
source: #link("https://arxiv.org/html/1211.6895v3")[D'Andrea–Krick–Szanto, Theorem3.1].
KB receipts and assumptions are in #raw("LINEAR-PROTOCOL.md").
