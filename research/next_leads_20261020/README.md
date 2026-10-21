# Next paths after the five-lead end-to-end cohort

This is a research portfolio, **not** a claim that the proposed variants have
been implemented or timed. The fixed baseline is the frozen five-lead run in
[`../five_leads_e2e_20261019/STATUS.md`](../five_leads_e2e_20261019/STATUS.md).
The frozen inputs here were read from local source commit `1271df4fa`;
their seven projective session digests are recorded in the linked cohort
summary.
All cost ratios below are cold, one-target operation-count ratios at
`M + 0.5S + 10I` and 0.1 M per declared native logical operation unless
specified. They are not wall-time ratios. `analyze.py` reads the existing raw
1,260-target projective cohort and F257 mapped-run receipt; `headroom.json`
is its deterministic output. Run with
`python3 research/next_leads_20261020/analyze.py > research/next_leads_20261020/headroom.json`.

## What the counters now say

The 1,260-target genus-2 projective route costs 3,137,323,980
source-addition equivalents (GAE), or **2.881445×** its matched source rho
(1,088,802,370 GAE), while beating its affine collector by **0.642714×**.
Its 221,270,948 processed divisors are unchanged from the two-choice
collector. Holding the existing logical counts fixed, making every charged
field multiplication, square, and inversion free leaves 1,688,381,340 GAE,
or **1.550678× rho**. Charged field additions alone are 1,375,240,795 GAE,
or **1.263077× rho**. These are algebraic floors for the *current* accounting
and counts, not lower bounds for a redesigned solver. A uniform total-cost
scaling would need a **65.295% reduction** to tie rho; changes to the mix of
operations can reach the same threshold differently.

The mapped F257 genus-3 cold run costs 568,870,295 GAE, or **807.291×**
source rho. Its first relation pass costs only 4,095,228 GAE (**0.720%** of
cold cost). The separately recorded mapped setup plus descent costs
234,373,285 GAE; the increment through destination verification is
317,106,337 GAE. These phase figures are taken from monotonic snapshots;
setup cannot be separated from descent in this receipt. Scalar relation
work/recovery costs another 13,287,315 GAE. Faster line enumeration alone
has limited headroom on this fixture.

## Executable leads

