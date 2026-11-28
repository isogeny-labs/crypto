# Lab-wide work and leads, ranked (scout `scout-repo-wide`, 2026-11-18 ~18:15Z)

Every number here is quoted from a producer record, a PR body or a session log
and is **[pcc:unverified]**. This scout registered no pcc claim and built or
ran nothing. It used no paid compute and caused no remote side effects; it only
made read-only GitHub REST reads. A lead is a hypothesis with a falsification
test, not a result. Accounting or relabelling changes are listed as such and
never as improvements. The graph-IC lane's own leads are in `../scout-graph-ic/LEADS.md`
and are not repeated here.

Sources read: `docs/ACTIVE.md`; `docs/session-logs/2026-11-18.md` (42,350
lines; section heads plus the sections cited); `docs/session-logs/2026-11-17.md`
(pod sections); `docs/session-logs/2026-11-18-hybrid-e8.md`;
`/home/user/cryptanalysis/docs/ACTIVE.md` and `docs/session-logs/2026-11-17.md`
(that checkout is at cfc03a37e, 2026-11-17 10:03 PT, so it may be stale);
`benchmarks/progress/PROGRESS.md`; `docs/ic/LEADERBOARD.md` (section heads and
validation lines); `tools/benchrec.py check`; `research/hky_pod_runs_20261117/`;
`research/representation_leads_20261109/`;
`research/ic_genus2_16_speedup_20261117/P1/RESULTS.md`;
`docs/ic/order-free-hyperelliptic-index-calculus.md`. The REST state and bodies
of PRs #4525–#4756 were read with `gh api`.
**research-kb unavailable**: `scripts/research-kb` exits with "no executable
found", and no MCP server is attached. The lead atlas is also unavailable:
`tools/lead_atlas.py` is absent and no crypto-autoresearcher checkout exists.
No novelty or "not tried" claim is made.

---

## A. Infrastructure blockers. Fixing one unblocks many lanes.

### A1. Repair or pin-exclude the malformed `20261117T0334Z-hec-multi-production-cold.json` (rank 1)
- **State:** `python3 tools/benchrec.py check` still fails on current main. Commands 1 and 2 have no successful integer exit, and every metric has a `None` name, unit, value, better and spread. The record was last touched in e5eedf44a1, "Retain standard-input multi-orbit cold results…".
- **Effect:** `close` and `report` refuse for every record, so 16 records stay `open`. They include `20261118T0302Z-mt-ic-hp-2p32-2p36`, `20261118T0247Z-ksum-sibling-p2`, `20261117T2043Z-genus4-cheap-ic` and `20261118T0302Z-ic-gpu-rootfind-devmap-r3`, and #4738's `20261118T1738Z-tableb-fewcol-stack` is in the same state. #4649, #4638, #4651, #4738 and #4682 all name this record as their blocker. `benchmarks/progress/` is stale on main.
- **Cheapest fix:** the tool's own message offers "invalid/censored or an explicit pinned historical exclusion". The owner of the hec-multi lane marks the record `invalid` or `censored` with its real reason. The original must be preserved.
- **Who:** the owner of the hec-multi record, or the owner. Not this lane.

