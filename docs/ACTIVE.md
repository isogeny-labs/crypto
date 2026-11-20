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
- [ ] Rational-discriminant batch-inversion fast path in `semaev_projection.rs` (1D-1): the S3 discriminant on rational inputs is `(4 y1 y2)^2`, so `Irreducible` is unreachable and Tonelli-Shanks is unnecessary; add `quadratic_roots_known_sqrt` + Montgomery `batch_invm` + early-exit `decompose_four_first`, keeping the originals as `*_generic` and the public API. Verified: identity exact (symbolic + 2400 pairs), 6/6 tests pass, Prime31 B=64 build 4096->1 invm / per-pair sqrtm eliminated / 12.5x fewer mulm, 0 mismatches. Benchmark-row gates (wall-clock on isolab-eks, matched-rho validation cert, bench.record/S3, research-visuals, pcc) deferred to a lab run (Claude Code, started 2026-11-20, branch claude/semaev-projection-batch-inv-20261120)
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

- [x] Formalize conductor prime-gap bounds, scalar-Frobenius oracle hardness, and DLP transfer ceilings in Lean 4 (Cursor, completed 2026-11-20, branch cursor/lean-conductor-prime-gap-bounds-bdb7, PR #4815; claim PCC-20261120-70fc62 standing unverified; proof-gate PASS 125 thm)

- [x] Accelerate PKM Semaev and T_2-torus F4 via basis inheritance, Schwartz–Zippel refutation-trace replay, tower descent, and X_R-split ParamS5; evaluate m >= 8 T_2-torus and aligned P-256 auxiliary curve (Cursor, completed 2026-11-19, branch cursor/prime-pkm-aux-tower-3858, PR #4820; receipts in research/prime_pkm_aux_chase_20261119/{REPORT.md,RESULTS.json})

- [x] Implement order-free prime-field hyperelliptic index calculus (Codex, started 2026-11-18, branch codex/order-free-prime-hyperelliptic-ic-20261118, PR #4676)

- [x] Prove a division-free exact subexponential decomposition oracle for the hashed prime-field networks, including fixed witness bits (Codex, completed 2026-11-18, branch codex/ecdlp-novel-oracle-20261118, PR #4668)

- [ ] Benchmark the Boolean Faugère–Lachartre F4 split on matched Semaev systems (Codex, started 2026-11-18, branch codex/semaev-fl-benchmark-20261118, PR #4679)

- [ ] Execute the frozen ecbench Phase-1 panel through the cold one-target IC-versus-rho gates, starting with the smallest runnable Koblitz and prime cells (Codex, started 2026-11-18, branch codex/ecbench-phase1-e2e-20261118, PR #4681)

- [ ] Improve the exact Semaev projection/index-calculus lane toward cold one-target rho parity, with complete counted costs and verified recovery (Codex, started 2026-11-18, branch codex/semaev-rho-parity-20261118, PR #4682)

- [ ] Complete the next curve-transfer infrastructure slice: catalog-general norm replay, independent divisor-map oracle, odd two-infinity support, cryptanalysis mirror, standalone packaging, and CI evidence (Codex, started 2026-11-18, branch codex/curve-transfer-next-phase-20261118, PR #4683)

- [ ] Formalize or refute the ready yield/rank frontier obligations: population contract, solver visibility, and kernel-mode scope (Codex, started 2026-11-18, branch codex/extend-ecdlp-obligation-graph-20261118, PR #4685)

- [x] Split the index-calculus yield/rank frontier into attackable obligations and evidence-linked targets (Codex, completed 2026-11-18, branch codex/extend-ecdlp-obligation-graph-20261118, PR #4685)

- [ ] Preregister and execute the exact GitHub-runner norm-branch fallback after EKS admission failure (Codex, started 2026-11-17, branch codex/norm-branch-github-fallback-20261117, PR #4686)

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
- [ ] Root-finding 100x, next: land lever 1 as an oracle with witness recovery and ledger parity, then run the cheap tests for levers 2–4 (interleaving, L2-resident filter, threads)
- [ ] Repair the 28 examples/tests/bins on main broken by lossy merges (syntax, struct drift, lost library APIs) (claude, started 2026-11-17, branch claude/fastest-grobner-base-algorithm-u74ipw; in progress)
- [x] Fix the `wide-vars` build (u64 vs `Mask` in f4_trace, pq_f4_f2, koblitz_groebner) and clear `clippy-maintained.sh` -D warnings (claude, 2026-11-17, branch claude/wide-vars-clippy-cleanup, PR #4607, stacked on #4589; done, awaiting review)
- [x] Clean up main CI breakages and add an ecbench CI check (claude, started 2026-11-17, branch claude/fastest-grobner-base-algorithm-u74ipw, PR #4569 merged; ecbench-smoke check passing; remaining main blockers need the owner: AWS key, pcc claim_type, hec-multi record, rho currency)

- [ ] Freeze H115 Frobenius--Kummer resolvent-moment ECDLP lead, derive its cohomological cost obligations, and preregister exact small-curve controls (Codex, started 2026-11-17, branch research/h115-frobenius-kummer-resolvent-freeze, PR #4625)

- [x] Audit the tensor-contraction and fixed-arity proof routes to total subexponential ECDLP, and test the other assessed leads against the same scaling criterion (Codex, completed 2026-11-18, branch codex/ecdlp-subexponential-proof-report-20261118, PR #4665)

- [x] Assess the subexponential ECDLP literature and the status of the lab's index-calculus leads, then identify the first unresolved mathematical test (Codex, completed 2026-11-17, branch codex/ecdlp-subexponential-goal-20261117, PR #4627)

- [x] Regenerate artifact-store pull steps in the 36 drifted workflows (`tools/artifacts/wire_workflows.py`) so the `artifacts` check passes (claude, started 2026-11-18, branch aburan28/vigilant-wright-xm1vt6-workflow-wiring)

- [ ] Chase open graph-IC (extension_ic_graph_20261115) leads in parallel: Run51 outcome, descent finder, complete accounting, fresh/scaling protocol, rho fold, independent validation, root build (claude, started 2026-11-18, branch aburan28/charming-clarke-qfc2gp)

- [x] O4 protocol Amendment 2 before any data: settle the eight open readings in chained_transfer_next PROGRESS.md and fix choice 4 (a wrong log from a rank-deficient solve must trigger a retry, not an uncertified run) in quartic_ic (claude, started 2026-11-18, branch claude/ecdlp-chained-transfers-8cpiek, PR #4632)

- [x] Track conditional survivors and variable effects in the degree-fall toy report (Codex, started 2026-11-18, branch codex/degree-fall-variable-colors-20261118, PR #4639)

- [ ] LEAD-ICE-hybrid-guess-determine-l7: stretch cell E8 (3, 22, 8), k* under R1 and its nested extension, k = 0 baseline, fresh draws, on a RunPod CPU pod (claude, started 2026-11-18, branch claude/ic-lead-hybrid-gd-l7)

- [x] k-sum table build with x-only siblings at the last level (P2 KSUM-SIB): paired benchmark on the Table B curves, plus the P1 x(iP) recurrence probe (claude, started 2026-11-18, branch aburan28/inspiring-dijkstra-ay05qi, PR #4649): done; record close blocked by main's malformed 20261117T0334Z-hec-multi-production-cold.json

- [ ] Table B k-sum: profile the cost split of the pruned k-sum arm and land the next measured lever (claude, started 2026-11-18, branch claude/f4-f5-algorithm-speed-bvp7qd, PR #4651)

- [ ] Koblitz K1 small rows (n23/n29/n37): next lever on the generator-orbit IC arm, gap reported against both rho strong and v5 (claude, started 2026-11-18, branch claude/f4-f5-algorithm-speed-bvp7qd, PR #4651)

- [ ] Repair main's compile and clippy 1.98 breakage (all targets) so PRs can pass `check` and `clippy (crypto)`; with the GPU rootfind/echelon/cost-leads work in the same lane PR (claude, started 2026-11-18, branch claude/index-calculus-gpu-optimization-otouqv, PR #4650)

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


- Lazy-orbit column sweep vs strong rho (claude, 2026-11-17, branch claude/admiring-gauss-vo7kxf, PR #4565): protocol frozen (`research/ic_lead_chase_20261116/protocol.json`, sha256 19116bb1…, source 3658d3aa26), specs and analysis bound, queue registrations `experiments/queue/EXP-LAZYCOL-N{17,29,31}-001.json` filed `blocked`. Needs: live daily+monthly budget headroom in account 104186811093 (latest readback over the monthly cap; this runtime has no lab AWS identity), approved expspec records, an isolab image built from 3658d3aa26, and a perf-ipc w_cal host.

- Vieta:504/504 primary and72/72 calibration calls completed/replayed. Canonical seed-registration360/60 failures retained; no L2 admission. Official censored-close blocked by unchanged main20261105T0828Z archive command1 lacking successful integer exit. Shared crypto-lean PR26 CI cannot start due account billing lock; no bypass/spend. Full general tower NF instance remains unverified.
