# Atlas verdict patch: cells 0.1, 0.2, E3, D4 (and notes on C5, A4)

Date: 2026-11-15. Follow-up item 4 of `research/leads_followup_20261114/`. Source proposals: `research/leads_20261112/README.md` §4 (the wording below is copied from there). Owner of the apply step: the lead-atlas owner in crypto-autoresearcher (the atlas bodies are vault-encrypted, so this patch was not applied and nothing in any atlas was read or changed).

Every result statement below is `[pcc:unverified]`: the numbers are quoted from the source documents named in each entry, and nothing was re-run for this patch. This patch changes no atlas body, leaderboard row, or `benchmarks/README.md` row.

## Before applying (read first)

1. **Atlas bodies were not readable here.** The current verdicts come from the atlas brief as recorded in `research/leads_20261112/BRIEFING.md` (legend: REF, ENG, OPEN, JAC, WEAK). The owner must check each cell body against the definitions quoted below, especially 0.2 and D4 (see the two discrepancies in the entries).
2. **Label vocabulary.** "closed by structure" (E3) and "open on the solver axis only" (0.2) are not terms in the brief legend. The owner picks the legend label; the wording is kept as proposed.
3. **Evidence depth.** RESULTS.md (`research/leads_chase_20261112/RESULTS.md`) has sections only for P1, S1, J2, M1 and J1. Nothing was run for 0.2 (S2), E3, D4 (K1) or A4. Those entries rest on desk arguments and the encyclopedia, and say so.
4. **research-kb was not queried.** No research-kb tool is present in this session's tool list (checked with ToolSearch). The "Sources checked" pass that CLAUDE.md requires before a lead claim is therefore owed; the atlas owner should run it before applying.
5. **Repository state at drafting.** `/home/user/crypto` at `44d35033fa`; `/home/user/ml-cryptanalysis` at `e8aecdd`.

Source hashes (sha256, as read):

- `research/leads_20261112/README.md` c569296c3af97524bea6cfa9dfa44cb393bf023a10921c2bd5462e9b88db02c4
- `research/leads_20261112/BRIEFING.md` 6c217ccf75b609f9861cdfae451e8fb1624125fb851c65a5787553761a1866a2
- `research/leads_chase_20261112/RESULTS.md` 235bc6331c86a3b2d948f0866dbaf67e79e9f87770db7988fb3f0ba508b4e8c3
- `research/leads_chase_20261112/figures/p1_quotient_law.svg` 66e8c579f525f53e44a626d01a724deaf495ddece67774e7fe7a9e3e2af79c43
- `research/leads_chase_20261112/figures/s1_departures.svg` 0e5e079d6aceb8ecf48e8adca7d04e7e29b3cf97778d23c43dcf6836f6dac64a
- `docs/ic/factor-base-encyclopedia/README.md` f97abc0f9abb4eadee6007b61662c953c59639d1a4df90a4aeadc64c5cb78500
- `research/cyclotomic_falsifier_20261015/README.md` be5199e1a20534e06159a6a615eb4e311ebd67ae4971976280ee8c96399023b6
- `research/subgroup_trap_20261107/README.md` 53c720c9a9087375ea854afae066b8f6fec724b2a1cab3e34afa809f833affa5

---

## Entry 1: cell 0.1, Kummer g·μ_{2^t} (P-224 shape)

**Current verdict:** OPEN (flagship), per the brief.

**Proposed verdict (verbatim from README §4):** `open only for m ≥ 3 with LFD/k < 0.184, or slope < 0.083 at m = 2` (§1).

**Evidence:**

