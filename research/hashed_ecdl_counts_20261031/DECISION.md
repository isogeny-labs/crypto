# Coordinator decisions: counted-S-only amendment (2026-10-31)

This file holds two decisions, newest first:

- the v2 re-review (**approve, conditional**);
- the v1 review (**revise**), kept unchanged below.

Nothing has run under either version. No result exists and none is implied.

# Re-review of protocol v2: approve, conditional

Decision: **approve (conditional).** Protocol v2 resolves all five required
changes from the v1 review, so the design and decision rule are approved as
frozen. The approval takes effect only after the main session completes the
verification conditions under "Conditions". Until then no session may run
under it. The approval does not authorize wall-time claims.

Reviewed:

- `protocol.json`: `crypto-hashed-ecdl-counts-20261031` v2. protocol_sha256
  `8a794feb…`; freeze.commit `af300fa67`; freeze commit `ab3b148a4` per
  `.git/worktrees/wt-counts/logs/HEAD`.
- `spec-C-tensor-counts-10-14.json` (`4bf13a20…`),
  `spec-C-tensor-counts-16.json` (`1ec5dcb4…`) and
  `spec-B-hashed-native-counts.json` (`9bf4e7b1…`).
- `analysis/decide.py` (`3b6c6990…`), `README.md` and `SOURCES.sha256`.
- Cross-checked against the main checkout's ecbench source (origin/main,
  48c8a4767 or newer):
  - `compare.rs`: `CurveRatio`, `match_pairs`, `ratio_report`, `band_report`
    and `validate_pricing_weight`;
  - `stats.rs`: `Linearised::ci95_log` and `linearised_ratio`;
  - `band.rs`: `BandRank` and the rank verdict strings;
  - `record.rs`: the `Cost`, `Record` and `counts()` fields;
  - `src/bin/ecbench.rs`: the `compare` and `band` CLI flags.

This runtime has no shell. The reviewer did not recompute any hash, did not
run `experiment_protocol.py validate`, could not run git, and could not
allocate a DEC id.

## Required changes from v1: status

1. **Implementable decision rule: resolved.**
   - `decide.py` runs `ecbench compare … --weight w --resamples 4000 --seed
     20261002 --json` at each weight in {0, 0.1, 1}. It reads `curves[]`
     (top-level in `Comparison`, `compare.rs` L373). Every field name it
     reads exists: `slug`, `pairs`, `censored_pairs`, `ci_method`,
     `ratio_b_over_a`, `ci95`, `ci95_bootstrap` and `log2_r`.
   - Primary interval: linearised_t, with the 8 workloads as clusters and
     df 7. `ci95` is `ci95_log` = R·exp(±t·se/R) (`stats.rs` L331-339). So
     cv = ln(hi/R)/t₀.₉₇₅,₇ recovers se/R exactly, and T = ln R/cv is the
     delta-method log-ratio statistic.
   - Cell p = max of p_w over the three weights (intersection-union). Holm
     runs over 32 cells at alpha/(32−k+1).
   - The cluster bootstrap is descriptive only.
   - Band ranks supply only the "order holds" condition. The verdict string
     `"{cheaper} < {costlier}"` and the `curve` = slug key match
     `band.rs` L275-283.
   - The script's hash is frozen in `analysis_plan.script`.
2. **Censored rows out of rankings: resolved, and stricter in practice.**
   - With `--weight`, `match_pairs` (`compare.rs` L622-640) pairs only
     `counts()` rows: non-warm-up rows with status `verified`. Timeout rows
     never enter at a wall-implied S, so `censored_pairs` is 0.
   - A cell with any non-verified row therefore has fewer than 8 pairs. The
     script makes such a cell `incomplete_unknown` on two independent
     grounds: the record census and the pair count.
   - The protocol's `invalidation_rules` and `inadmissible` say this
     explicitly.
3. **Wall-clock timeout: resolved.**
   - Declared in `links.timeout_note`, in C5 and P4 and in the spec
     descriptions.
   - P4's set of complete cells is declared host-dependent.
   - The determinism invalidation is narrowed to S or total_gae differences
     on pairs where both rows are verified or both are exhausted. A status
     mismatch involving a timeout is censoring (`decide.py determinism`).
4. **C timeout: resolved.** Spec C is split again: 900 s on 10/12/14 bits
   and 1800 s on 16 bits, each equal to its original. v1's false claim is
   withdrawn in the protocol and the README.
