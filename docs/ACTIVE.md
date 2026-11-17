# Active goal and items

Owner-pinned work queue. `tools/goal_brief.py` prints this file at the start
of every Claude and Codex session (SessionStart hooks) and for `/goal`
(AGENTS.md, "Goal: what to work on next"). When this file has no
unchecked item, sessions fall back to the newest "Open threads" in
`docs/session-logs/`.

Keep it short: one goal, a few ordered items. Sessions check items off and add
follow-ups here; history belongs in the session log, not in this file.

## Current goal

Achieve at least a 2x reduction in degree of regularity for the P-256
factor-base relation systems, under a frozen definition and matched workload.
Require exact algebraic certificates and relation replay; input degree,
variable count and final Gröbner-basis degree alone do not meet this goal.

## Active items (top = next)

- [x] Freeze the ideal-module regularity definition and comparison protocol (codex, started 2026-11-17, branch lane/p256-complete-law-dreg-20261117, PR #4590; merged component PR #4567; tracking4555 merged before research publication)
- [ ] Establish a complete-system baseline on the frozen finite depth4 cell, preserving the original depth8 workload (codex, PR #4590; merged component PR #4567; four native cells censored; no admitted full D)
- [ ] Extend retained membership certificates (signed D8→4 and16→5) to complete baseline/candidate relation ideals (codex, PR #4590; merged component PR #4567; full-system certificate pending)
- [ ] Establish a candidate with D_candidate <= D_baseline/2, preserving exact relations and charging auxiliary variables/covers (codex, PR #4590; merged component PR #4567)
- [ ] Validate m=2–7 and record scope, replay, costs and remaining obligations (codex, PR #4590; merged component PR #4567; 48 paired exports/12 planted replays checked; full solve/depth8 pending)

- [ ] Add a Faugère–Lachartre reducer/free-column split to Boolean F4 for Semaev systems (Codex, started 2026-11-17, branch codex/semaev-f4-subset-reducers, PR #4562)

- [x] Freeze and audit #4556 exact-source public B8 RR9 result with native JSON receipt (automation, started 2026-11-17, branch research/prime31-barycentric-evidence-20261117, PR #4558)

- [ ] Build TorsionGuard finite-primary-module soundness analyzer (Codex, started 2026-11-17, branch codex/torsionguard-core-20261117, PR #4554)

- [ ] Publish Vieta Rust speed results and resolve official record/publication blockers (codex, started 2026-11-17, branch lane/vieta-speed-20261117, PR #4525)

- [ ] Derive exponent-moving Curve25519 mechanism contracts (Codex, started 2026-11-17, branch codex/curve25519-exponent-ideas-20261117; in progress)

<!-- One line per task, added as the task's first step with its draft PR:
- [ ] <task> (<runtime>, started YYYY-MM-DD, branch <branch>, PR #<n>) -->
- [x] Gröbner solver shootout on GF(2^n) Semaev systems: local diagnostic, inherited-f4 vs msolve vs CMS (claude, started 2026-11-16, branch claude/fastest-grobner-base-algorithm-u74ipw, PR #4543; CI blocked by AWS/S3 and main breakages)
- [x] Track every task first in docs/ACTIVE.md and a draft PR; roll /goal out to isogeny-labs repos (claude, started 2026-11-17, branch track-first-active, PR #4499)
- [ ] Autolab rounds 38 to 40 (claude, started 2026-11-17, branch ic-autolab-20261117b): round 39 ratchets `tw2l-x20-j16-n2` at k = 28 (0.944, counted GAE; wall time 1.236x slower; validator disposition validated for the counted claim); open: a second-host replay or human validation, a code-level lever beyond the K = 3 collision floor, and the `hashed-ecdl-tensor` standalone build failure (eight modules with no `mod` line in `lib.rs`)

## Blocked / questions for the owner

- Lazy-orbit column sweep vs strong rho (claude, 2026-11-17, branch claude/admiring-gauss-vo7kxf, PR #4565): protocol frozen (`research/ic_lead_chase_20261116/protocol.json`, sha256 19116bb1…, source 3658d3aa26), specs and analysis bound, queue registrations `experiments/queue/EXP-LAZYCOL-N{17,29,31}-001.json` filed `blocked`. Needs: live daily+monthly budget headroom in account 104186811093 (latest readback over the monthly cap; this runtime has no lab AWS identity), approved expspec records, an isolab image built from 3658d3aa26, and a perf-ipc w_cal host.

- Vieta:504/504 primary and72/72 calibration calls completed/replayed. Canonical seed-registration360/60 failures retained; no L2 admission. Official censored-close blocked by unchanged main20261105T0828Z archive command1 lacking successful integer exit. Shared crypto-lean PR26 CI cannot start due account billing lock; no bypass/spend. Full general tower NF instance remains unverified.