| Path | Why it is live | First decisive experiment and controls | Current status |
| --- | --- | --- | --- |
| **1. Target-coefficient rank collector** | The old two-choice component/cycle proxy was 1.000857× its affine incumbent and 4.483247× rho on the 1,260 targets. The projective chart processed 221.27 million divisors. | A frozen signed-potential target-coefficient preview was implemented and charged on the same 21 curves × 60 targets, with both candidate additions and valid relations retained. A cheaper one-candidate or different score remains a separate experiment. | **Measured loss:** all 1,260 source answers and replays verified; [full result](../target_rank_20261021/README.md) is **1.016333× projective** and **2.928507× rho** at the representative price. Its ordinary-merge score occurs on 99.984393% of valid relation previews. |
| **2. Complete genus-2 law plus batchable branch arithmetic** | The existing fraction-free chart still uses a normalization inversion and 483,273 exceptional affine fallbacks. All 42 recorded Scholten models passed transformation to the monic quintic form required by the complete inversion-free [Ozbay Gürler–Hisil law](https://link.springer.com/article/10.1007/s13389-024-00351-7). An independent [Rust divisor-arithmetic implementation](https://github.com/salindne/divisor-arithmetic) exposes an optimized affine law and batched inversions; it is distinct from the published inversion-free law. | Implement the complete law and independently cross-check each addition/doubling against affine Cantor on the same 21 curves, including exceptional inputs. Compare same-host operation counts and qualified wall time against our chart, the external Rust package, and smalljac where its model applies. Batch only independent candidate additions; sequential walk steps cannot be batched unchanged. | **Correctness gate complete, speed open:** the pinned external `add(D,D)` fails 1,312/1,342 equal-input cases, but dispatching equal inputs to `double` passes [85,764 ordered additions](../genus2_external_gate_20261021/README.md) on the 21 curves against Cantor. No full-cost ratio exists. The 1.550678× fixed-logical floor still applies to backend-only changes. |
| **3. Genus-3 extension torsion → odd-kernel quotient → mapped DLP** | The F257 positive control has a Frobenius-stable order-27 subgroup over F257^6, although the rational Jacobian order has 3-adic valuation one. Thus rational rank-three torsion is an invalid gate. The 20 frozen Smith-negative p31 curves still lack an actual ell=3,5 odd-kernel screen. [Tian's construction](https://arxiv.org/html/2007.03172) needs a maximal isotropic kernel and a quotient/map. | Compute full Frobenius action on extension-field ell-torsion and Weil pairing; enumerate invariant maximal isotropic subgroups, then reconstruct quotient and evaluate a map on P/Q. Replay F257 as positive control, use deliberately nonisotropic and nonstable kernels as negative controls, then run all 20 frozen p31 inputs. For any successful candidate, run the mapped line-sieve DLP and source verification and price complete cold work against rho. | Open. The p31 order probe timed out; 15/20 one-infinity model gates and the F257 mapped DLP are only partial prerequisites. [General genus-2/3 isogeny algorithms](https://arxiv.org/abs/1709.06063) give another construction route, not a working adapter yet. |
| **4. Quartic mapped descent and verification arithmetic** | On F257, mapped setup/descent and destination verification account for 551.48 million of 568.87 million GAE. The initial line pass is 0.72% of total. | Profile the evaluation-space Jacobian, divisor normalization, mapped descent and destination equality checks separately on F257. Replace one hotspot at a time with independent group-law cross-checks and full source verification. Compare cold single-target costs and growing-q fixtures, retaining the same map, relations, and rho controls. Treat setup reuse as a separately reported multi-target result. | Proposed. A faster relation enumerator alone would save at most 0.72% here if all other costs stay fixed. The [degree-four sieve study](https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/SievingB.pdf) also cautions against assuming its line-sieving variant improves degree-four double-large-prime work. |
| **5. Mixed-fiber PKM solver audit and external oracle** | Pair-table mixed/rho is 3.2222×; exhaustive polynomial/rho is 32.2975×; current F4/rho is only a **≥232,110× lower bound** because several primitives are uncharged. A tower-structure coverage bonus was absent in the earlier 18-cell screen. | Finish primitive accounting for all 540 F4-backed DLPs, export the same 180 relation-query systems to [msolve](https://github.com/algebraic-solving/msolve) and Magma F4 on the same host, and compare exact root sets with the exhaustive oracle. If a solver is materially faster, reprice the full mixed pipeline, including setup, failed queries and source checks, then test larger instances. | Proposed. The current lower bound cannot support a fastest-implementation claim or an end-to-end win. |
| **6. Cubic transfer generality and quotient certificate** | The one published-map DLP is correct but 645.65× rho at representative price, and a class gate on the supplied formula is not an independent quotient construction. | Construct the 3-isogeny quotient from the checked kernel rather than the published coefficient fixture; prove/evaluate the map on divisor classes, handle denominator-zero inputs, and run mapped DLPs on additional public small-field fixtures. Keep each single-target setup and source verification in cost. | Proposed. This closes a mathematical coverage gap; it does not imply a speed gain. |

Three adjacent mechanisms were already opened in
[PR #2304's newer branch](https://github.com/aburan28/crypto/pull/2304),
which diverged from this local checkout. They should be kept distinct from
new measurements in this directory:

| Path | Current evidence on that branch | Decisive next run |
| --- | --- | --- |
| **7. Genus-2 point-driven pencil collector** | A transferred `F_739` fixture has a 20-target, source-verified three-point-relation solve. The early-stop prototype uses 67 walks and 7,244 verified relations per 20-target round; its 6.073 ms median online call is an unisolated host diagnostic with a different setup boundary from the 2.955 ms two-point and 0.536 ms rho controls. Witness buckets reduce direct candidate-verification multiplications from 22,396,106 to 903,030 per round versus full-root scans. | Freeze common setup/online boundaries and run matched isolated comparisons on the original 1,260 targets, including source checks, matrix work, rejected candidates, complete cold costs, and an external implementation. This tests whether the stage win survives the full solver. |
| **8. Adaptive quartic small-prime base** | Exact fixed-base eligible-line populations exist on the two published quartics; no charged adaptive policy has solved a transferred DLP. | Train a target-blind promotion rule on disjoint curves, then compare the same mapped DLP and line-sieve engine with fixed and adaptive bases, including duplicate handling, graph/rank updates, setup, and matched rho. The F257 phase floor above still applies to a relation-only change. |
| **9. Direct trace-zero index calculus** | The published Tian cubic source has non-base `j`, so it fails the input gate for a base-defined trace-zero method; this does not reject the method generally. | Freeze a fresh base-defined cubic-extension cohort with prime-order trace-zero subgroups; execute relation solving and verify each scalar, then price against whole-curve index calculus and Frobenius-aware rho. See the [trace-zero construction](https://arxiv.org/abs/1405.1059). |

The ordering is a suggested execution path based on measured bottlenecks and
missing certificates, not a claim that any hypothesis will work. Paths 1 and
2 should be measured individually **and** together: a solver can win against
its current index-calculus baseline without crossing rho, and a component
gain is not an end-to-end gain. For path 3, do not replace extension-field
torsion by the rational-order shortcut that failed the positive control.

The [five-lead report](../five_leads_e2e_20261019/REPORT.md) supplies the
frozen protocols, raw sessions, source-verification receipts, replay audits,
and the exact prior losses. No new DLP, kernel, or solver experiment was run
for this portfolio; `headroom.json` is a recomputation of existing records.