5. **Forced vs discretionary: resolved.**
   - L2 to L0 is labelled gate-forced.
   - The rounds, warmup and order changes are labelled discretionary, each
     with its rationale. The merge is reverted.
   - These labels appear in the protocol (`changes_from_originals`), the
     README and every spec description.
   - `sample_size.power` is computed by `decide.py power`. I checked it by
     hand: rho cv 0.523, se 0.185, t(0.05/32 two-sided, df 7) ≈ 5.00, giving
     a minimum detectable ratio (MDR) of 2.52 at the first step and 1.55 at
     the last.
   - `passes: false` is stated honestly. C3 and B-source-selectors are
     declared probably underpowered, and `no_ranking` there is declared not
     to be parity.

## Conditions (must hold before any run; otherwise this approval is void)

1. The main session recomputes every entry of `SOURCES.sha256` and the
   protocol_sha256 `8a794feb…`, and they match.
2. `experiment_protocol.py validate` passes on v2.
3. The main session commits this file and the DEC ledger record.
4. The declared `execution_preconditions` hold:
   - checkpoint init, push and gate;
   - enough free disk for both release builds (about 81 MB was free on
     2026-10-31, which blocks both builds);
   - binary sha256 and `rustc -vV` recorded for each build;
   - `ecbench plan` shows 96, 32 and 288 measured executions.

   A mismatch in any of these stops dispatch and needs an additive
   correction. It is not a reason to edit v2.

This is a crypto-repo `experiment.protocol`, not a crypto-autoresearcher
`EXP-*` contract, so `tools/experiment_preapproval.py` has no packet to
check. If an `EXP-*` contract is filed for it, that probe must report
`approvable: true` before dispatch.

## Advisory (no refreeze required; carry into the report)

- **Mislabelled descriptive note.** In `decide.py` `cell_from_compares`,
  `descriptive_note` says "includes exhausted pairs at the S they spent".
  Under `--weight`, though, compare drops every non-verified row, so that
  descriptive ratio covers verified pairs only. It affects no ranking, P1
  or P4. The report must footnote the correct meaning.
- **Status mismatches other than timeout.** `determinism` logs
  verified/exhausted and verified/wrong_answer mismatches on a shared seed
  as `other_status_mismatch` without invalidating the track. The backstop
  is `verify --replay-all`, which must reproduce every row. The report must
  list every `other_status_mismatch`, and it must be empty for a clean
  determinism claim.
- **Worktree hygiene.** `analysis/__pycache__/decide.cpython-313.pyc`
  exists. The worktree appears sparse, with no `.gitignore` in its working
  tree. Stage only declared paths and never commit the `.pyc`.
- **Single round.** At one round, the P2 and P3 slopes rest on 8 targets per
  size. They are descriptive calibration only.
- **Scope.** Every conclusion is limited to:
  - the four 10-16-bit roster curves;
  - these arm parameters and budgets;
  - counted S, at 0, 0.1 and 1 GAE per unpriced op.

  Each row's gap to rho is reported as distance remaining. On 64-bit builds
  lambda ≤ 64, so root_u = 0 and public inputs take the finite-size
  fallback. Nothing here exercises the recursive subexponential kernel.
  `ic.tensor_contraction_v1` is an exponential toy (H1). Native S is a lower
  bound. No runtime, wall-time or speedup number is a result.

## coordinator_decision (v2)