- `research/leads_20261112/README.md` §1: the thresholds (m = 2: columns 0.220, sparse linear algebra at ω = 2 is 0.083, ω = 2.81 is 0.054; m = 3: 0.522, 0.184, 0.117). The measured m = 2 slope is 0.25. Its "no record of a certified m = 3 prime-chain LFD" sentence is quoted as the state of the files read there.
- `research/leads_20261112/BRIEFING.md`, "What is settled today about the fall degree of the flagship family 0.1": kernel LFD ≤ k + 2 (Lean, no axioms); measured LFD = ⌊(k+14)/4⌋ to k = 15, conjecture beyond; written proof gives LFD ≥ ⌊(k−10)/14⌋ + 1 for unit instances with t ≠ 0. Open items named there: slope between 1/14 and 1, t = 0, m ≥ 3, singular curves.
- `research/leads_chase_20261112/RESULTS.md`, section "M1 three-summand arity grid": m = 3 gives LFD = 7 at every k ≤ 5 and LFD ≥ 8 at k = 6. LFD/k is 1.4 at k = 5 and ≥ 1.33 at k = 6. Columns against enumeration: 16× at k = 5 and 26× at k = 6. Desk result from `ml-cryptanalysis/experiments/ic-bounds-m3-depth/`.
- `research/leads_chase_20261112/RESULTS.md`, section "P1 quotient law": all 72 plateau cells equal the committed q/2^k out of sample; the band [2.5, 6.0] holds for j ≤ 3. Stage 2 (D = 6 at k = 15, predicted q = 167,665) is not run. Figure: `research/leads_chase_20261112/figures/p1_quotient_law.svg`.
- `research/leads_chase_20261112/RESULTS.md`, section "S1 crossing": the Dm hypothesis survives (a second equation lowers the fall degree at k = 10; characteristic down to 19 with one equation does not). Figure: `research/leads_chase_20261112/figures/s1_departures.svg`. Context only; the §4 wording does not use it.

**Caveats and discrepancies:**

- The M1 ratios come from k = 5 and 6 only. Small-k ratios carry an offset. The measured m = 2 law gives LFD/k = 4/5 = 0.8 at k = 5 while its slope is 0.25 (derived from ⌊(k+14)/4⌋). So the M1 ratios do not decide the asymptotic m = 3 threshold of 0.184. The m = 3 branch is neither met nor refuted by the data in hand. [pcc:unverified]
- The m = 2 branch is not met by the measured law (slope 0.25 to k = 15, conjecture beyond). The proved floor 1/14 = 0.071 lies below 0.083, so the proof does not exclude this branch on the ω = 2 line. It does lie above the ω = 2.81 line (0.054), as README §1 states.
- README §2a says P1 holding would make 0.1 "engineering-only, independent of the slope". The §4 wording does not say that and does not use P1. The owner should pick one; this patch copies §4.

**What would reverse it:**

- A certified m = 3 LFD(k) series with LFD/k below 0.184 at growing k (README §1). The next measurement named by M1 is D = 8 at k = 6 and k = 7 (GPU-class, not run).
- A measured m = 2 slope below 0.083. The named trigger is 1 ∈ V_7(18), a plateau longer than four (README §2a P2). Current measured slope is 0.25.
- A stage-2 mismatch in P1 (D = 6 at k = 15 with q ≠ 167,665) would undercut the P1 quotient-law reading. It would not reverse the §4 wording.
- The open items named in the BRIEFING proof summary (slope in (1/14, 1), t = 0, m ≥ 3, singular curves) resolved against the cell.

---

## Entry 2: cell 0.2, isogeny tower x(R' + ⟨Q⟩)

**Current verdict:** OPEN (flagship), per the brief.

**Proposed verdict (verbatim from README §4):** `open on the solver axis only; S2 decides whether its graded algebra changes the degree law`. README §4 adds: "Lane P's P4 reading of 0.2 was a misreading and is withdrawn."

**Evidence:**

- `research/leads_20261112/README.md` §2c, S2 (`LEAD-SYN-velu-tower-exact-lfd`): exact closure LFD of the Vélu tower on unit instances at k = 4..10, weight-matched against a degree-2 map-image base, with k = 14 as stage 2. Prior from the encyclopedia's F4 degrees for this tower: m = 2 gives 4,5,5,…,5,6 at N = 2..20, against Kummer's 4,4,4,4,4,5,…,6. README notes these are F4 degrees, not certified LFDs. S2 is not run; it needs a Rust tower-closure port (1–2 days).
- `research/leads_20261112/README.md` §2a, P4 (dropped): it read 0.2 as an isogeny into the target's class. The encyclopedia §0.2 (`docs/ic/factor-base-encyclopedia/README.md`, from line 354) defines 0.2 as an auxiliary curve E′ with a Vélu x-map tower. P4 belongs to the already-refuted A11/D7 reading.
- `research/leads_20261112/lanes/synthesis.md` (lines about 145–148, "A contradiction between records"): the same conflict, with the coordinator to read the atlas body.
- `research/leads_20261112/lanes/synthesis.md` (around line 318): registered prior probabilities H_same 0.45, H_flat 0.35, H_steep 0.20. These are priors, not results.
- The encyclopedia §0.2 "Measured" paragraph: measured only at p₀ = 786433, t ≤ 10 (about the Vélu tower, not the degree law).
- `research/leads_chase_20261112/RESULTS.md`: no section for 0.2. Nothing was run.

