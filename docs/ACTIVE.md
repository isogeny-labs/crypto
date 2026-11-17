# Active goal and items

Owner-pinned work queue. `tools/goal_brief.py` prints this file at the start
of every Claude and Codex session (SessionStart hooks) and for `/goal`
(AGENTS.md, "Goal: what to work on next"). When this file has no
unchecked item, sessions fall back to the newest "Open threads" in
`docs/session-logs/`.

Keep it short: one goal, a few ordered items. Sessions check items off and add
follow-ups here; history belongs in the session log, not in this file.

## Current goal

_(none pinned — set one with `/goal <text>`)_

## Active items (top = next)

- [ ] Repair the 28 examples/tests/bins on main broken by lossy merges (syntax, struct drift, lost library APIs) (claude, started 2026-11-17, branch claude/fastest-grobner-base-algorithm-u74ipw; in progress)

- [ ] Novel radical (closed-form) univariate root finder for low-degree (d≤4) Semaev polynomials over F_p vs Cantor–Zassenhaus: prototype, validate, measure field-op counts (claude, started 2026-11-17, branch claude/radical-rootfind-20261117)

- [ ] Add a Faugère–Lachartre reducer/free-column split to Boolean F4 for Semaev systems (Codex, started 2026-11-17, branch codex/semaev-f4-subset-reducers, PR #4562)

- [x] Freeze and audit #4556 exact-source public B8 RR9 result with native JSON receipt (automation, started 2026-11-17, branch research/prime31-barycentric-evidence-20261117, PR #4558)

- [x] Build TorsionGuard finite-primary-module soundness analyzer (Codex, started 2026-11-17, branch codex/torsionguard-core-20261117, PR #4579)

- [ ] Publish Vieta Rust speed results and resolve official record/publication blockers (codex, started 2026-11-17, branch lane/vieta-speed-20261117, PR #4525)

- [ ] Derive exponent-moving Curve25519 mechanism contracts (Codex, started 2026-11-17, branch codex/curve25519-exponent-ideas-20261117; in progress)
- [ ] Port the linearized two-summand root finder (lfd-summation-systems PR #28) into ic_framework as a decomposition oracle and compare it with rootfind-mitm, rootfind-pairs and matrix-F4 on the same instances (claude, started 2026-11-17, branch claude/index-calculus-root-finding-e95kd6)

- [x] Repair main-side CI breakage left by the 2026-11-16 merges: parity lock and IC reference currency (workflow store pulls deferred until the store is repaired) (claude, started 2026-11-17, branch claude/hyperelliptic-index-calculus-td1fdn, PR #4576)

<!-- One line per task, added as the task's first step with its draft PR:
- [ ] <task> (<runtime>, started YYYY-MM-DD, branch <branch>, PR #<n>) -->
- [x] Gröbner solver shootout on GF(2^n) Semaev systems: local diagnostic, inherited-f4 vs msolve vs CMS (claude, started 2026-11-16, branch claude/fastest-grobner-base-algorithm-u74ipw, PR #4543; CI blocked by AWS/S3 and main breakages)
- [x] Track every task first in docs/ACTIVE.md and a draft PR; roll /goal out to isogeny-labs repos (claude, started 2026-11-17, branch track-first-active, PR #4499)
- [ ] Autolab rounds 38 to 40 (claude, started 2026-11-17, branch ic-autolab-20261117b): round 39 ratchets `tw2l-x20-j16-n2` at k = 28 (0.944, counted GAE; wall time 1.236x slower; validator disposition validated for the counted claim); open: a second-host replay or human validation, a code-level lever beyond the K = 3 collision floor, and the `hashed-ecdl-tensor` standalone build failure (eight modules with no `mod` line in `lib.rs`)

## Blocked / questions for the owner

- Lazy-orbit column sweep vs strong rho (claude, 2026-11-17, branch claude/admiring-gauss-vo7kxf, PR #4565): protocol frozen (`research/ic_lead_chase_20261116/protocol.json`, sha256 19116bb1…, source 3658d3aa26), specs and analysis bound, queue registrations `experiments/queue/EXP-LAZYCOL-N{17,29,31}-001.json` filed `blocked`. Needs: live daily+monthly budget headroom in account 104186811093 (latest readback over the monthly cap; this runtime has no lab AWS identity), approved expspec records, an isolab image built from 3658d3aa26, and a perf-ipc w_cal host.

- Vieta:504/504 primary and72/72 calibration calls completed/replayed. Canonical seed-registration360/60 failures retained; no L2 admission. Official censored-close blocked by unchanged main20261105T0828Z archive command1 lacking successful integer exit. Shared crypto-lean PR26 CI cannot start due account billing lock; no bypass/spend. Full general tower NF instance remains unverified.
