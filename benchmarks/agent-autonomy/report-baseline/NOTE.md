# Baseline status: partial, not representative

This pre-loop baseline (sessions ended 2026-11-01 to 2026-11-14) holds 3 episodes, not the
planned 25 to 40, and is **not usable as a baseline yet** `[pcc:unverified]`.

- **Selection bias.** The 3 sessions were recorded because their transcripts were short
  enough to read in full; about 15 qualifying sessions are unprocessed (PRs #3405, #3027,
  #3447, #3408, #3731, #3839, #3180, #3245, #3952, #3844, #4150, #4153 and others). Short
  sessions are not a random sample.
- **Cost.** `list_events` pages count raw events before filtering, and most "user" events are
  tool results, so a long session needs 5 to 50 pages.
- **Unaudited.** Interventions were classed by a Sonnet agent; none is audited yet. One case
  ("use Rust", against AGENTS.md "no Python") was classed redirect under the conservative rule.
- **Pending.** Two episodes stay pending until a `reverted` check is recorded after 14 days.

Next: a transcript filter that keeps only human-typed messages, one cheap agent per session,
and a uniform sample by date before this baseline is compared with anything.
