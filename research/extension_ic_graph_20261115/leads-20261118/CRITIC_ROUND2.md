# Round-2 red-team review

Verbatim output of the round-2 red-team critic agent (read-only review of commit bf97ee6920). [pcc:unverified]

RED TEAM: graph-IC leads round 2 (research/extension_ic_graph_20261115/leads-20261118)

Reviewed commit bf97ee6920, which equals origin/aburan28/charming-clarke-qfc2gp. The working tree is clean, so everything below is committed. Each lane's headline is labelled [pcc:unverified], and no lane claims a supported result. I record no status changes; that is for the Coordinator and Validator. Spot-checked numbers reproduce, and I found nothing fabricated. The main problems are pre-registration flaws in fresh-scaling and contradictions between lanes about the rho reference.

## Findings, most severe first

**Wrong**
- **F1. fresh-scaling's exponent test (P1/C2) cannot fail on the lane's own model.** In `fresh-scaling/protocol.draft.json`, `decision_rule` reads alpha from a global OLS of ln C on ln l over p=127..2017. Feeding the lane's own `estimate.csv` IC rows through that fit gives alpha_IC = 0.119 (e=2) or 0.151 (e=3), against 0.488 for rho. So "smaller exponent" passes because the descent cost is fixed (about 7.6 s at every p), even when e=3 makes collection scale as l^(1/2), the same as rho. The local slope over the top two sizes is 0.42 at e=3.
  - Lesson L-eda09df2bc11 applies.
  - Cheapest fix, before freeze: decide on the largest per-phase alpha, or on the top-pair local slope, and re-run the `estimate.csv` rows through the amended rule.
- **F2. fresh-scaling would call M1 "complete" when it is not.** G-cov only checks that hooked descent multiplications match the legacy counters within 0.5%. It would pass once the counted patch lands. But complete-accounting's inventory still has null-count gaps on IC (#5, #9–#11, #13) and on rho (#18–#19). §5a still forbids ranking, so M1 would be labelled "complete counted GAE" while it is still a lower bound.
  - Fix: G-cov should require an emitted count for every inventory row.

**Unsupported**
- **F3. The lanes disagree about the rho reference.**
  - complete-accounting's README distance paragraph and scout-graph-ic's LEADS.md §0 call rho plain-b16 "the cheapest rho" and quote a distance of 0.0625–0.0705 against it.
  - rho-fold (`results.txt:89-92`) shows that arm stopped at 0.387×E[steps], probability about 0.11. Independent-validation shows it is the same 249,417,689-step walk as stage41, so the reference reuses one seed.
  - The cheapest registered configuration in expectation is negation b1 at 1.549e14 bitops. The Karatsuba model gives 9.784e13.
  - Recomputed deferred-arm distances: 0.0444–0.0500 against 1.549e14, and 0.0703–0.0792 against 9.784e13. All of these are lower bounds.
  - complete-accounting's 1 GAE (331,250 bitops) is priced from the registered tower that rho-fold says is 1.6x overpriced. If the reference is re-priced, the band changes too.
- **F4. fresh-scaling's arms are not the strongest on either side.**
  - IC side: Run50's quadratic-table arm (`--root-table`) drew the same 3,415-residual descent stream as the deferred graph arm. It costs 1.805e12 against 2.488e12 non-descent bitops, and 15.1 s against 27.8 s process time (`run-study.json` outcomes[1] and [3]). The protocol measures only `ic-graph` and gives no reason for leaving the cheaper arm out.
  - Rho side: there is no Karatsuba arm and no fastest external rho (AGENTS.md §15). Without them T1 is out of reach.
- **F5. descent-walltime ranks arms that complete-accounting says cannot be ranked.** Its Run50 "model only" section says "table is cheapest and grounded most expensive at every w". complete-accounting, on the same data, says §5a forbids any ranking because every IC arm has null-count gaps. The wording should be "lower-bound order, unranked".
- **F6. scout-graph-ic stacks models.**
  - L4 says C1 gives "1.68x under conservative pricing". descent-walltime shows it is not conservative: local wall is 1.26–1.29x and callgrind gives 1.30x.
  - L1×C1 "≈0.69e12" multiplies counted ratios.
  - L3's crossover shift (p≈450–660 to ≈380) combines a counted saving with a wall-time model whose IC side has no run at any p other than 1009.
  - The "New here" novelty claim (the f², f³ terms) was made with research-kb unavailable.
  - The stale item "f4_fp.rs does not compile" contradicts root-build: it was fixed on main by 4219f6fa28.
