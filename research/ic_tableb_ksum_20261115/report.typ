#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#show table: set text(size: 8.3pt)

= The k = 5 sum table on Table B's own prime curves

*Study:* `research/ic_tableb_ksum_20261115/` (PROTOCOL.md, registered 2026-11-15T03:46:33Z before any evidence run). *Record:* `benchmarks/records/20261115T0349Z-tableb-ksum-l1.json`. *Track:* generic decomposition (bounded by √r; cannot beat rho or BSGS). *Standing:* counted cost at L0, `[pcc:unverified]` until the pcc claim is filed; wall time is not claimed.

== Outcome

- *Lower counted cost on all three Table B prime rows.* Against the strictest of the pair-table incumbents, run in the same session on the same curve, targets and seeds, the `ksum:m=5` pipeline costs 0.69 / 0.67 / 0.60× at 20 bits, 0.60 / 0.57 / 0.50× at 22 bits and 0.50 / 0.48 / 0.43× at 24 bits at w = 0 / 0.1 / 1 GAE per unpriced native operation. Every 95% interval has an upper end below 0.74 (pooled below 0.72), so the gain holds at all three weights, in each replicate session alone, and it exceeds the replicate noise (same arm, different seeds: at most 6.2%).
- *Registered test.* L1 holds on both primary rows (20 and 24 bits); the secondary row (22 bits) agrees. The registered predictions held: R₀ = 0.691 in [0.55, 0.95] at 20 bits and 0.501 in [0.30, 0.60] at 24 bits. L1 is not falsified.
- *Outputs.* No incumbent output changes: the k-sum arm is a new method configuration beside the unchanged incumbents. All 13,824 records verified; all 10,368 replays reproduced on a second host class with the producing binary, 0 problems.
- *Distance remaining.* Matched negation rho is still ahead at every weight on every row: under the §5a band the k = 5 table costs 5.6–6.9× rho and about 6–7× BSGS (`ecbench band --ranks`: rho-neg and bsgs-neg below every IC arm across the band). This distance depends on how uncharged native work is priced (validator hazard H5): the band charges only `table_insert_attempts`, while `rho-neg` carries large uncharged counted work (`cycle_window_word_ops`); in bitop/v1 the ratio is 5.6–5.8×, and pricing all `work_native` at w brings it to 3.0–3.2× at w = 0.1 and 1.4× at w = 1. Rho stays ahead in every reading; the ksum5/incumbent claim is robust to them (bitop/v1 0.48–0.66, all-work stress 0.29–0.59). Table B records the pair-table incumbent at 8.3× rho (20 bits) and 13.5× (24 bits).

#table(columns: 7, align: (left, right, left, right, right, right, left),
  [*curve (log₂ r)*], [*w*], [*strictest incumbent*], [*S incumbent*], [*S ksum5*], [*S rho-neg*], [*ksum5 / incumbent [95%]*],
  [fp20 (20.0)], [0], [inc-tb (N = 51)], [10.29], [7.11], [1.269], [*0.691* [0.677, 0.705]],
  [], [0.1], [inc-sel (N = 64)], [11.77], [7.86], [1.380], [*0.668* [0.656, 0.679]],
  [], [1], [inc-sel (N = 64)], [24.49], [14.59], [2.375], [*0.596* [0.584, 0.608]],
  [fp22 (21.65), secondary], [0], [inc-tb (N = 74)], [11.15], [6.65], [1.172], [0.596 [0.583, 0.611]],
  [], [0.1], [inc-tb (N = 74)], [12.89], [7.39], [1.280], [0.573 [0.559, 0.588]],
  [], [1], [inc-hi (N = 104)], [28.04], [14.07], [2.249], [0.502 [0.491, 0.512]],
  [fp24 (23.38)], [0], [inc-w0 (N = 115)], [12.74], [6.38], [1.087], [*0.501* [0.491, 0.511]],
  [], [0.1], [inc-w0 (N = 115)], [14.78], [7.16], [1.185], [*0.484* [0.474, 0.495]],
  [], [1], [inc-w0 (N = 115)], [33.16], [14.13], [2.063], [*0.426* [0.416, 0.437]],
)

S in ecbench counted GAE per √r, native work priced at w GAE per operation (AGENTS.md §5a). Ratios are the paired total cost of `ksum5` over the incumbent that is cheapest at that weight, pooled over 3 replicate sessions (432 pairs, 144 targets × 3 rounds per curve). The intervals are `ecbench compare` 95% intervals (linearised t; the bootstrap interval is also computed and the larger upper end is the one tested). All numbers: `results.json`, `results.md`, `summary.json`.