### A2. AWS credentials and the checkpoint token are dead in agent containers
- **State:** both key pairs return `InvalidAccessKeyId` (`2026-11-18-hybrid-e8.md`). The checkpoint upload token expired on 2026-11-05 (2026-11-18 log, K2 entry). STS rejects the token, so there is no EKS route.
- **Effect:** every run launcher is refused by the checkpoint hook, no S3 raw upload is possible, and no isolab route exists. The hybrid E8 lane is blocked on this, along with the lazy-orbit sweep (#4565) and K2 D3. Every "needs isolab" validation below waits on it too.
- **Who:** the owner, since credential rotation is outside agent authority.

### A3. #4689 removed part of the CI hydration blocker. Re-run the checks it affected.
- **State:** #4689, merged 2026-11-18 06:45Z, maps 615 store digests to Git blobs. The `curve-names`, `claim-corpus` and `validation-gate` workflows can hydrate again. **159 validation-gate entries were never in Git and still need the original store.**
- **Lead:** PRs whose red `names`, `corpus` or `gate` checks were blamed on main should re-run those checks. #4598 (merged red) recorded this. Re-run them before blaming the change itself. The 159 missing entries are a separate repair and need the store, so they wait on A2.

### A4. Main library compile drift (recurring)
- **State:** #4676 reported that the full lib test build at 9ec88b7637 fails with duplicate definitions in `ic_boundary` and `ic_framework`, missing `linalg_mixed` integration, and mismatched F4 Hilbert APIs. #4751 (open, ready) repairs main's check and clippy jobs. #4743 (draft) ports the solver code for "the last four non-compiling targets". The ACTIVE line "Repair the 28 examples/tests/bins" (claude, 2026-11-17) has no PR of its own.
- **Lead:** check whether #4751 and #4743 together close the 28-target list. If they do, tick the ACTIVE line. If not, list the targets that remain.

---

## B. Results never validated or promoted (highest value per hour once A1/A2 clear)

### B1. Table B `tw2l` (C = 2 wwalk): 0.41–0.48x of the ksum5-sib incumbent in counted GAE (#4738, open, ready)
- **Producer result:** 48 targets × 2 rounds, 720/720 verified per curve, all candidate runs replayed. Distance to matched rho falls from 4.6–5.3x to 2.1–2.2x at w = 0. Stacking prune+sib gives 0.89–0.92x of the incumbent. **[pcc:unverified]**
- **Missing:** the validation-gate certificate and validator report, which means there is no leaderboard row. The A/A arm checks determinism only (`seed=run` derives from the shared seed), so no noise floor was measured.
- **Next:** (i) an A/A with independent seeds; (ii) the **`tw2l`+prune+sib stack**, which was measured only separately. Prune+sib shrinks the table build, `tw2l` changes the column count, and the two may compose. This is a local ecbench run with no code change. (iii) A `VALIDATION.json` once A2 clears.
- **Expected value:** the largest measured counted improvement to the prime IC baseline in the window read. It still sits about 1.88x rho at the generic collision floor (producer's statement), so it does not break the √r wall.

### B2. Multi-target IC with huge pages at 2^36 within about 1–4% of tuned rho; IC 0.84–0.88x rho at 2^32 (`research/mt_ic_hp_20261118`, #4638, merged)
- **Producer result:** IC / best rho = 0.988 at T = 16384 and 0.999 at T = 4096 at 2^36 (from 1.175 and 1.109 on 4 KB pages), and 1.043 at T = 1024. At 2^32, IC was 0.84–0.88x rho. These are wall times on one unisolated host. **[pcc:unverified]**
- **Missing:** the record is open (A1), there is no isolated host (A2), no validator has run, and there is no §5a band. It is a wall-time claim, not counted S.
- **Lead:** combine with #4725's batched Montgomery/AVX-512 k-sum (21x per solve on one thread against the library ksum5). The producer's named next lever is the table build (67 ns per entry against 44–45 ns per trial). Both arms must get the same arithmetic upgrade, or rho wins by construction.
- **Cheapest test:** a local A/B of `mt_fieldops` with the #4725 field kernel on both the IC and rho tables, at 2^32 and T = 4096.

### B3. Genus 6–8 hyperelliptic IC with degree-2 place arms below matched rho (PR #4598, merged)
- **Producer result:** best arm / expected matched rho = 0.066 at g = 8, p = 31; 0.064 at g = 7, p = 53; 0.024 at g = 6, p = 211. All solves are certified on planted logs. **[pcc:unverified]** The method is classical (Enge–Gaudry). Below-rho cost at large genus is expected, not a discovery.
- **Missing:** an independent replay, a validator report and a pcc claim, as the session log states. Follow-up protocols F1–F3 (`research/g2plus_followups_20261117/`) are frozen with no completed run. P3 (non-hyperelliptic plane-quintic models) has not started.
- **Why it matters for the goal:** a T-tier "transfer counts when fully charged" route exists only through a cover map, such as GHS or the validated JV weak-class cover sieve. See C3.

### B4. K1 Koblitz rows: the rho reference is understated or ambiguous (accounting, not an improvement)
- `research/koblitz_rho_reference_20261115/` shows v5/strong at 0.079 for n29, rising to 0.96 at n53. Rows below about 2^38 should be reported against min(strong, v5). v5 first needs a per-step jump-index tally, which is a source change. This is an owner decision and is still open (2026-11-18 log, 03:40Z entry). Until it is settled, every K1 "gap to rho" is reported against a reference that is not the strongest.

### B5. 16 open bench records (see A1). Close them in a single pass after A1, using the close commands already in each RESULTS.md.

---

## C. New or reopened technical leads toward the IC baseline or rho

### C1. Put the counted wins on one arm: `tw2l` × prune+sib × fast arithmetic × target-independent hoisting (#4756)
- **Mechanism:** each part reduces a different term. `tw2l` reduces the column and collision floor. Prune+sib reduces the table build. #4756 (draft, unbenchmarked) builds the MITM left-half table once per factor base instead of once per trial, makes the linear algebra incremental, and hoists the sweep index. Their composition has not been measured.
- **Test:** one frozen Table B protocol with 4 arms: incumbent, `tw2l`, `tw2l`+sib, and `tw2l`+sib+#4756. Same targets and seeds as #4738, run on ecbench locally.
- **Falsifier:** the stacked arm is not below `tw2l` alone in counted GAE at all three sizes.

### C2. `genroot` oracle (#4755): price the root-finding and compare it with `rootfind-subtract` in S
- #4755 marks every `genroot` row `partial_counted_cost`, because field multiplications and inversions inside a fiber are tallied but not priced. The next step is the pricing rule: field ops at their GAE fraction, under the §5a band 0, 0.1 and 1. Then compare with `rootfind-subtract` on prime r ≈ 2^20. Until then genroot cannot be ranked.

### C3. Transfer route: degree-2 place arms (B3) applied to the GHS / JV weak-class cover Jacobians
- **Mechanism:** the JV cover sieve has validated-with-caveats rows (`research/jv_sieve_line_vs_rho_20261103/VALIDATION.json`). The cover's Jacobian is hyperelliptic of genus g over a small field. #4598's degree-2 arms gave 11–108x over the lab 2LP arm at g = 6–8.
- **Test:** run the P1 instrument (`research/ic_genus2_16_speedup_20261117/P1/instrument.patch`) at the (g, q) of the smallest validated JV cover row, with the cover map charged as in the JV study. A fully charged gain counts as a T-tier transfer (`docs/ic/SUCCESS_CRITERIA.md`).
- **Risk:** cover genera are tied to n and may be outside the instrument's #J < 2^63 limit. Check the (g, q) first.

### C4. Order-free prime-field hyperelliptic IC (#4676, merged) has no measurement
- It is a native solver with 11 known-answer tests and a pcc claim, `PCC-20261118-c35f94`, standing unverified. No cost or scaling has been measured, and "generic fixed-genus inversion remains open".
- **Cheapest test:** a ladder at g = 3 and 4 over growing p against the `genus_ladder` 2LP arm, in counted group operations.

### C5. `prime_fast_j0` multi-target 4-summand against folded BSGS (#4645): tune B per T
- The log has 1.26x behind at T = 2048 and 1.79–2.31x at T = 4096, with B not tuned per size and one group only (2026-11-18 log, 4-summand entry). The goal is unmet; the producer's own next step is to tune B at T = 2048 and 4096 on a fresh group. A low-cost local lead.

### C6. Kaarelson v2 Theorem 3 coverage bound
- This is already in flight as #4745 (open). The other follow-on, an independent re-check of the paper's Lean engine claim (Thm 2(ii) `subexponentialRate_all`, the disclosed `sorry`), has no ACTIVE line. The 2026-11-18 log says both were "teed up in ACTIVE.md", but neither line is in current `docs/ACTIVE.md`. They were probably lost in a merge. Re-add the Lean audit line.

---

## D. Unfinished threads whose owner has been quiet for more than 24h, or ACTIVE lines that are stale

| ACTIVE line | State found | Action |
|---|---|---|
| Novel radical root finder, d ≤ 4 (claude, 2026-11-17) | branch `claude/radical-rootfind-20261117` last commit 2026-11-17T18:02Z, **no PR** | quiet for more than 24h; needs an owner or a draft PR |
| Boolean FL F4 on matched Semaev systems (#4679) | **merged at 06:42Z with only ACTIVE.md and session-log changes**; no benchmark landed | line unticked; the work was never done |
| ecbench Phase-1 cold IC-vs-rho (#4681) | merged with tracking content only; continued in #4752 (draft, canonical panel) | relabel the line to #4752 |
| Semaev rho parity (#4682) | merged with the RR9 D9 protocol only; "no candidate or measurement exists yet" | open: native Rust kernel candidate, 5 frozen arms on EKS (blocked by A2) |
| Norm-branch GitHub-runner fallback (#4686) | merged with **3 ACTIVE.md lines only** | never executed |
| curve-transfer slice (#4683), yield/rank obligations (#4685) | merged | check the deliverables, then tick or split |
| Prime31 conductor via 3-division (automation, no PR) | #4660 merged 04:37Z with the same scope | likely duplicate; tick |
| Prime31 CM degree gate (#4657) | merged | tick |
| Vieta publication (#4525) | merged 2026-11-17; blocked on official censored close (A1-like) and crypto-lean CI billing lock | owner |
| Root-finding 100x next: lever 1 as an oracle | **no owner tag** | unowned; overlaps #4725 and #4755 |
| Autolab rounds 38–40 | open: second-host replay, `hashed-ecdl-tensor` standalone build (8 modules without `mod`) | quiet since 2026-11-17 |
| Lazy-orbit column sweep (#4565, merged) | blocked: budget headroom, AWS identity, isolab image | owner (A2) |
| K2 decisive D3 | needs a ≥ 24 GB node; D = 5 at k = 7 | protocol draft only |
| Genus 2–16 P3, F1–F3 | frozen, never run | see B3 |
| Hybrid E8 (`claude/ic-lead-hybrid-gd-l7`) | branch active 17:09Z today; #4643 merged as the freeze | **not stale**; blocked by A2 |

Sibling `cryptanalysis` (checkout at 2026-11-17 10:03 PT): the only open
lines are the Xedni `r = 7..9` regime (#1722) and native buffer-state admission
(no PR). Both are blocked on owner or provider admission and on the original
695/701 fixture custody. The checkout may be stale, so these were not checked
against GitHub.

---

## E. RunPod pods reported RUNNING: repository records (read-only; nothing was contacted)

| Pod name | Repository record | Assessment |
|---|---|---|
| `hky-j1` … `hky-j4s` | PR **#4735** (draft, updated 16:57Z today), `research/hky_paper_runs_20261118/pod/` on branch `claude/lfd-ffd-paper-eprint-status-qq8fit`; **no record on main**. J1's k = 14, D = 6 result is landed in the PR (`results/J1/`), but "the degree-7 lift is still running". J2 (k = 14, p = 163841), J3 (k = 15) and J4 (k = 16 with the skip, relaunched on owner instruction) are "still running". | **Owned and active**, not orphaned. **Risk:** the PR states that `pod.py`'s in-pod lifetime cap does not work (PID 1 ignores SIGTERM), so "spend is bounded by explicit termination only". If the #4735 session goes quiet, these pods bill without a cap. Owner check: is J1 needed once its lift finishes? |
| `bl-ladder` (`k7fjlexahsl468`, cpu3m 32 vCPU / 256 GB) | Same PR #4735, `research/binary_falldeg_20261118/pod/LAUNCH.md`. Teardown is written down: fetch `/workspace/out/`, then `pod.py terminate k7fjlexahsl468 --yes`. Amendment A1 estimates are long: "`keep 6` × 3 arms is ~450 pod-hours". | Owned and active. Same uncapped-lifetime risk. |
| `replead-r1-20261116` | **Orphan candidate.** The run card is `research/representation_leads_20261109/RUN_CARD.md`; its protocol PR #4114 was merged 2026-11-15. **No `runs/` directory exists on main**, so the pod's `/work/out/` may hold the only copy. The 2026-11-17 log at 06:02Z records it "idle by the API" and kept only because "its session asked the owner to copy `/work/out/` first". The resolution is logged as "owner decision". No later record exists. | Idle for at least about 36h by the repository record, and still RUNNING. The owner should copy `/work/out/` (or confirm it is not needed), then terminate. The repository says it **should have been deleted after the copy**, and the copy was never recorded. |

Related: network volume `yuzuh9rmej` survives its pods. The 2026-11-17 log says
"decide whether to keep it", and #4742 later used it for its self-terminate
test. The 2026-11-17 log also says ten deleted pods' proxy hosts are "still on
the `isogeny-labs` environment allowlist". #4742 (self-terminating pods) and
#4740 (`rp.sh reap`) would remove the uncapped-lifetime risk above. Neither is
merged.

---

## Ranked summary (expected value per unit of effort; no paid compute assumed)

1. **A1:** unblock benchrec by pinning or excluding the hec-multi record. This frees 16+ records and the progress report. Owner action, minutes of work.
2. **E:** `replead-r1-20261116`. The owner copies `/work/out/` and terminates. This stops an idle charge and saves possibly unique data.
3. **B1/C1:** the `tw2l` stack on Table B (local ecbench, frozen protocol), plus an independent-seed A/A. The largest measured counted gain in the window read.
4. **A2:** rotate AWS credentials and the checkpoint token. This unblocks every isolab, validation, S3 and checkpoint path (owner).
5. **B2:** the multi-target IC at 2^36 with the #4725 arithmetic on both arms. A local A/B first, then isolab.
6. **C2:** a pricing rule for `genroot` fiber root-finding so the oracle can be ranked in S.
7. **C3:** the degree-2 place arms on the JV cover Jacobian (T-tier transfer path). Check the (g, q) feasibility first.
8. **D:** reconcile `docs/ACTIVE.md`. Five merged PRs left unticked lines, three of them merged with only tracking content (#4679, #4681, #4686). The radical root-finder branch has been quiet for more than 24h with no PR. The Kaarelson Lean-audit line is missing.
9. **C4/C5:** measure the order-free HEC IC solver and tune B per T for `prime_fast_j0`. Low cost.
