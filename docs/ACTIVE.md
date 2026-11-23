# Active goal and items

Owner-pinned work queue. `tools/goal_brief.py` prints this file at the start
of every Claude and Codex session (SessionStart hooks) and for `/goal`
(AGENTS.md, "Goal: what to work on next"). When this file has no
unchecked item, sessions fall back to the newest "Open threads" in
`docs/session-logs/`.

Keep it short: one goal, a few ordered items. Sessions check items off and add
follow-ups here; history belongs in the session log, not in this file.

## Current goal

Find an algorithmic breakthrough for the DLP in Jacobians of genus g >= 2 hyperelliptic curves over prime fields: a measured, fully charged improvement in exponent or constant over the lab's best index calculus, and over matched rho where IC already wins. Targets: genus 3 prime-order Jacobians (exponent 1 via a transfer to a plane quartic and Diem's index calculus, against 4/3 for double large prime and 3/2 for rho), genus 2 (no known IC beats rho's exponent, so constants and structure), and the genus-g exponent ladder. (Owner, 2026-11-20. Previous goal, kept for reference: subexponential IC for ordinary elliptic curves over prime fields.)

## Active items (top = next)
- [ ] DiSSECT hyperelliptic-transfer trait family for prime-field curves (OpenCode, started 2026-11-22, branch feat/curve-traits-hyperelliptic-transfer-20261122, PR #4932; validated with build/test/check at 58356b85 and traits.json regenerated; CI blocked on the base-tree break at 52e986e114)
- [x] Prime31 certify x-only [16] Kummer permutation on curve/twist, eliminate phantom chain branching (automation, started 2026-11-19, branch automation/prime31-kummer-doubling-twist-20261119, PR #4810; source-specific Rust CI passed; extractor/speedup still open)
- [ ] Audit and benchmark public-x Semaev F3 batch-root two-summand extraction on original Prime31 Rust curve; compare generic affine recovery and freeze negative controls (automation, started 2026-11-19, branch automation/prime31-f3-batch-root-gate-20261119, PR #4809)
- [x] Prime31 block-scale exponent k=4..8 W8 public relation coverage and exact repeated-half cost sweep (automation, completed 2026-11-19, branch automation/prime31-blockscale-sweep-20261119, PR #4808; 40+4 Rust tests, 20 affine checks, 512Q×5 arm original-source CI passed)
- [x] Prime31 W8 held-out signed-weight collision, 1,024-target yield and second-cohort exact four-arm full-fibre multiplicity controls (automation, 2026-11-19, branch automation/prime31-w8-collision-certificate-heldout-20261119, PR #4807; 40+3 original Rust tests, 16 independent affine witness replays, CI #44533134056 PASS; genuine public-only RR9/F4 extraction remains owned by open PR #4692)
- [x] Prime31 B8 weighted-slot formal-collision audit and matched public-hit grader (automation, started 2026-11-19, branch automation/prime31-w8-weight-signature-audit-20261119, PR #4805)
- [x] Prime31: exact signed cancellation coverage and unweighted/weighted B8 RR9 handoff (automation, started 2026-11-19, branch automation/prime31-sign-cancellation-coverage-20261119, PR #4803; 40+2 source-specific CI PASS, signed PCC pending)
- [x] Attack the open problems imported from `math-street/math-st` (knowledge `imports/math-st/`, 16 problems and 23 questions open): triage, then bounded falsifiable attacks in ranked order (Claude Code, started 2026-11-19, branch claude/tender-cori-tzs90j, PR #4791; triage plus A1–A3 and P3.3 done, all at kill; P1.6/Q018 and Q099 left as attack-later)
- [x] Test RR9 Grassmann four-plane 4x4 graph-chart determinant and benchmark against exterior4 on original Prime31 (automation, completed 2026-11-19, branch automation/prime31-rr9-grassmann-chart-20261119, PR #4780; exact-source paired CI #44452737502 PASSED, independent Python exact residues/truth PASS; public-only B8 nonenumerative extractor still open with #4692; signed PCC pending)
- [x] Prime31 exterior-4 public-only Plücker-span compression gate: analytic exterior-span proof and 32 one-slot certificates plus owner-waived local producer/checker report 70/70/70 in N4/P4/N8/P8; fixed linear compression closed only, raw attempts/report retained, source-bound CI and signed PCC absent, SAT/F4/XL separate (automation, completed 2026-11-19, branch automation/prime31-exterior4-plucker-span-20261119, PR #4779) `[pcc:unverified]`
- [x] Prime31 original-vs-superincreasing W8 signed-weight full-cost paired Rust screen (automation, started 2026-11-18, branch automation/prime31-w8-ternary-weight-screen-20261118; PR #4770; same-runner source-specific 1.262x generic-grader result, no non-generic claim)
- [x] Prepare a native bridge for transported endpoint ecbench workloads, scalar-input manifests, coordinate/A-A controls and explicit target-field compatibility guards; local correctness/planning component complete, large-field harness and measured sessions remain open (Codex, started 2026-11-18, branch research/conductor-paired-driver-20261118, PR #4721 stacked on #4703)
- [x] Certify Prime31 RR9 two-sign weighted-interpolation rank obstruction on the original curve (automation, started 2026-11-19, branch automation/prime31-rr9-two-sign-wrank-20261119, PR #4801; source-exact native Rust 40/40 + 2/2 PASS Actions #44497509496; independent signed PCC remains pending)
- [x] Derive Prime31 RR9 public-only extractor budgets and correct historical optimized shared-BSGS frontier (automation, started 2026-11-19, branch automation/prime31-rr9-extractor-budget-gate-20261119, PR #4793; original-Rust source-bound CI PASS #44489047210; next unknown-index extraction remains with #4692)

- [ ] Reproduce the genus 2–4 HEC index calculus comparison and examine the mathematical contribution (Codex, started 2026-11-18, branch research/hec-tenfold-replay, PR #4789)
- [x] Publish the fresh P-192 plane-quartic transfer acceptance replay, machine receipt and exact remaining obligations (Codex, completed 2026-11-19, branch codex/p192-quartic-transfer-assessment-20261119, PR #4788)

- [ ] Graph-anchored multi-pattern quadratic descent, certified F4 trace replay, and linear-stage accounting for extension-field graph IC (Cursor, started 2026-11-18, branch cursor/graph-index-calculus-ecdlp-7226)

- [x] Keep the n83_d18_shared_bits include_bytes! CSV in Git (store policy keep entry) so CI `check --all-targets` can compile it (Claude, started 2026-11-19, branch aburan28/zealous-lovelace-tc390j-keep-n83-csv, PR #4786)

- [x] Isogeny P0 closed without code: sqrt-Velu is already 1.6-5.6x faster than Velu from l~150-400 (isogeny_algos/docs/large-ell-20261016), chain callers use l=2,3 and sqrt-Velu in CSIDH-512 was a measured null, so a dispatcher has no consumer; trees lose even with the IFMA NTT; verdicts for all isogeny leads in isogeny-labs/workspace docs/isogeny-speedup-assessment.md (claude, 2026-11-19, branch claude/isogeny-sqrt-velu-p0, PR #4784)
- [x] Restore `cargo check --all-targets` on main: port ic bench/progress/experiment bins, ic_quartic_core_probe example and ps6_tool to the current APIs (Claude, started 2026-11-19, branch aburan28/zealous-lovelace-tc390j, PR #4778)

- [x] Original-curve Prime31 public-only exterior-4 RR9 algebraic candidate, B4 correctness/cost falsifier and B8 scaling gate (automation, completed 2026-11-19, branch automation/prime31-wedge4-public-algebraic-20261119, PR #4776; exact-source CI #44445676750 passed, exhaustive B8 direct-search gate negative; nonenumerative extractor remains with PR #4692)
- [x] Prime31: evaluate signed one/two-prefix RR9 chart reduction without generic sum tables, Rust exact-source algebra and cost gate (automation, started 2026-11-19, branch automation/prime31-rr9-prefix2-cost-20261119, PR #4773)
- [ ] Prime31 public RR9 factor-resultant/Jacobian gate and scaling falsifier (automation, 2026-11-19, PR #4772; prospective protocol recorded, original native Rust + independent dual replay CI pending)

- [ ] Speed up prime-field ECDLP index calculus across counted cost `S(w)` (`w in [0, 1]`), `bitop/v1` gate counts, and cold single-target solve wall time (Cursor, started 2026-11-18, branch cursor/prime-ecdlp-ic-speedup-809a)

- [x] Test Frobenius cross-bilinear x-coordinate sum-bucket filter on Koblitz toy fields (automation, completed 2026-11-18, branch research/frob-cross-bilinear-sumfilter-20261118, PR #4768; toy-negative, independently unverified)
- [x] Evaluate round-robin balanced vs random partial pair-table prefixes on toy ECDLP, freeze falsifier, independent audit (automation, started 2026-11-18, branch research/ic-pair-schedule-r3-20261118, PR #4765; falsified, no leaderboard promotion)
- [x] Preregister D4/S5 coefficient-span root-lattice falsifier and complete S3 planted-root calibration (automation, completed 2026-11-18, branch research/d4-s5-coefficient-span-falsifier-20261118, PR #4764; full D4 lattice trial remains queued)
- [x] Preregister and test sparse partial pair-table construction on public toy ECDLP (automation, 2026-11-18, branch research/ic-partial-pairtable-20261118-run2, PR #4763; 24 B32+8 B16 correct, 160 local arm replays, [pcc:unverified]; native follow-up queued)

- [x] Speed up ECDLP index calculus: binary x-only sibling pairs (`BinaryGroup::sibling_pair`) and necklace-reduced `ksum-frobenius` with Montgomery leaf batches, target-free setup collision harvesting, and scale-free `scale=X` sizing — up to 11.84x counted speedup across the §5a band (14.75x wall) on 44.3-bit `icv1-f2m53-tm56619371-dac20a85` and 6.09x (7.14x wall) on 39-bit `icv1-f2m41-tm2308219-7f48b14a` (`research/koblitz_frob_ksum_sib_20261118`, Cursor, completed 2026-11-18, branch cursor/ecdlp-ic-speedup-809a, PR #4762) [pcc:unverified]

- [x] Test genus-2 smooth-companion torsion sieve against matched ECDLP coverage nulls (automation, completed 2026-11-18, branch research/genus2-companion-torsion-sieve-20261118, PR #4761; toy predicate rejected)

- [x] Implement and test a norm-fiber Riemann--Roch router for the n=83 image-compatible calculus, beginning with an exact n=9 gate (Codex, completed 2026-11-18, branch codex/n83-norm-fiber-rr-router-20261118, PR #4759; registered finite-crossover gate stopped n=13+)
- [x] Derive and screen a materially higher-degree image-compatible Riemann--Roch family, requiring a finite crossover before measurement (Codex, completed 2026-11-19, branch codex/n83-norm-fiber-rr-router-20261118, PR #4759; no route passed the frozen analytic gate)

- [x] Execute and evidence the GHS strong-rho optimization registry R0-R99 in parallel, with composed-baseline promotion gates (Codex, completed 2026-11-19, branch codex/ghs-rho-parallel-20261118, PR #4757)

- [x] Specify the GHS strong-rho implementation and matched frontier-measurement program (Codex, completed 2026-11-19, branch codex/ghs-rho-parallel-20261118, PR #4757)

- [ ] Run the frozen EKS timing study research/ic_inrun_caching_timing_20261119 (OLD 104ca79 vs NEW ab42aff, 8 jobs, USD 15 ceiling). BLOCKED: isolab-experiment.yml fails in its contracts job on research/* artifact hydration (run 44577926579; also 44374607134 on main) (Claude Code, started 2026-11-19, branch aburan28/confident-sagan-cudszh, PR #4756)
- [x] Fix the default lanczos deferred arm accepting undetermined solutions, and clear pre-existing compile/clippy errors (ml_dataset_live_capture, genus_ladder_fastgroup, ml_dataset drop_non_drop) (Claude Code, started 2026-11-19, branch aburan28/confident-sagan-cudszh, PR #4756)
- [x] IC caching round 2: ecbench factor-base log store (amortised rows only), method-aware is_reusable_phase, petit_quisquater x-index hoist, opt-in F4 trace replay, opt-in sparse linear-algebra attempt schedule (Claude Code, started 2026-11-19, branch aburan28/confident-sagan-cudszh, PR #4756)
- [x] Remove repeated target-independent work inside one cold IC run: MITM left-half table, linear-algebra restarts, sweep x-index hoist, Weil-descent template, batch inversion in prime mitm/subtract, shared F4 symbolic preprocessing across same-shape systems (Claude Code, started 2026-11-18, branch aburan28/confident-sagan-cudszh, PR #4756)
- [ ] Add one-large-prime partial-relation pairing to the projected genus-3 cover solver, retaining exact transfer certificates, generic controls, and complete phase accounting (Codex, started 2026-11-20, branch codex/ecbench-canonical-panel-20261118, PR #4752)

- [x] Implement a specialized genus-3 destination solver for the canonical Phase 1 cover-transfer route, retaining generic rho/BSGS controls and complete transfer accounting (Codex, completed 2026-11-20, branch codex/ecbench-canonical-panel-20261118, PR #4752)

- [x] Implement the first phase-matched end-to-end Phase 1 hyperelliptic/Jacobian and cover/transfer benchmark adapters in the canonical phased ecbench panel (Codex, completed 2026-11-19, branch codex/ecbench-canonical-panel-20261118, PR #4752; typed lane infrastructure completed 2026-11-19)

- [x] Repair main's `check` and `clippy (crypto)` jobs beyond #4691: duplicate `main` in the jv_sieve controls examples and clippy -D warnings in eight examples (claude, started 2026-11-18, branch claude/main-check-clippy-examples-20261118, PR #4751) — done: with #4691, check/clippy commands leave only the store-managed n83_d18 CSV error (materialized in CI)
- [x] Repair second-pass audit findings: restore the pcc gate workflow and re-land the headline tolerance cap (the claim types, test_pcc call and lessons entries were already handled on main) (claude, completed 2026-11-18, branch claude/codebase-assessment-report-fp044d, PR #4750)

- [ ] Next IC levers (owner: a–c): (a) decomposition-based single-target IC vs batched rho, (b) multi-target IC vs rho on a second host, (c) Montgomery-form field arithmetic for every arm of mt_fieldops (claude, started 2026-11-20, branch aburan28/youthful-hopper-hfrwkd, PR #4747)

- [x] Single-target (T = 1) baseline: the multi-target IC instrument's single-target IC against batched single-target rho at 2^32, 2^36, 2^40 (claude, started 2026-11-19, branch aburan28/youthful-hopper-hfrwkd, PR #4747): measured 2026-11-19, IC/rho 3.8x / 5.0x / 6.9x, gap grows; bench record close awaits main's malformed hec record

- [x] Cut the multi-target IC's table-build cost at 2^36 (batched in-order inserts, zero-encoded empty slots; measured against tuned rho) (claude, started 2026-11-19, branch aburan28/youthful-hopper-hfrwkd, PR #4747): measured 2026-11-19, IC 0.90–0.92x at 2^36; distance to rho host-dependent; bench record close awaits main's malformed hec record

- [x] Land round-7 prime-field IC closure results (PKM m8-10, cost-rule correction, F4 degree bound, BKK, lifting heights) (Claude, completed 2026-11-18, branch claude/prime-field-techniques-89ma6o)

- [x] Reproduce the Kaarelson v2 Theorem 3 coverage bound engine-free: native exact measurement of mean 2-sum coverage over dilations and additive energy E_2/X^2 at p in {1009,4099,10007}, m=2, on prime-order curves (A!=0 and A=0), plus the plain-box u=1 value; compare to the paper's reported 0.39-0.42 and ~4. E_2/X^2=3.86-3.89 reproduces ~4; mean coverage 0.367-0.385 sits just below the band; cross-checked by an independent implementation. Report in research/kaarelson_coverage_20261118 (claude, started 2026-11-18, branch claude/ecdlp-sub-sqrt-algorithm-rp5cvm, PR #4745) [pcc:unverified]
- [x] Kaarelson coverage follow-up: average coverage over 16 prime-order curves per prime (coverage_multi.cpp, folded into PR #4745). Mean coverage reaches 0.389-0.390 at p>=4099 (paper's 0.39-0.42 lower edge), E_2/X^2=3.98-4.05 (paper's ~4), and cov(u=1) approaches the over-dilation mean as p grows; single-curve values were finite-sample low. First curve reproduces coverage_t3 to all digits
- [x] Audit the Kaarelson Lean engine claim, statement level: read the source at kaarelkaarelson/fast-ecdlp@d1deb5ab; the Thm 2(ii) oracle statement `subexponentialRate` is faithful and non-vacuous (M bound to compute lexLeast m; curve membership type-enforced; time 2^o(l)=p^o(1)). Report in docs/leads/2026-11-19/fast-ecdlp-lean-audit (folded into PR #4745)
- [ ] Kaarelson Lean audit, remaining (needs Lean/Mathlib v4.35.0 + ~10 GB disk): run `lake build`+`Verify.lean` (one sorry, clean axioms), run the Comparator (comparator.json + nanoda) for Challenge<->Solution identity, confirm the genericity condition equals Definition 3.1, and independently review the vendored Rudich-Rousseau engine (lean/vendor/rudich-factoring/); factor-base set is x<ceil(p^1/m) not [1,floor(p^1/m)]
- [ ] Speed up the Koblitz index calculus online phase (per-target descent: target_pdp and target_query) with frozen same-session A/Bs, after the set-up levers (#4292, #4299, #4316, #4360) (claude, started 2026-11-18, branch aburan28/laughing-darwin-3qdnk2)
- [x] All three after the res2 finding (res2 = rho in disguise): closure note (k-sum family is the generic incumbent, bounded by sqrt(r)); m=2 pair-table constants: no improvement; generic binary k-sum incumbent 5.1-7.0x rho, free rows no improvement; binary ksum ecbench dispatch restored (Claude, 2026-11-19, PR #4743)
- [x] Parallel IC lanes (Sonnet workflow; multi-target and weak families excluded by owner): (1) E(F_{p^3}) Gaudry IC 605-2,501x rho, fitted crossover 2^171 or none; (3) static variable relabelling in F4 exact but 1.4-35.6x slower. Lane 2 (GHS weak families) withdrawn: out of scope (Claude, 2026-11-19, PR #4743)
- [x] Step-back on pruning/filtering in IC: (A) factor-base size sweep confirms GAE = |F|^2 + N/(4|F|), optimum N^{1/3}/2, distance to rho 14.6x/24.4x (p24/p28); (B) filter economics: every binary-curve prefilter already has a negative or null verdict (Claude, 2026-11-19, research/ic_fb_size_model_20261119, PR #4743)
- [x] Repair main's last non-compiling targets and restore the lost pipeline input checks (duplicate items from overlapping repairs removed; r/cofactor and factor-base index checks from f73b2edcc7 restored; rust_extension_security 7/7) (Claude, 2026-11-18, PR #4743)

- [x] Koblitz matched rho reference: tally v5's jump-index selection (owner decision 2026-11-20: matched reference min(strong, v5) below 2^38), re-measure K1 n17-n37 references and the gen-orbit IC distance (claude, 2026-11-20, branch claude/tooling-build-alq44y, PR #4742; done: tallied v5 still the matched reference at log2 r 15-28, n37 tied with strong at w=1; adoption in docs/ic and boundary_targets.json is the follow-up; bench record left open, close blocked by a malformed main-side record)
- [x] EXP-PKM-IC-001: Kummer-coset factor base in the prime-field pair-sum index calculus vs small-x and random factor bases, with the distance to rho (claude, started 2026-11-19, branch claude/tooling-build-alq44y, PR #4742)
- [x] EXP-PKM-RESULTANT-001 Amendment 1: amortise the V-only work (product tree, reciprocals; enumeration's lifting) over targets (claude, started 2026-11-19, branch claude/tooling-build-alq44y, PR #4742)
- [x] EXP-PKM-RESULTANT-001: quasi-linear resultant / multipoint-evaluation decomposition oracle at m = 2 vs enumeration and the F4 dense oracle, S up to 2^16 (claude, started 2026-11-19, branch claude/tooling-build-alq44y, PR #4742)
- [x] RunPod resilient tooling: pod stops itself after its job completes, so a lost controller cannot leave a finished pod billing (claude, started 2026-11-18, branch claude/tooling-build-alq44y, PR #4742)
- [x] Graph-IC leads round 2: accounting, fresh/scaling protocol, rho fold, independent validation, root build, descent wall-time check, new-lead scouting (claude, started 2026-11-18, branch aburan28/charming-clarke-qfc2gp, PR #4741; results in research/extension_ic_graph_20261115/leads-20261118/, owner decisions listed in its SUMMARY.md)

- [x] m = 3 rootfind-subtract: sweep a rotating fraction of the base per trial (lead 1 of research/ic_m3_cost_leads_20261115), counted-S study at 37.4/39.3 bits: S 0.36–0.55x of the full sweep at 1/16 (w = 0), validated (research/ic_m3_sweep_window_20261118; claude, 2026-11-18, PR #4739)

- [ ] Round-5 Table B leads: L5-BINSG (stride-generator targets on binary), L5-BINSIB (x-only siblings on binary curves, code), L5-LB (leaderboard component for rounds 2-4) (Claude, started 2026-11-20, branch aburan28/inspiring-dijkstra-ay05qi, PR #4738)
- [x] Round-4 Table B leads (Sonnet agents): JUMPS no change; C1SIB 0.79-0.89x rho at w = 0 (C = 1 ruling pending); GATE rounds 2-3 evidence valid; SCALE flat ~0.9x rho at 28-32 bits; BINWALK ww-tw 0.33-0.41x of the binary k-sum incumbent, 2.2-2.3x rho [pcc:unverified] (Claude, completed 2026-11-19, branch aburan28/inspiring-dijkstra-ay05qi, PR #4738)
- [x] Round-3 IC lead on Table B: sib8-p256 0.44-0.48x of tw2l at w = 0 and below it at every weight; 0.89-1.11x rho at w = 0; ablation: stride-generator targets carry the w = 1 gain, siblings the w = 0 gain [pcc:unverified] (Claude, completed 2026-11-19, branch aburan28/inspiring-dijkstra-ay05qi, PR #4738)
- [x] Chase new single-target IC leads after P2 KSUM-SIB: L2-HRATE closed; L2-STACK 0.89-0.92x and L2-FEWCOL (tw2l) 0.41-0.48x of the Table B incumbent, 2.1x rho [pcc:unverified] (Claude, completed 2026-11-18, branch aburan28/inspiring-dijkstra-ay05qi, PR #4738)
- [x] LFD paper runs (records, engine extract and Rust lift moved to lfd-summation-systems PR #37); this PR keeps the hky3 engine changes, the encoding control and the binary d_F ladder (Claude, started 2026-11-18, branch claude/lfd-ffd-paper-eprint-status-qq8fit, PR #4735)

- [ ] Prime31 weighted-slot RR9 resultant encoding, exact Rust/independent controls and full-cost extraction gate (automation, started 2026-11-18, branch automation/prime31-rr9-weighted-resultant-20261118, PR #4728)
- [ ] Daily public-source research PR sweep refresh (automation, started 2026-11-19, branch codex/daily-research-leads-20261118, PR #4724; preserve 2026-11-18 frozen evidence; no paid compute or merge)
- [x] Exact universal formal-collision quotient for weighted Prime31 M6; bound gains from weight retuning (automation, completed 2026-11-18, branch automation/prime31-m6-throughput-ceiling-20261118, PR #4722; source-specific original Rust 40+2 tests and independent BigInt replay passed, signed PCC pending)
- [x] Prime31 M6 exact uniform-query throughput ceiling and non-generic cost-gate audit (automation, completed 2026-11-18, branch automation/prime31-m6-throughput-ceiling-20261118, PR #4722; pinned Rust 40+2 tests and separate Python exact replay PASS, signed PCC pending)
- [x] Prime31 signed pair Semaev projector: exact denominator-safe polynomial membership, bounded Rust test, and B31 cost gate (automation, started 2026-11-18, branch automation/prime31-algebraic-pair-projector-20261118, PR #4720; source-specific Rust 42/42 and independent 715-case BigInt replay passed; no full-solver speed claim)
- [x] Specify and test a compact growing-degree or adaptive target-local router for the n=83 image-compatible relation calculus (Codex, completed 2026-11-18, branch codex/n83-adaptive-image-router-20261118, PR #4719)

- [ ] Audit and implement RR9 low-power multiplicity-strata exact gate on original Prime31 (automation, started 2026-11-18, branch automation/prime31-rr9-low-power-strata-20261118)
- [x] Prime31 independent-slot masks: exact parity and cost controls on original curve, compare prior weighted slots (automation, started 2026-11-18, branch automation/prime31-independent-slot-mask-20261118, PR #4716)
- [x] Compare complete Prime31 weighted M6 XOR-parity with exact any-witness/256-chunk Rust oracle: rank 31/31 and 1,024/1,024 source-curve logs for both; matched 1.43998–1.44093× within-M6 improvement, 87 versus 106 training queries; no non-generic/frontier win, signed validation pending [pcc:unverified] (automation, 2026-11-18, branch automation/prime31-weighted-slot-parity-20261118, PR #4714)

- [x] Match original-curve B31 weighted-M6 complete 1,024-Q Rust solver to strongest shared-BSGS and native mixed 4+4 IC: source-bound 2-run ABBA complete, 1,024/1,024 each; M6 20.85–23.32× slower than BSGS and 1.77–1.88× slower than native IC; no non-generic win; reports/raw evidence archived [pcc:unverified] (automation, started and completed 2026-11-18, branch automation/prime31-weighted-slot-parity-20261118, PR #4714)
- [x] Prime31 weighted-slot parity gate on unchanged p=2^31−1 curve: uniform-slot/Frobenius cancellation proved; distinct-weight positive-parity test B8/B16/B31 measured; pinned original Rust 40/40 + independent Python replay passed (#4714, exact-source CI #44278553811). This is a generic-MITM parity feasibility control, not a cheaper full solver; next nonenumerative extractor remains open.
- [x] Extend PR #4714 with an original-curve weighted six-slot 3+3 Rust index-calculus complete-solve control, same 2^31−1 curve, 1,024 public targets, rank/cost and prior-lead audit; 31/31 rank, 1,024/1,024 target recovery, 7.368x matched scalar-vs-batched whole-program gain (generic-equivalent; unsigned; #44295520583), (automation, started 2026-11-18, branch automation/prime31-weighted-slot-parity-20261118, PR #4714)
- [x] Prove or refute a uniform exponential-rank lower bound for the hashed prime-field semantic decomposition matrix (Codex, completed 2026-11-18: all-seed and nonnegligible-probability lower bounds refuted; exceptional selected seeds and the low-level BTNIR family remain open, branch codex/extend-ecdlp-obligation-graph-20261118, PR #4704)

- [x] Add optional bounded projective walks, retain the exact 80-bit window audit, and record small-pipeline plus target-field coordinate controls without target timing claims (Codex, completed 2026-11-18, branch research/conductor-window-audit-20261118, PR #4703)
- [x] Fraction-free batched Prime31 Frobenius-cubic inverse, same original-curve Rust 256-point control (automation, started 2026-11-18, branch automation/prime31-frobenius-cubic-inverse-20261118, PR #4701; exact-source CI #44267376366 passed 40/40 + 2/2 + independent 256-target pseudo-gcd, measured 1.223x stage-only; RR9 complete IC gate remains open)
- [x] Prime31 nonenumerative Frobenius-cubic inverse-Vélu certificate and Rust gate (automation, started 2026-11-18, branch automation/prime31-frobenius-cubic-inverse-20261118, PR #4701; exact-source 40/40 + 2/2, independent 256-target replay; full IC advantage remains open)

- [x] Prove and audit the complete prime-field ECDLP bound from graded parity counting through order-free recovery (Codex, completed 2026-11-18, branch codex/ecdlp-end-to-end-20261118, PR #4698)
- [ ] Establish read-only IsoLab budget telemetry through the existing GitHub identity before conductor-worker cloud execution (Codex, started 2026-11-18, branch ops/conductor-budget-preflight-20261118, PR #4695)
- [ ] Freeze and validate the separate exact GitHub-runner norm-branch protocol and workflow; dispatch only after merge in a fresh result PR (Codex, started 2026-11-18, branch codex/norm-branch-runner-freeze-20261118, PR #4694)

- [x] Repair main (follow-up): remove the duplicate test the_trace_coset_base... so the lib-test target compiles (claude, started 2026-11-18, branch claude/repair-main-dup-test) — done: lib-test target compiles (PR #4691)
- [x] Include and hash the projective-walk helper in the native worker build; keep historical image qualification separate (Codex, resumed 2026-11-18, branch research/conductor-worker-20261118, PR #4690)
- [x] Locally qualify the native conductor evaluator worker, fix its runtime dependency, and prepare a supplied 47-bit control without changing the minimum 80-bit goal (Codex, started 2026-11-18, branch research/conductor-worker-20261118, PR #4690)
- [x] Chain-system solving degree, follow-up to PR #4278: attack the open linear lower bound (cofactor reduction, Lean) and the depth-15 upper certificate (Claude, started 2026-11-21, branch claude/tender-einstein-n6cp5x, PR #4898)
- [x] Chain-system solving degree, shift-stable variant of Theorem A: measure shift-stable top-residue avoidance (all low-weight multiples and reachable closure) and prove the weakened Theorem A in Lean (Claude, started 2026-11-22, branch claude/tender-einstein-n6cp5x, PR #4898)
- [x] Wave 7, the five untried Curve25519 IC leads from the 2026-11-22 survey, in parallel: (L1) hash-membership factor base in the relation pipeline vs bits/random bases; (L2) higher-degree coordinate functions from projective/Jacobian/extended charts (y, xy, Jacobian-normalised) as decomposition variables; (L3) translated-involution degree-2 functions (P -> T - P, T != O; Huff/Jacobi-intersection models on the 2-isogenous curve); (L4) mixed E/twist S3 root count; (L5) frame/coset selector cost and gain extrapolated toward 255 bits (Claude, done 2026-11-22, branch claude/curve25519-index-calculus-i674j4, PR #4900, `research/curve25519_wave7_untried_20261122`; all five validated with errata; no lever found; one small E' x-frame gain awaits an engine check) [pcc:unverified]
- [x] Flat-presentation exponent study at m = 3 and m = 4 with wall time as co-primary unit (Curve25519 k = 20 miniature, protocol frozen 2026-11-15): m = 3 verdict settled as no lever under the preregistered estimator, s_T = 4.60 [4.40, 4.79]; m = 4 still undetermined (memory wall at D = 18); P2 failed; validated with errata (Claude, run 2026-11-15 to 11-22, branch claude/curve25519-index-calculus-i674j4, `research/curve25519_flat_exponent_20261115`) [pcc:unverified]
- [x] R4 (action plan 2026-11-21): triage the papers/ drafts into proposed submit / revise / archive with reasons, at most three to submit; Principal Cryptographer decides (Claude, started 2026-11-22, branch claude/papers-triage-20261122, PR #4913; triage drafted and checked 2026-11-22, decisions with the Principal Cryptographer)
- [x] F3+F4+F5 (action plan 2026-11-21): MIT LICENSE, disclosure / acceptable-use / ethics sections and dual-use table in SECURITY.md, gpu/btcpuzzle decision record; owners decide licence vs private (Claude, started 2026-11-22, branch claude/licence-security-policy-20261122, PR #4912; drafted and checked 2026-11-22, awaiting owner decisions: MIT vs private, btcpuzzle option, legal entity)
- [x] Land the tensor-contraction / clique-sum review work here (moved from workspace PR #60 on owner instruction): structured-LWE tensor-attack review and its ECDLP index-calculus connection, Majdoddin threshold model with proved overhead corrections and Lemma Q (density-one #3-SAT main term 2^324), distance to ECDLP-256 / AES-128 / Ring-LWE-1024, and the polynomial-decay analysis (Claude, completed 2026-11-22, branch claude/youthful-hawking-4n6jhp, PR #4904; `research/tensor_contraction_ecdlp_review_20261121/`; open threads in the session log)
- [x] R2 (action plan 2026-11-21): cost-to-break table v1 (P-256, secp256k1, Curve25519, RSA-2048/3072, ML-KEM-512/768, ECC2K-130 calibration) as versioned JSON + generator + README + SVG, derived from committed measurements and cited estimates; not a benchmark row (Claude, started 2026-11-22, branch claude/cost-to-break-table-20261122, PR #4914; v1 drafted and checked 2026-11-22, for the Principal Cryptographer's review; ecbench sessions for the rates are follow-up)
- [x] Chase genus >= 2 leads 3-5 from PR #4885: (3) extension-field genus-2 Jacobian decomposition with reduced factor base, two large primes and sparse LA; (4) genus-4 trigonal pencils with a large-prime step and cubic root finding plus hash lookup in place of the factor-base scan, larger p; (5) Smith route with Diem line relations and sparse LA on the produced quartic, measured negation rho on H, quartic group-order phase inside S (Claude, started 2026-11-21, branch claude/dlp-hyperelliptic-curves-genus-pj10r6)
- [x] Genus 6-16 degree-2 bases, follow-up: g=8 decay at p >= 101 (crossover p*(8)), independent validator replay of P1/P2 (`research/ic_genus2_16_speedup_20261117`), F1 (L03 base constant 1 vs 2 in genus_ladder, frozen protocol), and P3 non-hyperelliptic small-e models (g=4 two-node plane quintic, g=6 smooth plane quintic). F2/F3 are left to the PR #4693 items that own L06 and L01 (Claude, 2026-11-20, PR #4852: P1/P2 validated-with-errata incl. 236/236 certificates; g=8 gain 22-24x at p=73/101; P3 genus-4 quintic 0.003-0.03 of rho (relation phase, not a solve); F1 supported but not adopted (break-even LA price above measured). F1 sweep: best cc tracks the LA price, cc=1.5 at measured kappa~2. Done 2026-11-20; open threads (kappa at genus 3, P3 descent, p*(8)) in the session log)
- [x] Residue/character leads on Q = kG: (R1) homomorphic residues trivial on prime-order <G> incl. ECC2K-130 trace/halving; (R2) bias of non-homomorphic residue predicates vs k mod m against the character-sum bound; (R3) residue predicates as relation pre-filters for Semaev decomposition; (R4) cheap invariants for rho equivalence classes beyond negation/Frobenius (Claude, done 2026-11-20, branch claude/elliptic-curve-generator-scalar-0i752j, PR #4850; all four closed negative at toy scale, research/residue_leads_20261120/REPORT.pdf)
- [ ] Hyperelliptic frontier map, genus 2-5 over prime fields: per genus, the literature exponent (rho, 2LP, Diem quartic), the lab's best measured IC/rho row, and every open exponent lever with its blocking question; knowledge base and lead atlas first (Claude, started 2026-11-20, PR #4693)
- [ ] Genus 3, (2,2,2) non-pair kernels: decide by explicit small-p computation whether the 30 non-pair (affine-type) Lagrangians of a prime-order hyperelliptic genus-3 Jacobian give a non-hyperelliptic codomain, the open question left by g2plus L06 (Claude, 2026-11-20, PR #4693)
- [ ] Genus 3, odd l: re-freeze and finish the L06 (l,l,l) kernel census, then prototype one (3,3,3) isogeny to a plane quartic on a toy prime-order curve and price it against 2LP and rho (Claude, 2026-11-20, PR #4693)
- [ ] Genus 2 end-to-end: anchored pencil relations (L01) and automorphism orbits (L02) in one complete solve against matched rho at the §5a band (Claude, 2026-11-20, PR #4693)
- [x] Genus-4 cost calibration: measure the real GAE price of relation and mod-r linear-algebra operations against a rho group operation (timing tier), to test whether the w = 1 pricing of the validated genus-4 results is conservative (Claude, done 2026-11-19: w_rel ≈ 0.0025, w_la ≈ 0.0042–0.0045 GAE, robust bound w ≤ 0.042; w = 1 is conservative; validated-with-errata, no gate row because scalars are not independently certified; `research/genus4_cost_calibration_20261119`, PR #4693)
- [x] Factor-base rebalance on the elliptic IC lanes (lead B): no-go, LA <= ~3% on every elliptic lane and bases already tuned (research/lead_b_elliptic_la_share_20261119/ASSESSMENT.md). Redirect: DL1 base-size sweep on the genus-3 quartic GHS lane (LA 34-64%) is the one place the lever could pay (Claude, 2026-11-19, PR #4693)
- [ ] Genus-4 three-point exponent (third prime p = 54293, deviation D1: the binary cannot run p = 60013) and genus 5 at matched group sizes against the ideal negation-rho bound (lead C; Claude, started 2026-11-19, PR #4693)
- [ ] Measured transfer atlas: cover genus, base size and measured IC-vs-rho crossover for the transfer families the lab has tooling for (lead D; Claude, started 2026-11-19, PR #4693)

- [x] Genus-4 follow-up: more p = 16411 targets for arm x with matched plain rho, Ferrari inside the sieve (not applicable; factor-base balance instead), validator replay for the leaderboard (Claude, completed 2026-11-18, branch claude/hyperelliptic-index-calculus-td1fdn, PR #4693; validated)
- [x] Genus-4 arm x on prime-order (h = 1) Jacobians at p = 16411 and 32771 against the ideal negation bound, with linear-algebra tuning (cc x SGE) on the h = 2 curves (Claude, completed 2026-11-18, PR #4693; validated: x* 0.876 / 0.640 of ideal negation rho at w = 1, below at all w)
- [ ] Regenerate the IC leaderboard outputs (docs/ic/LEADERBOARD.md, leaderboard.json, ic-leaderboard.html) once artifact-store inputs can be pulled; the genus-4 section is in the builder (PR #4693)

- [x] F4 over F_p, follow-up to #4473, on the real consumer jv_cover (Claude, done 2026-11-20, PR #4851): shipped f4-parallel default 1.10x on jv_cover dlp; nested-dispatch and FL-threshold levers null; glibc malloc ~30% of F4; opt-in `--features jemalloc` 1.256x wall / 1.251x CPU end to end, pre-registered, 8/8 rounds [pcc:unverified]; records open pending the malformed hec-multi record on main; research/f4_inner_dispatch_20261120

- [x] Formalize conductor prime-gap bounds, scalar-Frobenius oracle hardness, and DLP transfer ceilings in Lean 4 (Cursor, completed 2026-11-20, branch cursor/lean-conductor-prime-gap-bounds-bdb7, PR #4815; claim PCC-20261120-70fc62 standing unverified; proof-gate PASS 125 thm)

- [x] Accelerate PKM Semaev and T_2-torus F4 via basis inheritance, Schwartz–Zippel refutation-trace replay, tower descent, and X_R-split ParamS5; evaluate m >= 8 T_2-torus and aligned P-256 auxiliary curve (Cursor, completed 2026-11-19, branch cursor/prime-pkm-aux-tower-3858, PR #4820; receipts in research/prime_pkm_aux_chase_20261119/{REPORT.md,RESULTS.json})

- [x] Implement order-free prime-field hyperelliptic index calculus (Codex, started 2026-11-18, branch codex/order-free-prime-hyperelliptic-ic-20261118, PR #4676)

- [ ] Test shared-coefficient membership nonenumeratively with blinded B=4/8 controls and a fully charged B=31 one-target solve, compared with cached BSGS and optimized Pollard rho across at least six growing sizes (Codex, started 2026-11-18, branch codex/shared-coefficient-membership-20261118-v2, PR #4692; follow-up to merged setup PR #4675)
- [x] Prove a division-free exact subexponential decomposition oracle for the hashed prime-field networks, including fixed witness bits (Codex, completed 2026-11-18, branch codex/ecdlp-novel-oracle-20261118, PR #4668)

- [ ] Benchmark the Boolean Faugère–Lachartre F4 split and enforce canonical binary-field profiles across Gröbner/Semaev workloads (Codex, started 2026-11-18, branch codex/semaev-fl-benchmark-20261118, PR #4710)

- [ ] Execute the frozen ecbench Phase-1 panel through the cold one-target IC-versus-rho gates, starting with the smallest runnable Koblitz and prime cells (Codex, started 2026-11-18, branch codex/ecbench-phase1-e2e-20261118; tracking PR #4681 merged, content PR #4785)

- [ ] Improve the exact Semaev projection/index-calculus lane toward cold one-target rho parity, with complete counted costs and verified recovery (Codex, started 2026-11-18, branch codex/semaev-rho-parity-20261118, PR #4682)

- [ ] Complete the next curve-transfer infrastructure slice: catalog-general norm replay, independent divisor-map oracle, odd two-infinity support, cryptanalysis mirror, standalone packaging, and CI evidence (Codex, started 2026-11-18; tracking PR #4683 merged; implementation continuation PR #4697; cryptanalysis continuation PR #1770; per-job artifact-hydration PR #4705)

- [x] Formalize or refute the ready yield/rank frontier obligations: population contract, solver visibility, and kernel-mode scope (Codex, completed 2026-11-18, branch codex/extend-ecdlp-obligation-graph-20261118, PR #4704; follows merged #4685)

- [x] Split the index-calculus yield/rank frontier into attackable obligations and evidence-linked targets (Codex, completed 2026-11-18, branch codex/extend-ecdlp-obligation-graph-20261118, PR #4685)

- [x] Track the separate exact GitHub-runner norm-branch fallback after EKS admission failure (Codex, started 2026-11-17, branch codex/norm-branch-github-fallback-20261117, PR #4686; implementation moved to a fresh branch after automatic merge)

- [x] Add the fail-closed pre-launch checkpoint for the frozen norm-branch tensor census (Codex, started 2026-11-17, branch codex/norm-branch-census-run-20261117, PR #4570)
- [ ] Certify prime31 exact geometric CM conductor via 3-division and tighten nonscalar endomorphism degree bound (automation, started 2026-11-18, branch automation/prime31-cm-conductor3-certificate-20261118)
- [x] Publish prime-field coverage, Split4 and RR9 follow-up evidence (Codex, started 2026-11-17, branch research/prime-field-followup-20261117, PR #4624)

- [ ] Prime31 CM endomorphism-degree exclusion and non-generic solver lead triage (automation, started 2026-11-18, branch automation/prime31-cm-degree-gate-20261118, PR #4657; exact gate implemented, CI pending)

- [x] Root-finding 100x: baseline and plan (claude, 2026-11-17, branch claude/rootfind-baseline-100x, PR #4619, stacked on #4607; report in research/rootfind_baseline_100x_20261117)
  - [x] Survey what the repository already measured and ruled out for root finding (`SURVEY.md`)
  - [x] Local baseline: per-phase time and counted operations, prime and Koblitz, with an A/A noise floor (`RESULTS.md`)
  - [x] Component breakdown of where the time goes (addition kernels, probe, table build)
  - [ ] Roofline and cycle counts: not possible here (no hardware PMU); needs the lab's isolated runner
  - [x] Plan: ranked levers with predicted factors and the cheapest falsifying test each (`PLAN.md`); lever 1 prototyped and measured
- [ ] Root-finding 100x, with the 50x frozen-prime-field milestone first: land lever 1 as an oracle with witness recovery and ledger parity, then run the cheap tests for levers 2–4 (interleaving, L2-resident filter, threads) (Codex, started 2026-11-18, branch codex/rootfind-50x-prime-20261118, PR #4669 merged; Lean follow-up PR #4696)
  - [x] Add a native independent recomputation of the frozen raw rows, exact 50x thresholds and arithmetic erratum (`research/rootfind_baseline_100x_20261117/{VALIDATION,ERRATA}.md`)
  - [x] Prove soundness and completeness of an abstract finite-domain decomposition-to-SAT encoding in Lean, without real-curve attack integration (`papers/rootfinding-decomposition/lean/PointDecompositionSAT.lean`, PR #4696)
- [ ] Repair the 28 examples/tests/bins on main broken by lossy merges (syntax, struct drift, lost library APIs) (claude, started 2026-11-17, branch claude/fastest-grobner-base-algorithm-u74ipw; in progress)
- [x] Fix the `wide-vars` build (u64 vs `Mask` in f4_trace, pq_f4_f2, koblitz_groebner) and clear `clippy-maintained.sh` -D warnings (claude, 2026-11-17, branch claude/wide-vars-clippy-cleanup, PR #4607, stacked on #4589; done, awaiting review)
- [x] Clean up main CI breakages and add an ecbench CI check (claude, started 2026-11-17, branch claude/fastest-grobner-base-algorithm-u74ipw, PR #4569 merged; ecbench-smoke check passing; remaining main blockers need the owner: AWS key, pcc claim_type, hec-multi record, rho currency)

- [ ] Freeze H115 Frobenius--Kummer resolvent-moment ECDLP lead, derive its cohomological cost obligations, and preregister exact small-curve controls (Codex, started 2026-11-17, branch research/h115-frobenius-kummer-resolvent-freeze, PR #4625)

- [ ] H116 formal-scalar low jet: lift the excluded-fixture constructor through the elliptic formal-group/Frobenius bridge and test an unchanged out-of-sample cohort (Codex, started 2026-11-18, branch research/h116-truncated-fiber-traces-freeze, draft PR #4699; continuation after merged PRs #4630 and #4667; five full-map matches, 243-theorem Mathlib formal-group/point/Frobenius/reduced-rational-function/quadratic-sandwich/no-cancellation/quotient-digit/Kummer-symmetry/division-torsor/signed-trace/Prym-coefficient/Weil-homogenization/branch-discriminant/resultant/quartic-trace/invariant/marked-two-isogeny bridge including scalar-two/three/four/five/six/seven/eight/nine/ten coprimality, coordinate degrees four/nine/sixteen/twenty-five/thirty-six/forty-nine/sixty-four/eighty-one/one-hundred, four generic recurrence stencils, a source-linked arbitrary-scalar resultant-to-degree implication, an exact derivation `X1=u^2-u0^2`, base-six quotient identities, a generic x-only parity impossibility theorem, a fixed-translate Frobenius-cocycle theorem, a signed-twist trace-parity obstruction and characteristic-free elliptic--Prym coefficient recovery, base-field descent of the no-common-root/adjacent-torsion semantic layer, and the algebraic-closure root-to-nonzero-point lift, frozen splitting/branch/inseparable fiber and exact scalar-coprimality protocols, two 20/20 boundary panels, five-curve coordinate control, an excluded-fixture 576-record invariant-moment control rejecting all 11,520 fixed single-point digit predicates, an unchanged five-curve quotient-digit OOS test replicating `IX1-argmin` association at 845/4328 covered targets but rejecting exact decoding, rank scaling panels that require full width at orders through 19,927, quadratic-cover/Miller/division-polynomial character families calibrated null, a `[2]`-division-torsor selector proved to collapse to zero, fixed-genus parity/first-trace/second-zeta panels calibrated null, a Prym discriminant rule that failed unchanged OOS, auxiliary-prime real-Weil splitting and full-Weil eigenline-presence bits calibrated null, complete factor-degree transport null, target-tethered residual branch characters and pairwise resultants null, auxiliary genus-one branch-pair traces null at 536/1444 with p=0.253600, binary-quartic `I,J` characters null at 764/1431 with p=0.929216, and marked source/dual kernel-halving bits null at 762/1444 with p=0.997803; next test the complete Frobenius cycle type on each four-point halving fiber, while concrete H116 field embeddings, exact universal-resultant convention alignment, an exact relational public scalar predicate, duality and actual formal-map instance remain open) [pcc:unverified]
- [x] Audit the tensor-contraction and fixed-arity proof routes to total subexponential ECDLP, and test the other assessed leads against the same scaling criterion (Codex, completed 2026-11-18, branch codex/ecdlp-subexponential-proof-report-20261118, PR #4665)

- [x] Assess the subexponential ECDLP literature and the status of the lab's index-calculus leads, then identify the first unresolved mathematical test (Codex, completed 2026-11-17, branch codex/ecdlp-subexponential-goal-20261117, PR #4627)

- [x] Regenerate artifact-store pull steps in the 36 drifted workflows (`tools/artifacts/wire_workflows.py`) so the `artifacts` check passes (claude, started 2026-11-18, branch aburan28/vigilant-wright-xm1vt6-workflow-wiring)

- [x] Chase open graph-IC (extension_ic_graph_20261115) leads in parallel: Run51 outcome, descent finder, complete accounting, fresh/scaling protocol, rho fold, independent validation, root build (claude, started 2026-11-18, branch aburan28/charming-clarke-qfc2gp, PR #4654; 2 of 7 lanes landed)

- [x] O4 protocol Amendment 2 before any data: settle the eight open readings in chained_transfer_next PROGRESS.md and fix choice 4 (a wrong log from a rank-deficient solve must trigger a retry, not an uncertified run) in quartic_ic (claude, started 2026-11-18, branch claude/ecdlp-chained-transfers-8cpiek, PR #4632)

- [x] Track conditional survivors and variable effects in the degree-fall toy report (Codex, started 2026-11-18, branch codex/degree-fall-variable-colors-20261118, PR #4639)

- [ ] LEAD-ICE-hybrid-guess-determine-l7: stretch cell E8 (3, 22, 8), k* under R1 and its nested extension, k = 0 baseline, fresh draws, on a RunPod CPU pod (claude, started 2026-11-18, branch claude/ic-lead-hybrid-gd-l7)

- [x] k-sum table build with x-only siblings at the last level (P2 KSUM-SIB): paired benchmark on the Table B curves, plus the P1 x(iP) recurrence probe (claude, started 2026-11-18, branch aburan28/inspiring-dijkstra-ay05qi, PR #4649): done; record close blocked by main's malformed 20261117T0334Z-hec-multi-production-cold.json

- [ ] Table B k-sum: profile the cost split of the pruned k-sum arm and land the next measured lever (claude, started 2026-11-18, branch claude/f4-f5-algorithm-speed-bvp7qd, PR #4651)

- [ ] Koblitz K1 small rows (n23/n29/n37): next lever on the generator-orbit IC arm, gap reported against both rho strong and v5 (claude, started 2026-11-18, branch claude/f4-f5-algorithm-speed-bvp7qd, PR #4651); measured: known-log seed columns (generator_multiples + multiple_doublings) 0.59x gen-c4 at n23, 0.38x gen-c8 at n37 (w=0), n29 inert; awaiting independent validation (research/koblitz_known_log_columns_20261118) [pcc:unverified]

- [ ] Repair main's compile and clippy 1.98 breakage (all targets) so PRs can pass `check` and `clippy (crypto)`; with the GPU rootfind/echelon/cost-leads work in the same lane PR (claude, started 2026-11-18, branch claude/index-calculus-gpu-optimization-otouqv, PR #4650)

- [ ] Run the preregistered prime medium-tier rho calibration (research/prime_medium_tier_calibration_20261115) on isolab-eks: ecbench rho.negation/rho.negation_v2/rho.plain/bsgs.negation(_v2) + A/A at log2 r 31.9-47.6; audit, fit bounds, update frontier (claude, started 2026-11-18, branch claude/prime-medium-rho-calib-run-20261118)

- [ ] Novel radical (closed-form) univariate root finder for low-degree (d≤4) Semaev polynomials over F_p vs Cantor–Zassenhaus: prototype, validate, measure field-op counts (claude, started 2026-11-17, branch claude/radical-rootfind-20261117)

- [x] Add a Faugère–Lachartre reducer/free-column split to Boolean F4 for Semaev systems (Codex, started 2026-11-17, branch codex/semaev-f4-subset-reducers, PR #4562; merged)

- [x] Freeze and audit #4556 exact-source public B8 RR9 result with native JSON receipt (automation, started 2026-11-17, branch research/prime31-barycentric-evidence-20261117, PR #4558)

- [x] Build TorsionGuard finite-primary-module soundness analyzer (Codex, started 2026-11-17, branch codex/torsionguard-core-20261117, PR #4579)

- [ ] Publish Vieta Rust speed results and resolve official record/publication blockers (codex, started 2026-11-17, branch lane/vieta-speed-20261117, PR #4525)

- [ ] Derive and measure an exponent-moving Curve25519 mechanism in native Rust (Codex, started 2026-11-17, resumed 2026-11-18, branch codex/curve25519-exponent-ideas-20261117, PR #4677; split/sign contracts complete; projective m=3 qualification protocol frozen before implementation; m=4 exponent stage gated on correctness)
- [x] Port the linearized two-summand root finder (lfd-summation-systems PR #28) into the Semaev decomposer and measure it vs the pair loop (claude, started 2026-11-17, branch claude/index-calculus-root-finding-e95kd6, PR #4578); follow-up: wrap as an ic_framework oracle and compare with rootfind-mitm in the harness unit

- [x] Repair main-side CI breakage left by the 2026-11-16 merges: parity lock and IC reference currency (workflow store pulls deferred until the store is repaired) (claude, started 2026-11-17, branch claude/hyperelliptic-index-calculus-td1fdn, PR #4576)

- [x] Unbreak main: remove the duplicate recording_degree_falls and VisitReport fall-degree fields in f4_fp.rs left by a double-restore (claude, started 2026-11-17, branch claude/fix-f4fp-dup-degree-falls-20261117)

<!-- One line per task, added as the task's first step with its draft PR:
- [ ] <task> (<runtime>, started YYYY-MM-DD, branch <branch>, PR #<n>) -->
- [x] Gröbner solver shootout on GF(2^n) Semaev systems: local diagnostic, inherited-f4 vs msolve vs CMS (claude, started 2026-11-16, branch claude/fastest-grobner-base-algorithm-u74ipw, PR #4543; CI blocked by AWS/S3 and main breakages)
- [x] Track every task first in docs/ACTIVE.md and a draft PR; roll /goal out to isogeny-labs repos (claude, started 2026-11-17, branch track-first-active, PR #4499)
- [ ] Autolab rounds 38 to 40 (claude, started 2026-11-17, branch ic-autolab-20261117b): round 39 ratchets `tw2l-x20-j16-n2` at k = 28 (0.944, counted GAE; wall time 1.236x slower; validator disposition validated for the counted claim); open: a second-host replay or human validation, a code-level lever beyond the K = 3 collision floor, and the `hashed-ecdl-tensor` standalone build failure (eight modules with no `mod` line in `lib.rs`)

## Blocked / questions for the owner

- Curve-transfer infrastructure continuation (Codex, PR #4697): the scoped
  curve workflow is green at exact implementation/workflow head `d8df2130...`,
  but merge/shipping remain blocked on artifact-store recovery or an approved
  replacement, classification of broad repository CI hydration failures,
  human/PCC review and signed standing, and an independently downloadable
  package replay. Fully hydrated browser freshness, unsupported transfer
  backends, the projective exceptional-point-safe Edwards adapter, and the
  separate full Jacobian index-calculus solver remain open deliverables.

- Conductor crossing continuation (#4690): local worker packaging and supplied 47-bit input are recorded. Registry publication/launcher integration, live budget/reservation/capacity checks, full 80-bit construction feasibility and matched endpoint measurements remain open. No large-case job was launched; the minimum 80-bit goal is unchanged.


- Lazy-orbit column sweep vs strong rho (claude, 2026-11-17, branch claude/admiring-gauss-vo7kxf, PR #4565): protocol frozen (`research/ic_lead_chase_20261116/protocol.json`, sha256 19116bb1…, source 3658d3aa26), specs and analysis bound, queue registrations `experiments/queue/EXP-LAZYCOL-N{17,29,31}-001.json` filed `blocked`. Needs: live daily+monthly budget headroom in account 104186811093 (latest readback over the monthly cap; this runtime has no lab AWS identity), approved expspec records, an isolab image built from 3658d3aa26, and a perf-ipc w_cal host.

- Vieta:504/504 primary and72/72 calibration calls completed/replayed. Canonical seed-registration360/60 failures retained; no L2 admission. Official censored-close blocked by unchanged main20261105T0828Z archive command1 lacking successful integer exit. Shared crypto-lean PR26 CI cannot start due account billing lock; no bypass/spend. Full general tower NF instance remains unverified.
- [x] Formalize whether the semantic-rank premises imply a no-subexponential-ECDLP goal, with an explicit Lean countermodel or bridge theorem (Codex, completed 2026-11-18: implication refuted, sufficient universal-work bridge and architecture endpoint checked, real-ECDLP universal lower bound remains open, branch codex/extend-ecdlp-obligation-graph-20261118, PR #4704)
- [ ] Audit and pursue the conditional tensor-contraction and alternative-parameter routes toward an explicit subexponential prime-field ECDLP algorithm (Codex, started 2026-11-18, branch codex/extend-ecdlp-obligation-graph-20261118, PR #4704; objective remains open until an end-to-end algorithm and proof are supplied)