```yaml
coordinator_decision:
  id: "unallocated (no shell in this runtime; the main session allocates it in crypto-autoresearcher with python3 tools/allocate_id.py --next coordinator_decision --date 20261031, confirms it with --check, and files ledger/decisions/DEC-20261031-<tok>.yaml)"
  context: >
    Re-review of protocol v2 of crypto-hashed-ecdl-counts-20261031 (lead
    LEAD-EXO-hashed-factor-base-tensor-ecdl), the counted-S-only superseding
    amendment of the hashed/tensor ECDL vs rho panels, filed after the v1
    'revise' decision. Nothing has run.
  decision: approve
  approval_scope: "design and frozen decision rule of v2; run eligibility subject to the four Conditions in DECISION.md; no wall-time claims"
  target_ids:
    - crypto:research/hashed_ecdl_counts_20261031/protocol.json (v2, protocol_sha256 8a794febbd17fd388ac286ca2df98329e24c5b6696af870b10a07785a306ca95, freeze.commit af300fa67)
    - crypto:research/hashed_ecdl_counts_20261031/spec-C-tensor-counts-10-14.json (4bf13a20a4ae254efd1791c0f652db4e8f9c870e6b19ab8503b5b0f102886afe)
    - crypto:research/hashed_ecdl_counts_20261031/spec-C-tensor-counts-16.json (1ec5dcb475df6fe497b29674a27086b77d0c817221ca2fe0b3c5e496e51d710c)
    - crypto:research/hashed_ecdl_counts_20261031/spec-B-hashed-native-counts.json (9bf4e7b10b8c68437b069ea65c85bd4301b131b42887cd47d863cd26d1c6c917)
    - crypto:research/hashed_ecdl_counts_20261031/analysis/decide.py (3b6c69908ad6da040a214b1a4008807dc70d7a5ecfac30e26ae02b44e4a49519)
  rationale:
    - "All five v1 required changes are resolved (see 'Required changes from v1: status')."
    - "decide.py's ecbench calls and field reads match the source: compare --weight pairs only verified rows (compare.rs L622-640), ci95 is the log-scale linearised_t interval (stats.rs L331-339), band verdicts are '{cheaper} < {costlier}' keyed by slug (band.rs L275-283)."
    - "Censored rows cannot enter any ranking or P1/P4; the wall-clock timeout is declared and treated as censoring; per-execution timeouts equal the originals'."
    - "Power is computed and honestly reported as passes: false; C3 no_ranking is declared expected and not parity."
  evidence_refs:
    - crypto:src/cryptanalysis/ecbench/compare.rs L142-178, L606-689, L726-776, L1017-1050
    - crypto:src/cryptanalysis/ecbench/stats.rs L309-374
    - crypto:src/cryptanalysis/ecbench/band.rs L179-293
    - crypto:src/cryptanalysis/ecbench/record.rs L812-817, L912-914, L979-981
    - crypto:src/bin/ecbench.rs L347-408, L1477-1517
    - crypto:.git/worktrees/wt-counts/logs/HEAD (v2 commits af300fa67 and ab3b148a4)
  conditions:
    - "Main session recomputes SOURCES.sha256 and protocol_sha256 and they match"
    - "experiment_protocol.py validate passes on v2"
    - "Main session commits DECISION.md and the DEC ledger record"
    - "Declared execution_preconditions hold (checkpoint gate, disk for both builds, binary sha256 + rustc -vV recorded, ecbench plan gives 96/32/288)"
  limitations:
    - "No shell: hashes, validate, git state and DEC allocation not checked by the reviewer."
    - "Source cross-checked on the main checkout working tree (origin/main >= 48c8a4767), not byte-verified at 4df337e64; the B binary's ecbench (3ae1ac2bd) compare/band were not re-read."
    - "Not checked whether ic.tensor_contraction_v1 changed between the original freeze 13bbdf52 and 4df337e64."
    - "Power rests on an assumed Rayleigh-like rho cv; tensor and native per-target spread is unknown."
  next_actions:
    - "Main session: verify the four Conditions, commit DECISION.md with only declared paths (not the .pyc), allocate and file the DEC record in crypto-autoresearcher, push and refresh the PR; then free disk, build both binaries and run C-small, C-16 and B under v2 via the executor, then decide.py decide; report counted S only, gap to rho as distance remaining across the 5a band."
```

# Earlier decision on v1: revise

Decision: **revise.** The amendment is not approved as frozen. Nothing has run.
No result exists and none is implied.

Reviewed:

- `protocol.json`: `crypto-hashed-ecdl-counts-20261031` v1, protocol_sha256
  `ee1a52e8…`, frozen at `8a678a6b9`.
- `spec-C-tensor-counts.json`: `3692a289…`, binary from `4df337e64`.
- `spec-B-hashed-native-counts.json`: `4a21dd6c…`, binary from `3ae1ac2bd`.
- `README.md`, `SOURCES.sha256`, and `frozen-source/`.
- Track C originals in `docs/ecbench/specs/`, plus
  `research/ecdl_tensor_ic_speed_20261030/protocol.json` on the main checkout.
- ecbench source at `4df337e64`: `spec.rs` plan expansion, the `runner.rs`
  timeout, `compare.rs` `run_cost`/`ratio_report`, and `band.rs`.

The reviewer did not recompute the hashes; this runtime had no shell.

## What passes

1. **Supersedes rather than edits.** The originals are untouched. The B
   originals are kept byte-identical under `frozen-source/`, and
   `SOURCES.sha256` names every superseded file.
