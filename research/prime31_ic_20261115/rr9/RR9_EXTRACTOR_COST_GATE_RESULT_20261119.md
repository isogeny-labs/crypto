# Prime31 RR9: a quantitative make-or-break gate for public-only extraction

Date: 2026-11-19 UTC. [PR #4793](https://github.com/isogeny-labs/crypto/pull/4793). Scope: reanalysis of frozen **native Rust** evidence; **not** a newly benchmarked solver, a non-generic ECDLP victory, or a cryptographic-size extrapolation. Independent signed PCC remains unverified.

## Fixed target and evidence

Use the **unchanged ordinary, cofactor-one** curve over `p=2^31−1=2147483647`, `a=847514846`, `b=2087444724`, prime `n=2147446831`, `G=(625037715,2100627251)`. The source-bound original Rust arithmetic revision is `1df714645aa683cf975bc76fcacb9842cb4780fd`. The new Rust gate validates this curve using the existing `prime31-algebraic-ic::check_curve`; it does **not** implement or measure an ECDLP solve.

- Matched native-affine **same-runner, same 1,024 public targets**: [#4774](https://github.com/isogeny-labs/crypto/pull/4774), [CI #44436754627](https://github.com/isogeny-labs/crypto/actions/runs/44436754627). Signed shared BSGS **3,342.7934425 ms**; ternary W8 generic-table index-calculus control **7,834.9334925 ms**, both fully charged; 1,024/1,024 logs on each. Raw summary: `research/prime31_ic_20261115/rr9/W8_MATCHED_SIGNED_BSGS_SUMMARY_20261119.json` on #4774, blob SHA `0ca2050b3c7e060bd83679dd4d6d31c9b92d9493`.
- Ternary W8 [#4770](https://github.com/isogeny-labs/crypto/pull/4770) counted **42 training queries and 1,177 online queries**, i.e. **1,219** total algebraic-extraction requests in the frozen 1,024-log workload. The existing complete pipeline (#4728) learned B31 full rank through four B8 windows but its relation extractor was a **generic group-table oracle**.
- [#4780](https://github.com/isogeny-labs/crypto/pull/4780), [CI #44452737502](https://github.com/isogeny-labs/crypto/actions/runs/44452737502): the existing *exhaustive* B4 graph-chart algebraic negative scan took **1,445.472364 ms** averaged over two trials and tested all `8^8 = 16,777,216` pairs. The B8 direct algorithm requires `16^8=4,294,967,296` pairs, with **30 Fp products per pair** (not counting precomputation, inversions or additions). Do **not** call the B4/#4780 and BSGS/#4774 times a same-runner paired comparison; they are distinct CI executions.

## Necessary bound for a complete 1,024-log crossover

For a replacement extractor that preserves **the same 1,219 total requests**, even if **all remaining IC setup, rank collection, linear algebra, verification, allocation and dispatch were free**, its mean extractor wall time must satisfy

\[
\bar t_{\mathrm{extract}} \lt \frac{3,342.7934425\ \mathrm{ms}}{42+1177}
= \mathbf{2.742242365\ \mathrm{ms/call}}.
\]

This is a *strictly optimistic necessary bound*, **not** a sufficient condition: actual non-extraction work is positive, per-query distribution matters, and the strongest same-runner optimized generic/index-calculus controls could be faster than this native-affine reference. A planning target with 20% headroom is **2.193793892 ms/call**. With unchanged methods, the currently observed 4,492.14005 ms complete-batch deficit needs to be eliminated.

Under the unchanged graph-chart candidate evaluator, the B8 full negative walk performs exactly

\[
(16^4)^2\times30=\mathbf{128,849,018,880}
\]

Fp coordinate products, *excluding other work*. At the optimistic per-call bound it would demand about **4.70×10^13 coordinate products/s** if every query required a full B8 scan. Even allocating the **entire** 3.343-second 1,024-log BSGS batch to **one** such negative query demands ~**3.85×10^10 products/s**. These are **required hardware throughputs under the fixed exhaustive algorithm**, not lower bounds for F4 or any nonenumerative algorithm. The historical B4 negative case alone consumed 527× the per-call budget if one divides results from two **different** CI jobs; that ratio is illustrative only, not a controlled paired timing.

Thus the next gain must remove enumeration or the number of extraction requests by orders of magnitude; caching/rebuilding speedups alone do not meet the 1,024-log competitive gate. No B8 exhaustive wall-time or F4 unknown-index runtime has been measured here.

## Audit of overlapping leads / avoid duplicating active lanes

| Existing lead | Evidence boundary | Disposition |
|---|---|---|
| #4692 public-only RR9 coefficient/F4/remainder-tower | owner of nonenumerative unknown-index extraction, remains open | **Implement there, do not fork a second extractor** |
| #4728 four B8 windows; #4770 ternary signed weights | 31/31 rank and 1,024 generic-table target logs; weight speedup scoped to generic relation oracle | Reuse Rust recovery pipeline |
| #4774 paired signed shared BSGS | 3.343 s vs 7.835 s W8 generic IC on one host/targets | Frozen baseline and fixed query budget |
| #4776 exterior-4; #4780 graph-chart; #4783 rank-one/cache | algebraic pair and preprocessing speedups; direct search still `16^8` pairs at B8 | Keep as optimized controls; stop chart-only cycles |
| #4779 exact 70/70/70 rank shortcut | excludes fixed **linear** compression for most targets; source-bound Rust producer/checker and PCC still pending | Don't revive fixed-linear Plücker compression |
| #4547 seven-anchor completion; #4548 degree-5 Macaulay; #4772 gradient; #4773 conditional two-prefix | supplied-anchor generic equivalence; negative/inconclusive solver gates; 256 prefix branches | Retain as negative/control families |
| #4746 PKM exponent correction; #4679 F4 split | previous 0.43 exponent withdrawn (corrected 0.621); separate F4 implementation lane ongoing | No asymptotic speed claim from unverified exponent |

Relevant published **prior art**: Petit–Kosters–Messeng (PKC 2016, DOI `10.1007/978-3-662-49387-8_1`), Amadori–Pintore–Sala (FFA 2018, DOI `10.1016/j.ffa.2018.01.009`), and Kudo–Yokota–Takahashi–Yasuda (CANS 2018, DOI `10.1007/978-3-030-00434-7_19`). Prime-field algebraic PDP, hybrid Gröbner and symmetry methods are established, so our contribution here is a source-traceable **operational feasibility bound**, not a novelty claim. Shared research-KB and Conductor were unavailable through the connected tool set; this audit is bounded to named GitHub lanes and those publications.

## Follow-up audit: a substantially stronger historical shared-BSGS arm

A prior merged **same-curve, 1,024-complete-public-log** Rust experiment, [#4532](https://github.com/isogeny-labs/crypto/pull/4532), [source-specific CI #44031618724](https://github.com/isogeny-labs/crypto/actions/runs/44031618724), measured a **2^20-baby reusable generic Shanks BSGS** arm with **103.840 ms** setup, **66.274 ms** online and **170.113 ms** fully charged batch time (independently replayed arithmetic, `[pcc:unverified]`). Reported peak memory after setup was **67,924 KiB**. Its same-runner comparison was with the older mixed-4+4 IC, at 2,221.474 ms; it was **not run alongside** #4774's W8, which used a different native-affine BSGS implementation and measured 3,342.793 ms.

Thus **2.742 ms/request is not the strongest known repository frontier**, and no eventual positive result should stop at beating the #4774 affine-control arm. As a *cross-run sensitivity calculation only*, if the observed #4532 throughput were reproducible against the unchanged 1,219-call W8 workload, the analogous optimistic zero-overhead budget would be

\[
\frac{170.113\ \mathrm{ms}}{1219}=\mathbf{0.139551272\ \mathrm{ms/call}},
\qquad \frac{170.113}{1024}=\mathbf{0.166125977\ \mathrm{ms/solved\ log}}.
\]

The raw wall-time quotient `3342.7934425/170.113 ≈ 19.6504` is **not a measured paired speed ratio**: neither common runner/source, common Q cohort, identical memory budget nor same concurrency has been established. BSGS's 2^20-entry table requires more memory than many smaller-table controls. The testable hypothesis is that reusing a larger table can shift the competitive frontier dramatically; the **next required paired benchmark must include 2^20 BSGS as a separate fully charged arm on the exact same source, runner and public targets as the new IC extractor**, with memory/RSS and precompute explicitly reported.

**Confounder isolation / pre-registered next A/B matrix.** #4532 used the already-existing **Jacobian/mixed-add plus 256-point batch normalization**; #4774 deliberately reused **native affine** arithmetic and a sign-quotient BSGS (`m=32768`, 32,768 x-key baby entries), not the identical method. Thus the historical difference cannot be assigned to baby-table capacity alone. Reuse the two original Rust implementations as independent controls, then (where mathematically and code-path equivalent) measure four configurations under one run: **small/large baby table × affine/Jacobian-batched arithmetic**, with sign equivalence, memory and precompute accounting declared for each. No free copied table, no unseen scalar, and 1/64/1024 target counts. Fix the Q stream across arms; measure field inversions, Jacobian adds, amortized online giant probes, RSS, wall and A/A variance. If a 2^20 affine variant or 2^15 Jacobian variant is not supported without changing semantics, report that cell as absent instead of fabricating a result. At least #4532's original 2^20/Jacobian arm and #4774's original 2^15/affine arm must be run **side-by-side**. Use the fastest valid **memory-matched** arm as the operational baseline before assigning any complete-solver advantage.

This correction was checked against the merged original-curve #4532 source/CI and #4774's source-specific paired arm. It changes the experiment priority, not the demonstrated solver result: prefer completing #4692's nonenumerative extractor and preparing a stronger competitive baseline over further graph-chart-only tuning. Both PKM 2016 (DOI 10.1007/978-3-662-49387-8_1) and Amadori–Pintore–Sala 2018 (DOI 10.1016/j.ffa.2018.01.009) independently establish prior algebraic index-calculus approaches over prime fields; no novel complexity theorem is asserted here.

## Next falsifiable original-curve experiments (in priority order)

1. **Implement public-target-only B8 unknown-index RR9 coefficient extraction in existing #4692** using the remainder-tower or F4, reusing the current Rust field/tower solvers. No generic 4+4 group-sum table, no witness-index leakage, no supplied F coefficients, no planted-only scoring. Run the sealed 46-positive/18-negative B8 corpus. For every input log exact Fp operations, Gröbner degree, matrix dimensions, memory, wall time, reconstruction validity, misses and censoring; independently verify signed EC witnesses. Require all algebraic outputs to reproduce the public target.
2. **First gate:** report per-call distribution, including the worst negative case, not just median positive. Check the conditional 2.742 ms/call optimistic bound **and** the unchanged 1,219-query complete count; if request count differs, recompute the exact cost model before interpreting. For any slow input identify degree explosion, large kernel or support conditioning; do not substitute a generic table to pass.
3. **Complete rank31 / 1,024-log Rust integration** after corpus success: swap only the relation oracle under #4728, freeze identical public targets, show 31 independent equations and 1,024 scalar-verified answers, charge setup + every failed shift + relation collection + linear algebra + final verification. Compare 1/64/1,024 targets against matched optimized index-calculus frontier, signed shared BSGS and strong rho on the *same runner* with A/B/B/A and signed independent replay.
4. **Asymptotic follow-up:** at ordinary prime-order (not deliberately weak) 23/27/31/37-bit prime-field sizes, fit costs for relation extraction, independent-rank yield and full solve. A 31-bit win alone would establish engineering competitiveness at that size, **not** subexponential classical ECDLP.

No new complete non-generic solver is asserted from this study; completion depends on genuine public-only unknown-index extraction and fully charged cost.