#figure(image("l1-ratios.svg", width: 88%),
  caption: [Recorded ratios per row and weight (square: pooled, bar: 95% interval, circles: the three replicate sessions) and distance to matched rho at w = 0. Sessions ECBS1h4d275c069e9c, ECBS1h86c7a9692380, ECBS1hf39caffaa803 (20 bits), ECBS1he39da69fdace, ECBS1h56440e28db18, ECBS1hddbf442f4f18 (22 bits), ECBS1hb3e5ae7597bc, ECBS1h5dea38dc83f5, ECBS1ha01ee561fd12 (24 bits).])

== Against Table B's recorded rows

Table B (frozen 2026-09-22 ledger, `class: accounting`) records the pair-table recipe at S = 10.72 (fp20), 12.77 (fp22) and 13.97 (fp24), IC / rho 8.30× at 20 bits and 13.46× at 24 bits, with the same base sizes as this study's `inc-tb` arm (51, 74, 111 columns). These before-marks come from another harness and pricing pins and are not divided into this session's S; the in-session `inc-tb` S is 10.29, 11.15 and 12.82 at w = 0 (within 4–13% of them), which is the evidence that the incumbent arm reproduces the Table B recipe. The claim is the in-session ratio.

== Method

- *Curves.* The Round-5 ledger's explicit prime curves (`prime_explicit`), resolved by `ecbench plan` to Table B's slugs `icv1-fp20-t1293-3c2999ba`, `icv1-fp22-tm1267-a5acaf6d`, `icv1-fp24-t1577-77336155`.
- *Arms.* `rho-neg` (matched reference), `rho-neg-aa`, `bsgs-neg`; incumbents `inc-tb`, `inc-rec`/`inc-sel`/`inc-w0`/`inc-lo`/`inc-hi` (`mitm:negation_folded=1`); candidates `ksum4`, `ksum5` (`ksum:m=k`, base sizes frozen from `prime_ksum_pooled_20261029`'s train selection on other curves).
- *Replicates and noise.* 3 sessions per curve with different master and target seeds. Within a session every arm shares the algorithm seed, so `rho-neg` and `rho-neg-aa` have identical counts and say nothing; the noise floor is the max/min of one arm's mean S over the three replicates (inc-tb / rho-neg / ksum5 at w = 0): 1.039 / 1.027 / 1.019 (20 bits), 1.052 / 1.019 / 1.012 (22 bits), 1.007 / 1.062 / 1.030 (24 bits).
- *Binary and route.* `ecbench` built `--release --locked`, rustc 1.100.0, from the `git archive` of `origin/main` 7b3c057e1c (sha256 `7d46e5ee…c9cb`; source archive `3d61948e…fa40`), on a RunPod CPU pod (isolab-eks is blocked by the account budget); replay of every session with the same binary on an Intel Xeon container (`replay-certificate.json`).
- *Strictest incumbent.* Chosen on the same data per weight, which can only favour the incumbent. Where it is `inc-sel`, `inc-w0` or `inc-hi`, those are cells the pooled study had tuned, not Table B's recorded base.
- *Independent validation.* `validator/validator-report.json` (status pass, code in `validator/`): all nine ratios recomputed from the raw records with separate code (max relative difference 0), the scalar certificate of all 13,824 records checked, registration, arms, pairing and pricing re-derived; the strictest-incumbent choice is on the same data (it can only raise the ratio; leave-one-replicate-out ratios are within 0.020 of these).
- *Pilot, disclosed.* A 4-target, 1-round local run on the same curves (other seeds) sized the experiment before registration; its values are not used.

== Limits

Counted cost on three prime curves of 20–24 bits (one 22-bit row secondary); generic decomposition, so the method's limit is BSGS and rho stays ahead. The k-sum base sizes were frozen from other curves, and the 22-bit value is an interpolation. The binary is tied to its source by the pod build only (`session.json` has `git_commit` null; the validator did not rebuild). Not claimed: wall time (L0, shared RunPod host), scaling past 24 bits on Table B's curves (the pooled study covers fresh curves to 32 bits), any security statement, any ranking against tables A and C (clock units). The k-sum method itself was first landed in `prime_ksum_pooled_20261029`; what this study adds is its measurement on Table B's own curves against Table B's own base sizes, with a same-session strictest incumbent, replicate noise, a second-host replay and an independent validator.