2. **Workloads and arms are unchanged.**
   - B differs from its frozen source in `measurement.{rounds, warmup,
     isolation_required}` only.
   - C keeps the curves, target_seed 20261030, the planted targets, the four
     arms and seed 20261030.
3. **Reference and A/A are kept.**
   - Both tracks keep rho-neg and rho-neg-aa. B keeps native-selectors-aa and
     the rho-neg-v2 positive control.
   - The A/A arms share a seed with their partner. That is correct (`spec.rs`
     L363-378), so in counted S they are determinism controls. The protocol
     says so.
4. **Predictions are frozen before data.**
   - P1 and P2 are carried over verbatim.
   - P3 and P4 are new, and their provenance says no S value was read.
   - warmup 0 under the default absolute seed law gives the measured round
     algorithm-seed index 0. No measured round of the originals used index 0,
     so the new rows do not replay seeds whose S may already exist (smoke
     session ECBS1hd9fc33ad31ac, EKS gh-41008826427-1).
5. **No wall-time claims.** This is in `inadmissible` and in the invalidation
   rules.
6. **Binary source identity is pinned per spec.**
   - C uses `4df337e64`, which is a fast-forward of `48c8a4767` per
     `.git/logs/refs/remotes/origin/main`.
   - B uses `3ae1ac2bd`.
   - The B toolchain fallback is declared before data. The binary sha256 and
     `rustc -vV` are recorded at build time.
7. **Gate-forced change.** L2 to L0 is the change the gate forces
   (MIN_SOLVE_WALL_NS_V3). Running locally is consistent with the counts-only
   request.
8. **Caveat is present.** The root_u = 0 / finite-size fallback /
   exponential-toy caveat is in the specs, the protocol and the README.

## Required changes

File these as protocol v2, refrozen before any data, with new spec hashes if
any spec changes.

1. **Make the decision rule implementable as written.**
   - `ecbench band --ranks` (`band.rs` L256-286) compares point means at each
     weight. It has no interval and no Holm step.
   - `ecbench compare` (`compare.rs` `ratio_report`) reports a fixed 95%
     interval, linearised_t over workloads plus a cluster bootstrap. It has no
     multiplicity adjustment.
   - So "Holm-adjusted interval … (ecbench band --ranks)" has no implementation,
     and `analysis_plan.script` is null.
   - Freeze the exact procedure before data:
     - the per-test statistic and p-value, or the step-down interval levels
       alpha/(32-k+1);
     - which `compare` interval is primary (linearised_t or cluster bootstrap),
       with the resamples and seed;
     - that band ranks supply only the "order holds at 0, 0.1 and 1" condition.
   - Commit the analysis script, or exact commands, with the refreeze.
2. **Keep censored rows out of rankings.**
   - `compare.rs` `run_cost` (L564-584) puts `timeout` rows into the ratio as
     censored pairs, at an S implied by wall time.
   - It puts `exhausted` rows in at the S they had spent.
   - State that no curve-level ratio containing a censored pair is used for
     ranking or for P1/P4. Such a ratio may be shown only as descriptive and
     flagged `incomplete_unknown`. No wall-implied S figure is reported.
3. **State that the timeout is wall-clock.**
   - `timeout_seconds` is a wall-clock `Duration` (`runner.rs` L1245). At L0 on
     a shared host, whether a row ends verified or timeout depends on the host.
     This matters most for B's 30 s native rows.
   - Declare that P4's set of "complete (backend, curve)" cells depends on the
     host.
   - Narrow the determinism invalidation rule:
     - It invalidates a track only when S differs on a pair where both rows are
       `verified`, or both are `exhausted`.
     - A status mismatch involving `timeout` is recorded as host/infrastructure
       censoring. The cell becomes incomplete. That is not a determinism
       failure and not a negative result.
   - Any timeout row is unknown, never evidence that a method fails.
4. **Correct the timeout claim.**
   - The proposal says per-execution timeouts are unchanged. That is false for
     C: `tensor-ic-vs-rho.json` (10/12/14 bits) has `timeout_seconds: 900`,
     and only the 16-bit spec has 1800. The 1800 s in the original protocol is
     the job timeout.
   - Either split spec-C to keep 900 s on 10-14 bits, or declare 900 to 1800 s
     on 10-14 bits as a discretionary pre-data deviation in the spec
     description, the protocol and the README.
