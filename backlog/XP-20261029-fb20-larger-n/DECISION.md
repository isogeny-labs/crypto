# Coordinator decision: 20-bit prime-abscissa factor base N > 45 (candidate lane XP-20261029-fb20-larger-n)

**Date:** 2026-10-29, after 21:40Z. **Role:** coordinator. **Scope:** whether to open the
candidate lane "test prime-abscissa N > 45 against N 45 at 20 bits on a fresh holdout, with a
random-base control", taken from the open next step of
`LEAD-ICP-prime-ic-constant-engineering` (crypto-autoresearcher, as corrected in PR #2746).
This decision approves nothing, launches nothing, changes no hypothesis or goal status, and
closes no lane or lead. It reads no XP-20261029-packed-filter P28 record.

## 1. Decision

**Not opened.** The lane as stated duplicates a completed experiment. Its one untested
remainder, an N between 45 and 64, is held. It will be ranked at the packed-filter P32 verdict
decision against the other 20-bit levers that are still open, and measured against whatever the
20-bit baseline is at that point.

### 1.1 The question has already been answered on a fresh holdout

The candidate brief says "Larger N was never tested on a holdout". That was true of
XP-20261016, the earlier study (README line 90). It stopped being true on 2026-10-29:

- **XP-20261029-p20-large-n** asked this question. Its records:
  - frozen protocol: `backlog/XP-20261029-p20-large-n/PROTOCOL.md`, commit 97321bfc4, spec sha256 `d205bb7a…236a2`;
  - report: `research/p20_large_n_20261029/`, PR #2823;
  - session `ECBS1h33c033ce9c88`.
- **Design**, as frozen, before any data:
  - one pre-registered challenger, `n64`, against `inc` (N 45);
  - `n90` and `n32` as descriptive arms, plus `inc-aa`, `rho-neg` and `rho-neg-aa`;
  - 3 fresh 20-bit curves × 32 targets × 6 rounds, giving 576 pairs;
  - run as one isolab-eks job at L2.
- **Results** (as recorded in the main session's log):
  - instrument: audit ok, 4,704/4,704 records verified, 4,032/4,032 replays identical, L2 gate accepted every execution;
  - `n64/inc`: 1.001 [0.979, 1.022] at w = 0, 0.967 [0.946, 0.990] at w = 0.1, 0.834 [0.811, 0.857] at w = 1. The reading is "unresolved inside the band", and N 45 was kept;
  - `n90/inc` (descriptive): 1.276 at w = 0, 1.191 at w = 0.1, 0.852 at w = 1;
  - `n32/inc` (descriptive): 1.188 at w = 0, 1.217 at w = 0.1, 1.331 at w = 1. This replicates XP-20261016 on new curves.

  Sources: `docs/session-logs/2026-10-29.md` lines 456–505 in this worktree.
- **The files are on main.** The main tree listing `/dev/shm/pf-work/main117-tree.txt`
  (lines 806–807 and 133256–133312) holds the protocol, the spec, the README and the analysis
  JSONs.
- **The lead atlas is stale on this point.** `LEAD-ICP-prime-ic-constant-engineering`
  (`/dev/shm/ar-ledger/…/LEAD-ICP-prime-ic-constant-engineering.yaml` lines 14–20) still lists
  "test N > 45 at 20 bits on a fresh holdout" as open. It does not cite p20_large_n.

Re-running a grid of {45, 54, 64, 76} would mean testing N 64 a second time, and N 76 lies on the
side where N 90 already lost 28% at w = 0. A second N-64 holdout chosen after the first one came
back unresolved is a re-test aimed at a different answer. AGENTS.md and PROTOCOL §9 of the earlier
study treat that as inadmissible in spirit.

### 1.2 The 20-bit baseline is about to change

The brief names the incumbent as `prime-abscissa:size=45` + `mitm:negation_folded=1`, which is
the walk-target pipeline.

**Current 20-bit baseline: N 45 + stride-renew, validated, pending merge of #2826; on main the
baseline is still N 45 + walk.**

- **Change pending:** XP-20261029-stride-renew-prime makes `targets = stride-renew` the 20-bit
  target source.
- **Result:** srenew/walk 0.842 [0.807, 0.878] / 0.858 [0.821, 0.897] / 0.926 [0.879, 0.975].
- **Validation:** VAL-20261029-4aed57 (P20 pass).
- **Status:** PR #2826 is open with CI green and waiting on human review. Its leaderboard section
  is not on main (main-session check, 2026-10-29). The section was read from
  `/dev/shm/wt-chain/docs/ic/LEADERBOARD.md` lines 317–332.

The pause holds under either baseline:

- **Against walk** (main today): N 64 has already been tested there and came out unresolved (§1.1).
- **Against stride-renew** (if #2826 merges): the main session examined N 64 + stride-renew and did
  not run it (`/dev/shm/wt-chain/docs/session-logs/2026-10-29.md` lines 809–827). Its reason:
  stride-renew removes a cost that both N values share, so the w = 0 tie stays a tie.

Either way, an N test frozen now would be measured against a baseline that a pending merge can
replace at any moment. This adds to the case for waiting; it does not weaken it.

One more lever has not been measured at 20 bits: the deferred repeat guard
(`stride-renew-deferred-guard`). It became the 24- and 32-bit baseline when paired with a smaller
N (LEADERBOARD.md lines 334–342). The guard work it removes is native (w > 0) relation-phase work,
and that is exactly where larger N gains at 20 bits (w = 1). So the w > 0 trade-off that favours
larger N is expected to shrink once that guard is in the 20-bit pipeline. This is a *derived
expectation*, not a measurement. The consequence: tuning N now could be overturned by the next
20-bit change, so the order should be pipeline first and N second.

### 1.3 The random-base control cannot run, and it does not bear on the N decision

- **No plug-in exists.** `prime_base_for` accepts only prime-abscissa, prime-projected-abscissa,
  glv-orbit, prime-kummer, prime-isogeny, cm-scalar-orbit and known-log
  (`src/cryptanalysis/ecbench/methods.rs` lines 4927–4947 on this branch). Nothing in `src/`
  matches `prime-random`. The control therefore needs code, review, CI and a new image before any
  freeze.
- **What it would answer is a classification question.** Does the base's content matter to the
  pair-table oracle? The stride-7919 control already addresses that at 20 bits:
  - `stride/inc` 0.809 [0.758, 0.868] against `sel/inc` 0.818 at w = 0;
  - `band` ranks stride < sel at every size;
  - "choosing the smallest abscissae buys nothing measurable" (`research/prime_base_oracle_tuning_20261029/README.md` lines 73–77).
- **That question matters to a different decision.** It bears on owner decision D1 in
  `research/prime_ic_program_20261029/DECISIONS.md` (the small-x base counted as algebraically
  defined, tentatively). It does not bear on which N is cheapest. It belongs in its own proposal
  (§4, next actions), not as an arm of an N lane.

### 1.4 Ranking against the in-flight packed-filter lane

| | XP-20261029-packed-filter (in flight) | fb20 residual (N ≈ 54 against the baseline on main at freeze) |
|:--|:--|:--|
| status | P24 done; P32 waiting on RULING-P28 D4 (owner answers, or about 2026-10-30T20:47Z), needs both CodeBuild slots | not frozen; needs a fresh holdout and, for the random control, new code |
| what it decides | whether a probe filter pays once the pair table spills out of L2: a component cost that grows with size | one parameter at one toy size |
| transfer | aimed at 28–32 bits and above | none: at 24 and 28 bits the w = 0.1 selection picked a smaller N, which proved to be relabelling, and dg-small chose smaller N at 24/32 |
| expected effect | preregistered in its protocol | ≤ about 3% at w = 0, *derived* (§3.2), at the edge of resolution |
| class | constant engineering | constant engineering |

The packed-filter lane ranks higher. Both lanes are constant-cofactor work, and the roadmap is
biased toward exponent mechanisms (`docs/target-result-profile.md`), so neither earns a claim on
idle capacity. The focus discipline says that idle parallel capacity does not justify admitting a
lane, and the packed-filter lane is idle now only because it is waiting on the owner.

The residual's information value is small: at best a 2–4% constant at 20 bits that does not
transfer. It also faces a known baseline change (§1.2). And its only scheduling slot, before P32,
can be withdrawn at any moment if the owner answers D4. Admitting it now would add a third live
prime-field 20-bit thread for a result that the next pipeline change could make obsolete.

## 2. What would change this decision

The residual is reranked at the P32 verdict decision. The rerank also fires earlier if either of
these happens:

- a 20-bit deferred-guard result lands;
- the owner asks for 20-bit N tuning.

At the rerank, one of two 20-bit candidates is admitted, or neither. Both would be measured against
the 20-bit baseline on main at that moment. Neither is frozen while PR #2826, or any other
validated 20-bit baseline change, is still waiting to merge (§3.1). The candidates:

- **(i)** a single N challenger, sketched in §3;
- **(ii)** the deferred guard at 20 bits, with N handled as in XP-20261029-srenew-deferred-guard-v3.

Choosing one of them before the other is part of that rerank, not of this decision.

## 3. Sketch for the rerank (NOT approved; input to the P32-verdict ranking only)

The sketch below is written so that, if candidate (i) is admitted at the rerank, the director can
freeze it with no further design. It binds no one until a coordinator approval cites it.

### 3.1 Arms (one session, one spec)

| arm | role | definition | decision-bearing |
|:--|:--|:--|:--|
| `inc` | baseline | **the 20-bit baseline on main at freeze time**, re-run unchanged, copied from the spec of the study that set it on main, not from this text. On main today: `prime-abscissa:size=45`, `mitm:negation_folded=1`, `targets=walk`, `linalg=incremental-gauss`, `max_trials` 1,410,236, `solver=""`, `solver_budget_seconds=0`, timeout 1,800 s. If #2826 merges first: the same, with `targets=stride-renew`. See the freeze precondition below | yes |
| `n54` | the only candidate | `inc` with `size=54` only | yes |
| `inc-aa` | operations A/A | `inc` again; must be exactly 1.000 at every w | gate |
| `rnd45` | base-content control, descriptive | `prime-random:size=45,seed=<per-curve>` with `inc`'s oracle and targets (definition below). Included only if the plug-in has landed, is tested and is in the image at freeze; otherwise recorded as a gap, as XP-20261016 did | never |
| `rho-neg` | matched reference | `rho.negation` | distance only |
| `rho-neg-aa` | A/A | `rho-neg` again | gate |

**Freeze precondition: the 20-bit baseline is settled on main.** No freeze is admitted while a
validated 20-bit baseline change waits to merge. Today that is PR #2826. The two alternatives both
fail:

- a freeze against walk could be overturned by the merge immediately after;
- a freeze against stride-renew would rest on a baseline that is not on main.

Once #2826 merges or closes, `inc` is whatever main then holds. No arm is ever set from an
unmerged study, and `n54` differs from `inc` only in `size`.

**Why a single challenger, and why N 54.**
- **No grid.** A grid with a selection rule needs train → select → holdout, and that does not fit
  one EKS job. Selecting on the holdout would mean selecting on the data that judges the result.
- **Train cannot resolve this.** At 20 bits the per-run CV is about 0.30. Eight-run train cells
  cannot separate a 2–3% difference.
- **Where N 54 comes from.** It is round-half-up of 45·2^{1/4} = 53.5, chosen before any data from
  the p20-large-n descriptive arms, as follows.
  - Fit a quadratic in log₂(N/45) to the w = 0 log ratios of {n32, inc, n64}. That puts the
    minimum at N ≈ 53.5, with depth −2.2%.
  - Fit the same form to {inc, n64, n90}. That gives N ≈ 53.6, with depth −3.1%.
  - The curve is asymmetric (each fit mispredicts the third point by about 0.08 in log ratio), so
    both the depth and the location are uncertain. This is *derived* and labelled as an assumption.
- **Earlier N values are not re-run.** N 64 was tested and N 90 lost. Neither is re-run as a
  candidate.

**Random-base definition** (`prime-random:size=N,seed=S`). This is a new `FactorBaseBuilder`; the
name and seed form are fixed in its PR.
- **Point draw.** For k = 0, 1, 2, …, draw x_k uniformly from F_p with the crate's seeded RNG,
  keyed by (S, curve slug, k).
- **Acceptance.** Accept x_k if x_k³ + a·x_k + b is a non-zero square and x_k is not already in the
  base. Take the class representative of {P, −P} by the same sign convention the negation-folded
  oracle uses.
- **Size.** Stop at exactly N points: one column each, the same column count as the abscissa arm.
  Cofactor 1, so every point lies in the subgroup.
- **Independence.** The seed does not depend on targets or `measurement.seed`. It is fixed per
  curve in the spec.
- **Recording.** The record carries the base's hash, so that replay reproduces it.
- **Result.** The draw is uniform over x-coordinates of affine points. That is what XP-20261016
  §12.2 asked for, and it is distinct from the stride-7919 control, which only displaces the
  abscissa window.

### 3.2 Workloads (fresh, and disjoint from every earlier 20-bit session)

- **Curves.** 4 fresh prime-order curves from `prime_search`, bits 20, seeds `202610302011`–`202610302014`.
  - The director lists their slugs with `ecbench plan` before freezing.
  - The protocol fails closed if any slug, or any seed, appears in `docs/curves/sources/specs.txt`,
    in `docs/curves/sources/ci-registration-20261029.json`, or in any earlier 20-bit study.
  - Exclusions this covers:
    - XP-20261016: train `ECBS1h9fcb65b4770e`, holdout `ECBS1h685271327a8a` on `icv1-fp20-tm565-6d7d1d32`, and its second-host session;
    - XP-20261029-p20-large-n: `ECBS1h33c033ce9c88` on `icv1-fp20-tm1319-e428a267`, `icv1-fp20-tm39-90315544` and `icv1-fp20-tm979-7535758a`;
    - XP-20261029-stride-renew-prime P20: `icv1-fp20-tm327-45c21191` and `icv1-fp20-tm1549-7ec9131a`;
    - the unrun 05:55Z scratch draft (t397, tm557, tm81): excluded, because it was drafted after
      outcome knowledge;
    - the pilot curve `icv1-fp20-tm1247-8feb8b3e`.
  - If a seed collides, step the last digit upward (5, 6, …) and record that in the protocol.
- **Targets.** 64 public targets per curve, `target_kind: public`, target seed `202610302015`.
- **Measurement.** Algorithm seed `202610302019`; 1 warm-up and 6 measured rounds; order
  `alternate` (6 rounds, because 5 rounds are unbalanced under `alternate`); L2.

### 3.3 Pairs, executions and power

- **Pairs.** 256 targets × 6 rounds = 1,536 matched pairs per arm pair.
- **Executions.** 6 arms × 256 targets × 7 rounds = 10,752. That is 2.3 times p20-large-n's
  4,704, and that job finished well within one launcher build.
- **Interval width (derived).**
  - p20-large-n's w = 0 interval for an N change on the same targets had a half-width of about
    2.15% at 96 targets, so SE ≈ 1.10%.
  - The cluster bootstrap resamples targets first, so the half-width scales as √(96/256). That
    gives SE ≈ 0.67% and a half-width ≈ 1.3%.
  - Assumption: the paired dispersion is similar on whichever baseline is on main at freeze time.
- **Power at w = 0,** the binding weight (larger N gains more at w > 0, per p20-large-n):
  - true ratio 0.97: P(upper bound < 1) ≈ 0.99;
  - true ratio 0.98: ≈ 0.85;
  - true ratio 0.985: ≈ 0.6.
- **Resolution.** Effects under about 1.3% are not expected to resolve, and the protocol says so.
  - On walk, the main baseline today, the depth estimate is 2.2–3.1%.
  - On stride-renew, if #2826 has merged by the freeze, the shared fixed cost is smaller. The
    relative depth would then be about 1/0.842 times the walk-based estimate, so 2.6–3.7%.

  Both figures are derived and unverified.

### 3.4 Decision rule (fixed at freeze; the same form as stride-renew §6)

`n54` is an **improvement**, and becomes the 20-bit N, only if all of the following hold:

1. **Instrument.**
   - The audit is `ok` and every measured replay is identical.
   - `inc-aa/inc` and `rho-neg-aa/rho-neg` are exactly 1.000 at every w.
   - The L2 gate admits every execution.
   - The input spec is JSON-equal to the frozen spec.
   - `SHA256SUMS` verifies.
2. **Completeness.** All 1,536 measured `n54` runs and all 1,536 `inc` runs verify `[d]G = Q`,
   with 0 censored pairs.
3. **Ratio.** The 95% upper bound of Σ S_n54(w) / Σ S_inc(w) is below 1 at w = 0, 0.1 and 1.
   The bound comes from `ecbench compare --weight`, a cluster bootstrap over targets, 4,000
   resamples, seed 20261002.
4. **Band.** `ecbench band --ranks` gives n54 < inc on **every** curve, at every w.

| outcome | reading |
|:--|:--|
| 1–4 hold | improvement |
| below 1 at some w, not at others | unresolved inside the band; N 45 kept |
| falls at w = 0 and rises at w = 1 | relabelling; N 45 kept |
| interval includes 1 at every w | null at this sample; N 45 kept |
| upper bound of inc/n54 < 1 at every w | regression |
| condition 2 fails | incomplete, never a win |

Always reported:
- `inc/rho-neg` and `n54/rho-neg` with intervals at all three w: the distance remaining, never a gate;
- `rnd45/inc` (descriptive only), if `rnd45` ran.

No pooled claim with other sizes, and no exponent claim.

### 3.5 Route

- **Job.** isolab-eks through the AWS launcher: one job, one spec, `c8a.2xlarge`, L2 with
  native A/A admission, 6 rounds (≥ 5), the image's ecbench bound by sha256.
- **Cards.** A frozen card from experiment-director, run unchanged by executor-mechanical.
  Results are verified against the job's `SHA256SUMS`.
- **Image.** If `rnd45` is included, the image must be built from a commit that contains the
  merged plug-in, using the commit-image route.
- **Slots.** CodeBuild allows 2 concurrent builds, and every build gets an effective 45-minute
  launcher timeout (RULING-P28-ADDENDUM-1 (B)). The launch is admissible only when all of the
  following hold:
  - the slot check shows 0 builds in progress;
  - no owner answer to RULING-P28 D4 has arrived, because then P32 goes first;
  - launch time + 45 min + 15 min ≤ the P32 earliest launch (about 2026-10-30T20:47Z, using the
    recorded time).

  Otherwise it waits until both P32 jobs are terminal.
- **Duration.** The director sizes `job.seconds` from p20-large-n's `session.json` span, scaled by
  executions, so that `job.seconds + 180` fits within 45 minutes with margin.
- **Launcher-less fallback.** Pre-register it as in D2′: a launcher timeout is infrastructure
  signal, and the job is classified by its own S3 files.

### 3.6 Stop conditions

- **Instrument failure.** A failed audit, a non-identical replay, an A/A not exactly 1.000, an L2
  refusal or a spec mismatch stops the reading. It is an instrument fault, not a result.
- **Host loss.** A pod lost before `job-exit.json` is rerun whole under a new job name, with the
  loss recorded before the relaunch.
- **Infrastructure kill.** Exit 143 or 137 after a complete `session.json`, not the job's own
  deadline: one rerun, with the killed session's records sealed until the rerun has written its
  `job-exit.json` (RULING-P28 process note). Any other non-zero `job-exit.json` is never rerun.
- **Frozen after first execution.** No amendment once the first execution starts.
- **Any outcome other than improvement** ends N tuning at 20 bits for that baseline. No further N
  value at 20 bits is proposed without a new mechanism that changes the per-pair or per-trial cost
  structure.

## 4. Decision record (to archive)

I had no shell in this engagement, so I could neither allocate an id nor compute sha256s. The
session that commits this file must:
- allocate the id with `python3 tools/allocate_id.py --next coordinator_decision --date 20261029`
  and confirm it with `--check`;
- archive the record as `ledger/decisions/DEC-20261029-<tok>.yaml`;
- record this file's sha256 in it and in the session log.

No handoff is created: nothing is assigned by this decision.

```yaml
type: coordinator_decision
id: DEC-20261029-<allocate>
date: 2026-10-29
subject: Candidate lane XP-20261029-fb20-larger-n (20-bit prime-abscissa N > 45 on a fresh holdout with a random-base control) - not opened
context: >
  Candidate taken from LEAD-ICP-prime-ic-constant-engineering's open next step (crypto-autoresearcher
  PR #2746): test N > 45 at 20 bits on a fresh holdout with the random-base control XP-20261016 lacked.
  XP-20261029-p20-large-n (session ECBS1h33c033ce9c88, PR #2823) already tested a pre-registered N 64
  challenger against N 45 on 3 fresh curves x 32 targets x 6 rounds at L2 on isolab-eks: n64/inc 1.001
  [0.979, 1.022] / 0.967 [0.946, 0.990] / 0.834 [0.811, 0.857] at w = 0 / 0.1 / 1, unresolved inside the
  band; n90/inc 1.276 / 1.191 / 0.852 descriptive (PR #2823 merged 05:29:38Z; README numbers confirmed
  by the main session). Current 20-bit baseline: N 45 + stride-renew, validated (XP-20261029-stride-renew-prime,
  VAL-20261029-4aed57), pending merge of #2826; on main the baseline is still N 45 + walk. No prime-random
  factor-base plug-in exists. XP-20261029-packed-filter is in flight, with P32 waiting on RULING-P28 D4.
decision: pause
rulings:
  a: "The lane as stated is not opened: it duplicates XP-20261029-p20-large-n. Re-testing N 64, or adding N 76 on the side where N 90 lost 28% at w = 0, after an unresolved holdout is not admissible as a fresh question."
  b: "Residual (one N between 45 and 64; N 54 derived from the p20-large-n descriptive arms) is held, not approved. It is reranked at the XP-20261029-packed-filter P32 verdict decision, or earlier if a 20-bit deferred-guard result lands or the owner asks for it. It is ranked together with the deferred guard at 20 bits, against the 20-bit baseline on main at that time (inc = the baseline on main at freeze time). No freeze is admitted while a validated 20-bit baseline change (today PR #2826) is still unmerged. DECISION.md section 3 is the sketch for that rerank and binds no one."
  c: "Random-base control: it needs a new prime-random:size=N,seed=S builder (definition in DECISION.md section 3.1) before any freeze. It bears on owner decision D1 (is the small-x base algebraic?), not on the N choice, so it goes to a separate proposal."
  d: "Not a closure: no lead, lane, hypothesis or goal status changes. N 45 remains the 20-bit factor-base size. The target source is walk on main, and becomes stride-renew if #2826 merges; this decision does not depend on which."
target_ids: [LEAD-ICP-prime-ic-constant-engineering, XP-20261029-p20-large-n, XP-20261029-packed-filter]
rationale:
  - "The question was answered on a fresh holdout today by XP-20261029-p20-large-n with a single pre-registered challenger; the lead atlas omits it."
  - "The brief's incumbent (walk targets) is still the baseline on main, but a validated replacement (stride-renew, #2826) is pending merge. Against walk, N 64 is already unresolved. Against stride-renew, the main session's 05:55Z analysis already declined N 64 + stride-renew, because the w = 0 tie is preserved. A freeze now would target a baseline in flux."
  - "The untested deferred guard targets the native w > 0 relation work that drives the larger-N trade-off at w = 1, so N tuning should follow, not precede, the 20-bit pipeline change."
  - "The residual's derived effect is at most about 2-4% at w = 0, at one toy size, with no transfer (24/28-bit smaller-N selections were relabelling). The packed-filter lane targets a size-growing component cost and ranks higher. Idle capacity while P32 waits on the owner is not a reason to admit a lane, and both lanes are constant-cofactor work under the exponent-first bias."
  - "The stride-7919 control already shows that abscissa choice buys nothing measurable at 20 bits (0.809 vs 0.818). The random base is a classification control and needs new code first."
evidence_refs:
  - docs/session-logs/2026-10-29.md lines 456-505 (XP-20261029-p20-large-n freeze, launch, result), this worktree
  - /dev/shm/pf-work/main117-tree.txt lines 806-807, 133256-133312 (backlog/XP-20261029-p20-large-n and research/p20_large_n_20261029 on a main tree listing)
  - /dev/shm/wt-chain/docs/ic/LEADERBOARD.md lines 298-348 (prime-field sections: XP-20261016, stride-renew, deferred guard)
  - /dev/shm/wt-chain/docs/session-logs/2026-10-29.md lines 777-835 (stride-renew result; N 64 + stride-renew not run, with model)
  - /dev/shm/wt-chain/research/prime_base_oracle_tuning_20261029/README.md lines 70-95 (stride control; random base absent; shallow train minimum)
  - /dev/shm/wt-chain/backlog/XP-20261029-stride-renew-prime/PROTOCOL.md sections 3, 6 (curves, decision-rule form)
  - src/cryptanalysis/ecbench/methods.rs lines 4927-4947 (prime factor-base builders; no random base)
  - /dev/shm/ar-ledger/knowledge/leads/ecdlp/ic-prime/LEAD-ICP-prime-ic-constant-engineering.yaml lines 14-20 (stale next step)
  - research/ic_packed_filter_20261029/cards/RULING-P28.md (d), RULING-P28-ADDENDUM-1.md (B), (C) (slot and timeout constraints; no P28 record read)
limitations:
  - "The p20-large-n numbers come from the main session's log entries. The coordinator did not open research/p20_large_n_20261029/README.md or its compare JSONs, which are not in this sparse checkout. The main session confirmed that PR #2823 merged at 05:29:38Z and that the README numbers match."
  - "PR #2826 (stride-renew) is open and not on main (main-session check). The stride-renew figures come from the wt-chain worktree's LEADERBOARD.md and the validator id named there."
  - "The N 54 location and depth come from a two-way quadratic interpolation of three descriptive ratios on the walk baseline. The fits disagree by about 0.08 in log ratio at the held-out point, so this is an assumption, not a prediction with an interval."
  - "The effect of the deferred guard on the 20-bit N trade-off is a derived expectation; nothing at 20 bits measures it."
knowledge_promotion:
  promoted: []
  not_warranted: "no evidence review; a dispatch/ranking decision with no claim"
decided_by: coordinator
decided_at: 2026-10-29
next_actions:
  - "Done before archiving (main session): PR #2823 merged at 05:29:38Z and its README matches the n64/n90 numbers; PR #2826 is open and not on main."
  - "Main session: allocate and check the DEC id; archive this record and DECISION.md as one ledger archive (crypto-autoresearcher PR and crypto branch, merge commits only); append the session log."
  - "Main session (crypto-autoresearcher): correct LEAD-ICP-prime-ic-constant-engineering next_action. Mark 'test N > 45 at 20 bits on a fresh holdout' answered by XP-20261029-p20-large-n (N 64 unresolved inside the band; N 90 worse at w = 0). State the 20-bit baseline as 'N 45 + stride-renew, validated, pending merge of #2826; on main still N 45 + walk', and update it when #2826 merges or closes. Leave the random-base control open, as needing a prime-random builder. Name the residual and the 20-bit deferred guard as rerank items. Run lead_atlas.py --check."
  - "Coordinator, at the XP-20261029-packed-filter P32 verdict decision: rerank the 20-bit residuals ((i) a single N challenger per DECISION.md section 3; (ii) the deferred guard at 20 bits) and admit at most one, or record that neither is admitted."
  - "Idea-generator, when next dispatched on ICPERF or the D1 classification: propose the prime-random:size=N,seed=S builder with a base-content control at N_inc across 20-32 bits, as an implementation plus measurement proposal. Not dispatched by this decision."
```