**Caveats and discrepancies:**

- The proposed wording is valid only if the atlas body's 0.2 is the auxiliary-curve Vélu tower of encyclopedia §0.2. Under the P4 reading the cell belongs to A11/D7, and the wording does not apply.
- "Open on the solver axis only" is a prior-driven statement until S2 runs. The H_same, H_flat and H_steep priors are not evidence.

**What would reverse it:**

- S2 shows the Vélu tower's exact closure LFD on unit instances at k = 4..10 departs from the Kummer law (H_flat or H_steep, as registered in synthesis.md). The proposed wording would then need to change: the graded algebra would change the degree law, so the solver axis is not the only open part.
- The atlas body defines 0.2 as P4 reads it (an isogeny into E's class). The cell then falls under A11/D7 and the proposed wording must be replaced.

---

## Entry 3: cell E3, P-192 CM class ideals

**Current verdict:** OPEN, per the brief.

**Proposed verdict (verbatim from README §4):** `closed by structure` once `cm_closed_walk_scalar_action` is written: a closed class walk acts as one scalar on P and Q.

**Evidence:**

- `research/leads_20261112/README.md` §2b: E3 "closes by structure: a closed class walk acts as one scalar on P and Q and carries no logarithm information (Lean obligation `cm_closed_walk_scalar_action`, elementary)."
- `research/leads_20261112/lanes/lane_J_covers.md` (line 411, the E3 paragraph): the desk argument. An ideal class of small norm acts as an isogeny to a different curve of the class. A closed walk composes to [α], which acts on E(F_p) as the scalar (x + y) mod N, sending P to [λ]P and Q to [λ]Q with the same λ. Non-integer endomorphisms have degree about p, so none is cheap. The class group has about 2^96 elements. Stated as "Kept as an obstruction statement". The lane notes that a toy run would add nothing beyond the proof.
- Encyclopedia E3 entry (`docs/ic/factor-base-encyclopedia/README.md`, from line 1509; source `research/p192-weak-curve-20261006/REPORT.md` L249–380): 9,948,061 vectors gave 103 relations, all scalar; status INDETERMINATE. This is the measured census on the prime-ideal base; the scalar finding is the measured part.
- `research/leads_chase_20261112/RESULTS.md`: no section for E3. A grep for `cm_closed_walk_scalar_action` over `/home/user/crypto` and `/home/user/ml-cryptanalysis` (completed, exit 0) found the name only in prose: `research/leads_20261112/README.md`, `research/leads_20261112/lanes/lane_J_covers.md` and `docs/session-logs/2026-11-12.md`. No Lean declaration of that name was found in those trees. `crypto-lean` was not searched.

**Caveats and discrepancies:**

- Status of the Lean obligation: not established. Confirm that `cm_closed_walk_scalar_action` exists and checks before the wording is applied, since the verdict depends on it.
- The walk claim is a desk argument [pcc:unverified] until the obligation is written and checked.
- "Closed by structure" is not in the brief legend (see Before applying, item 2).

**What would reverse it:**

- A counterexample to the obligation as stated (a closed class walk whose action on P and Q is not one scalar). This would be a proof-level reversal.
- A relation source for the E3 base that is not produced by closed walks and does not act as a scalar. The encyclopedia census found all 103 relations scalar, but only for the vectors it searched.

---

## Entry 4: cell D4, cyclotomic

**Current verdict:** OPEN, per the brief. The brief row reads "D4 cyclotomic: yield-neutral ... OPEN".

**Proposed verdict (verbatim from README §4):** `engineering; the two-summand floor is exponent 2/3 and Koblitz curves proper are excluded from the m ≥ 4 Gaudry route by the group-order trap` (lane K; the fold K1 lives on the weak class over F_16).

**Evidence:**

- `research/leads_20261112/README.md` §2e: lane findings from `research/ecbench_pair_claw_20261003/README.md` (the pair claw at 2.195× matched rho pooled over six Koblitz curves, 2.51× at f2m41; orbit-IC at 18.16× at n41) and `research/subgroup_trap_20261107/README.md` (group-order trap: prime-order part at most about Q^φ(d)). The fold K1 is on `research/c1_twisted_line_ladder_20261027` (same yield 1.024 and per-trial cost 0.961 as the standard line; no fold, no end-to-end run).
- **Definition check (discrepancy).** The encyclopedia §D4 (`docs/ic/factor-base-encyclopedia/README.md`, from line 1274) defines D4 as the cyclotomic γ-weight base, C_w = {Σ γ^k : |S| = w}. Its measured result is yield-neutral at n = 23: all 15 tests lie in the null (smallest two-sided p 0.363; positive control p = 0.001), per `research/cyclotomic_falsifier_20261015/README.md`. Its cost against compact-orbit-scan is costlier by 1.022 [1.021, 1.023] (K_1, w = 2) and 1.122 [1.120, 1.125] (K_0, w = 2), both from the factor-base build phase.
- `research/leads_chase_20261112/RESULTS.md`: no section for D4. K1 is not run.

**Caveats and discrepancies:**

- **The evidence in §4 does not cover the encyclopedia's D4 object.** §4 and §2e rest on the Koblitz curve route, the two-summand exponent 2/3 and the Gaudry m ≥ 4 route. The encyclopedia's D4 is the γ-weight cyclotomic base, measured yield-neutral and costlier (above). The atlas owner must confirm which object cell D4 names before applying the wording. If it is the γ-weight base, the cited evidence is the falsifier and the cost ratios, not the Gaudry and Koblitz route.
- Lane K's "binary D4 (Gaudry, x in F_Q, n = 4m)" is a third use of the label "D4". Confirm it is not the cell meant.

**What would reverse it:**

- For the γ-weight base: a yield-positive result at a larger n than 23 (the falsifier's critical value was 0.00333 at n = 23), or a validated cost ratio below 1.0 against compact-orbit-scan at matched columns.
- For the Koblitz/Gaudry reading: a Koblitz instance where the group-order trap does not apply to an m ≥ 4 route, or a measured fold gain that changes the exponent (not just the constant), since "engineering" means constant factor only.
- K1's relations-to-full-rank counts at k = 5, 7, 9 falling outside the predicted 0.27, 0.17, 0.11–0.12 (±25%) would falsify the fold prediction. That leaves the wording standing on the other evidence, but removes its support from K1.

---

## Note: cell C5, trace-zero (no verdict change proposed)

**Current verdict:** ENG, per the brief.

**Proposed:** no change. README §4: "today's k = 11 binary measurement (LFD 5, every law) is consistent with 'engineering'."

**Evidence:**

- `research/leads_20261112/BRIEFING.md`, "Binary deep chain today": at k = 11, trace-zero gives LFD 5 (every law). This is interim: 16 of 57 systems, unverified.
- Encyclopedia C5 row (`docs/ic/factor-base-encyclopedia/README.md`, line 1171): as a pre-filter in the compact-orbit pipeline, 1.00003×, because odd-order IC targets lie in 2E and the filter never fires (0 rejections in 29,624 checks; `research/ic_improve_20261014/03-tracezero-freerel/README.md`, PR #1862).

**What would reverse it:** the final 57-system deep binary run (not the interim 16) showing trace-zero LFD below every law at k = 11, or a different pattern at larger k.

---

## Note: cell A4, subgroup/coset (no verdict change proposed; citation rule only)

**Current verdict:** REF, per the brief.

**Proposed:** no verdict change. README §4: "if its refutation is cited through the fall degree, the coset case of the k + 2 bound is written only, not kernel-checked."

**Action implied:** do not cite the fall degree as A4's refutation until the coset case is kernel-checked.

**Evidence for the existing REF (not fall-degree):** encyclopedia A4 entry (`docs/ic/factor-base-encyclopedia/README.md`, from line 659). A4a: Gröbner time per query 74.7 ms against 30.8 ms for a generic base at 19 bits, 2.4× slower. A4b: exponent 0.97 [0.95, 0.99] at m = 2 and 1.00 [0.98, 1.01] at m = 3, "behaves like a random set of the same size".

**Caveat and discrepancy:** `research/leads_20261112/BRIEFING.md` states the kernel bound LFD ≤ k + 2 holds for "every target" (`SelectorExtension.lfd_le_depth_add_two_of_two_ne`). README §4 says the coset case for A4 is written only. The atlas owner should reconcile these before citing the fall degree for A4. This patch does not resolve it.

**What would reverse it:** an A4 exponent that departs from the random-set behaviour at matched random, or a counterexample to the coset case of the k + 2 bound.

---

## Status

Written as a patch for the atlas owner. Nothing applied, nothing run, no atlas body read, no leaderboard or benchmark row touched. The companion structured file is `research/leads_followup_20261114/item04_atlas_patch/atlas_patch.yaml`. Session logging under `docs/session-logs/` is the main session's job; this item wrote only inside its own directory.