5. **Separate the gate-forced change from discretionary ones.**
   - Only `isolation_required` L2 to L0 is forced by the gate. The
     discretionary changes are:
     - rounds: C 6 to 1 on 10-14 bits, B 5 to 1;
     - warmup 1 to 0;
     - merging the two C specs;
     - C order alternate to alternate_measured on 10-14 bits;
     - the timeout in item 4.
   - List these as discretionary in the README "What changes and why", the spec
     descriptions and the protocol, each with its rationale.
   - `sample_size.power.passes: true` has no calculation behind it. Compute it
     (for example, the minimum detectable ratio at 8 clusters, df 7, Holm alpha
     0.05/32), or mark it not computed and rest it on the stated effect-size
     argument. State plainly that C3 (tensor vs ic-mitm) is probably
     underpowered at one round, so `no_ranking` is the expected outcome there,
     not evidence of parity.

## Not required (advisory)

- Restoring C's 6 rounds on 10-14 bits is cheap: about 0.3 h of extra tensor
  cells. It would make C3 and the P2 slope intervals more informative. Reusing
  absolute seed indices 1-5 would overlap the originals' seeds; say how the
  protocol handles that if this is taken up.
- Run track B with no concurrent build. Pinning each track to its own CPU
  (`taskset`) cuts avoidable 30 s timeouts.
- The disk blocker (about 10 GB needed for the two release builds) is an
  execution precondition, not a protocol defect.

## coordinator_decision

```yaml
coordinator_decision:
  id: "unallocated (this runtime had no shell; the main session must allocate it with python3 tools/allocate_id.py --next coordinator_decision --date 20261031 in crypto-autoresearcher, confirm it with --check, and file ledger/decisions/DEC-20261031-<tok>.yaml)"
  context: >
    Counted-S-only superseding amendment of the hashed/tensor ECDL vs rho
    panels (protocol crypto-hashed-ecdl-counts-20261031 v1, frozen 8a678a6b9;
    lead LEAD-EXO-hashed-factor-base-tensor-ecdl). Nothing has run, and the
    builds are blocked on disk.
  decision: revise
  target_ids:
    - crypto:research/hashed_ecdl_counts_20261031/protocol.json
    - crypto:research/hashed_ecdl_counts_20261031/spec-C-tensor-counts.json
    - crypto:research/hashed_ecdl_counts_20261031/spec-B-hashed-native-counts.json
  rationale:
    - Supersession, unchanged workloads and arms, the kept reference and A/A
      arms, pre-data predictions, no wall-time claims and the per-spec source
      pins all pass.
    - The decision rule names tools that do not implement it (band ranks have
      no interval; compare has no Holm step; analysis script is null).
    - ecbench compare puts timeout rows in as censored pairs at wall-implied S,
      which would bring wall time into counted ratios unless the protocol
      excludes it.
    - The per-execution timeout is wall-clock, so a timeout mismatch could
      falsely trip the determinism invalidation and makes P4 completeness
      host-dependent.
    - The proposal misstates C timeouts as unchanged (900 s to 1800 s on 10-14
      bits) and presents discretionary rounds/warmup/merge changes as forced
      minimums.
  evidence_refs:
    - crypto@4df337e64:src/cryptanalysis/ecbench/band.rs L256-286
    - crypto@4df337e64:src/cryptanalysis/ecbench/compare.rs L564-584, L726-765, L1023-1044
    - crypto@4df337e64:src/cryptanalysis/ecbench/runner.rs L1245
    - crypto@4df337e64:src/cryptanalysis/ecbench/spec.rs L357-412
    - crypto:docs/ecbench/specs/tensor-ic-vs-rho.json (timeout_seconds 900, rounds 6, warmup 1)
    - crypto:research/ecdl_tensor_ic_speed_20261030/protocol.json (execution.timeout_seconds 1800 is the job limit)
  limitations:
    - The reviewer did not recompute the hashes or rerun experiment_protocol.py
      validate; this runtime had no shell.
    - Did not check whether ic.tensor_contraction_v1 changed between the
      original freeze commit 13bbdf52 and 4df337e64.
    - Every conclusion of the eventual run is scoped to the 10-16-bit roster
      curves, these arm parameters and budgets, and counted S. root_u = 0, so
      the finite-size fallback runs and the subexponential kernel is not
      exercised. ic.tensor_contraction_v1 is an exponential toy.
  next_actions:
    - "Drafting session: apply required changes 1-5 as protocol v2, refreeze it before any data, update SOURCES.sha256 and the README, rerun experiment_protocol.py validate, and resubmit for Coordinator review."
```