- **F7. independent-validation's "no number is wrong" is scoped too widely.** It confirms descent-finder's 1.708x/1.622x arithmetic and leaves the "conservative" label standing. complete-accounting and descent-walltime both show the exact linear stage gives 1.679x/1.598x, so the lower-bound pricing overstated the ratio. The verdict should be limited to the arithmetic replay.
- **F8. The p=2017 rung may overflow.**
  - The dry-run gives l = 16,833,537,778,243,251,473 > 2^63.
  - `src/cryptanalysis/residual_walk.rs` `add_mod` computes `let s = a + b;`, and `SHOUP_MAX_MODULUS = 1<<63`.
  - IC has never run at any p other than 1009, and the generator was tested only on p101 and p1009.
  - Lesson L-61e5ea1f9e0d applies.
  - Cheapest test: a planted p2017 pilot built in release with `overflow-checks=true`, plus a grep of the worker closure for unreduced u64 sums mod l.
- **F9. scout-repo-wide B1 cites only the favourable weight.** It quotes tw2l's distance to rho at w=0 only (2.1–2.2x). Table oracles' per-step inserts are priced in the band under the 2026-11-08 rule, so w=0.1 and w=1 may read differently. The evidence (research/ic_tableb_fewcol_20261118) is not in this checkout.
- **F10. complete-accounting overstates two checks.**
  - It upgrades descent-finder's "likely but not separately verified" 0.30% gap to "Explains", citing modr_add 6,828, which is negligible in bitops. The real per-primitive difference (Run50 recorded minus replay) is field_mul +7,517,655, field_add +5,882,496, field_inv +68,900, word +8, hash_probe 0. That fits about 2,200 multiplications and 20 inversions per stream advance, but the lane does not show it.
  - "pipeline==harness on all 3,415" holds only for residuals without a fallback: the JSON has 949,534,086,226 against 945,425,775,576 bitops.

**Unpriced**
- **F11. rho-fold's Karatsuba re-pricing is one-sided and is not measured.** `gaudry_cubic::E3::mul` is documented as "Schoolbook product … (9 + 2 multiplications)", so IC's extension arithmetic stays schoolbook. The 1.58x is an op-count model, not a measured walk.

**Process**
- **F12. Missing records.**
  - There is no `bench.record/v1` for descent-walltime's 48 timed runs or the callgrind runs.
  - No round-2 run has a research-visuals report or PDF.
  - No `problems/` record exists. The ledger merge-drops repeat problems/pkm-pcc-ledger-divergence.md and problems/native-ecbench-pcc-ledger-fork.md, which the lane does not link. LOCAL custody and the §5a null gaps also need records.
  - independent-validation proposes re-appending the dropped ledger entries. Doing that now cannot restore pre-data anchoring and must be labelled post-hoc. The trace does not attribute feaf31's drop to any merge.
- **F13. Minor.** descent-walltime always ran production before traced, so the A/B is not counterbalanced. The linear-stage ratio of 1.009 [0.994, 1.027] partly controls for order. The scout's "idle pod" status (item E) comes from the 2026-11-17 log and was not re-checked today.

## Edits outside the leads directory (base ad2a014c00 to HEAD)
- **Root-build, disclosed:**
  - `src/cryptanalysis/ic_framework/plugins.rs` is allowed. I checked that the copy kept is a strict superset of the removed one (it adds a `trace_kernel_basis` is_err assert).
  - `tests/ml_dataset_live_capture.rs` and `examples/jv_sieve_quadratic_pair_relation_controls.rs` are outside `src/` and need owner confirmation.
- **Orchestrator:**
  - `docs/ACTIVE.md` (commit ed408fdca9) marks the round-1 line [x] with "2 of 7 lanes landed", which ticks unfinished work. Despite the commit message, it adds no round-2 line.
  - `docs/lessons/entries/L-60d4cdb72eac.json`, `L-d9655601a238.json` and `O-d1d4893b2a28.json` (577a73ee4e) are round-1 lessons in the right place.
- **Local only:** root-build deleted `debug/incremental` in the shared /home/user/cargo-target-round2.

## Remote side effects
None from the round-2 lanes. I found only reads: `gh api`/search by root-build and scout-repo-wide, and an ecdb GET that returned 403. The only pushes are the orchestrator's WIP checkpoints to PR #4654.

## Numbers reproduced
- complete-accounting:
  - deferred total R+U+L = 6,873,300,847,732
  - band 2,647,504 × 331,250 = 876,985,700,000
  - L = 945,425,775,576 − 2,513,689,650
- rho-fold:
  - E[iter] 4.579e8 and Rayleigh P = 0.111
  - expected walls 131.4 / 100.5 / 95.4 / 76.3 s
