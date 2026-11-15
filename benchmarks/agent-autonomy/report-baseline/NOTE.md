# Baseline status: 8 episodes, second-pass labels (owner audit still open)

Pre-loop baseline: sessions whose crypto PR merged 2026-11-01..14 `[pcc:unverified]`.

- **Coverage.** Every eligible session was attempted (no sampling). 17 candidates:
  8 recorded; 6 excluded with a reason (PR number pointed at an unrelated PR: #2994, #3027,
  #3180; no PR found or several PRs: two sessions; one transcript over the 60-page cap, #3731);
  4 not recorded because the batch-A agent could not save inline transcript pages (#4153,
  #4150, #3952, #3844).
- **Result after the second pass (DECISIONS.md D1, D2):** 0 unsupervised successes, 4 fails,
  4 pending (14-day revert window). Fails: 2 by `redirect` (#3723 "use Rust", #3408 agent stuck),
  2 by `nudge` (#3839, #3447: the agent stopped and was restarted). #3405 was first scored a fail
  for "CI not green"; its checks show gate, size and artifacts green before merge, so the episode
  was superseded with the corrected flag and is pending.
- **Labels are unaudited, and the redirects are the uncertain ones.** Several are "continue" /
  "resume" nudges after an agent stopped, or short fragments, labelled redirect by the
  conservative tie rule. The owner audit (every redirect plus a random 10% of other labels,
  seed 20261115) decides these; results are recorded as `--classifier human` interventions,
  which override the machine label.
- **Known gaps:** answers the owner gave through a tool prompt are not human-typed events
  and are not counted; some hook messages on inline pages may be missing (hooks are neutral).

Do not compare this baseline with later weeks until the audit is recorded.
