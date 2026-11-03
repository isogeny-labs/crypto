---
name: review-run
description: Review another runner's experiment from its experiment.export/v1 bundle, without the producer's session, and file a write-once experiment.review/v1 record of checks, observations, per-claim judgments, open questions and a recommendation. Pointer to the canonical skill in aburan28/crypto-autoresearcher; here a review is an input to the validation gate and the PR review, never a row, a leaderboard change or a claim.
---

# Review a run (pointer)

The canonical skill lives in `aburan28/crypto-autoresearcher`:

- skill: `.claude/skills/review-run/SKILL.md`
- schema: `schemas/experiment-review.v1.json`
- tool: `tools/experiment_protocol.py review new|validate`

Read the canonical `SKILL.md` and follow it. This pointer adds no rule and
lowers none; where it and the canonical skill disagree, the canonical skill
wins.

## In this repository

- Replay with the native instruments: `ecbench verify --dir <session>
  --replay K --exit-code` from a binary built at the session's commit
  (`ecbench-independent-runner`), `scripts/audit_runs.py` for a nonce-drawn
  sample, `scripts/validation_gate.py evidence <dir>` for a row's three
  artifacts. A replay writes only under the review directory.
- Apply AGENTS.md §5a: price every arm's uncharged counters (they are in the
  export's `measurements.counters`) at 0, 0.1 and 1 GAE and rank only where
  the order holds; classify the change as advance, engineering, relabelling
  or accounting (§3) in the judgment's basis.
- File the record under `<study>/reviews/REV-*.json` on your own branch and
  open or refresh a PR. The validation gate still needs its replay
  certificate, validator report and matched rho reference
  (`docs/VALIDATION_GATE.md`); a review may supply the validator report's
  recompute, and says so, but never replaces the gate.
- Never edit a session, a certificate, a benchmark row or a leaderboard
  row from a review.

```sh
A=../crypto-autoresearcher/tools/experiment_protocol.py
python3 $A review new research/<study>/export.json --role validator --runtime <r> --model <m> --session <s>
python3 $A review validate research/<study>/reviews/REV-*.json
```