- descent-walltime:
  - geomeans 1.2852 / 1.2474 / 1.2633 / 1.2731, recomputed from the 48 per-run files
  - non-descent 1.805 / 2.488 / 2.791 / 2.810 e12, and the deferred-vs-original flip from +287e9 to −586e9
- scout-graph-ic: split ratios 15/45/15 and m(0.079) = 2.47.
- scout-repo-wide: the malformed benchrec record is confirmed by `benchrec check`.
- independent-validation: all seven claim ids are absent from `pcc/ledger.jsonl` at HEAD.

## Leads ranked by expected value per cost
1. **[owner]** Item E: check the RunPod pod list once. If replead-r1-20261116 still exists, copy `/work/out` and terminate it. Spend may be running and data may be lost.
2. **Local, about 1 min.** L1 steps 1 and 2: tally S factorization patterns on the seed 389 and 401 streams, and count the class universe exactly. This decides the largest graph lead.
3. **Zero compute, before freeze.** Amend fresh-scaling for F1, F2, F4 and F8:
   - per-phase or local-slope decision rule
   - add the quadratic-table arm and a Karatsuba rho arm
   - independent rho seeds
   - overflow audit for p2017
   - G-cov defined as complete counts
4. **Local.** fresh-scaling planted pilot, one target per size. This gives the first IC data at p≠1009; build p2017 with overflow checks.
5. **Local, seconds.** L5 per-kernel split before any work on C4.
6. **Local.**
   - Opt-in Karatsuba Arith in jv_rho_strong, with per-step counts checked on a short walk.
   - A Karatsuba tower for IC's own E3/Fq3 arithmetic (gaudry_cubic, jv_cover). This is my new lead N1; it keeps the pricing symmetric and could be a T0 gain in the relation phase.
   - Changing the registered reference needs the owner.
7. **Owner or record owner, minutes.** A1: fix or pin-exclude 20261117T0334Z-hec-multi-production-cold.json; this unblocks 16 records.
8. **Local, plus owner decision on the convention.** Land the counted zero_dim_la patch and replay the original-IC stream, replacing the 477.9e9 estimate.
9. **Zero cost.** Report rho as expected cost of the cheapest registered configuration, with an interval, and stop calling plain-b16 "cheapest". This is my new lead N3.
10. **Local, conditional on 2.** L1 step 3, then L3 and L2.
11. **Local.** descent-walltime replay that skips symbolic preprocessing; allocation reuse in f4_fp (report as wall time only).
12. **Local, medium effort.** Emit counts for the null gaps (graph containers, RREF scans, PRNG, rho's three). This is a prerequisite for any §5a ranking.
13. **Local CPU hours.** B1/C1 tw2l stack, with the band at w = 0.1 and 1 required.
14. **[owner]** A2: rotate the AWS keys and checkpoint token. This unblocks every isolab, S3 and validation path.
15. **[owner]** Push the LOCAL archives for nagao-e2e and Run50; add a relation checker with factor-base points in the rows; ledger fix with post-hoc labels.
16. **[paid + owner]**
    - Isolated EKS A/B for C1 wall time.
    - fresh-scaling measured run, about 55 CPU-h.
    - rho-fold run with N≥20 seeds (only if wall time is needed).
17. **Lower priority.** B2, B3/C3, C2, C4, C5, D, A3.

## Still missing
- Any VALIDATION.json, replay certificate or validator report.
- A fastest-external rho for E(F_p^6) (§15).
- Isolated timing.
- IC data at any size other than the exposed p1009 target, and fresh public targets.
- pcc registration: the seven ledger entries (feaf31, 6432aa, e9f39d, 7defbb, 6224b9, 853115, 0ecee3) are absent at HEAD, and no round-2 claim is registered.
- The LOCAL raw archives.
- research-kb and lead-atlas checks; both were unavailable in every lane.
- Bench records, visual reports and problem records (F12).
- A round-2 line in docs/ACTIVE.md.

## Lessons
To keep the repository read-only, I wrote these to a scratch copy of the ledger. The orchestrator should copy them into `docs/lessons/entries/`. They are all in `/tmp/claude-0/-home-user/53beab17-35e6-5dd9-bf91-c8753a71ee81/scratchpad/lessons/entries/`:
- New lessons:
  - L-2ff461066555: check N=1 rho draws, seed reuse and GAE tower before quoting distance to rho.
  - L-7cdab2147415: compare every recorded IC arm on matched descent before choosing a protocol's candidate arm.
- Tests, all confirm:
  - T-bebb62e2506d for L-eda09df2bc11, which is now adopted.
  - T-f6b9387e443c for L-61e5ea1f9e0d.
  - T-3dc5d0f0e294 for L-d9655601a238.
  - T-9856a919c988 for L-60d4cdb72eac.
- Outcome: O-2ec2e8bbda45.
