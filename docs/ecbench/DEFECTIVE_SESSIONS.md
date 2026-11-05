# Defective committed ecbench sessions

Committed ecbench sessions are evidence and are never edited. When one turns
out to carry an integrity defect that cannot be repaired from original bytes,
the owner may disposition it here instead of withdrawing it. Its row in
[`DEFECTIVE_SESSIONS.tsv`](DEFECTIVE_SESSIONS.tsv) pins the sha256 of all five
capsule files (`session.json`, `plan.json`, `records.jsonl`, `spec.json`,
`host.json`) and the exact set of problem checks `ecbench verify --replay 0
--allow-interrupted` reports. `.github/scripts/ecbench-audit-sessions.sh` still
runs verify on every pinned session, in every shard. The row passes only if
verify fails with exactly those checks, so a changed byte, a new problem or a
vanished one fails CI.

A pinned session is **not integrity-passed**. Cite it only as its note below
allows. A row needs the owner's approval, recorded in its note.

## `research/curve25519_algebraic_ladder_20261102/sessions/smoke-k20`

- **Session:** `ECBS1h928331f3e6f3`, committed in 1f6c453e (2026-11-02,
  "research: algebraic-ladder sizing smoke, Q-D stage harness (Amendment 2)").
  It is an interrupted (SIGTERM) one-target timing smoke, with 8 executions
  planned.
- **Defect:** `session.json` says `records_written: 7`, `last_seq: 6`, status
  counts `{verified: 4, exhausted: 1, timeout: 1, interrupted: 1}`, and a stop at
  seq 6. `records.jsonl` holds 4 records, seq 0–3: `rho-neg` verified, `inc`
  verified, `iso-t3` exhausted and `bits-t3` verified. Verify reports four
  problems: `record_count`, `status_counts`, `last_seq` and `stop`. Every check
  on the 4 records themselves passes, and `records_bound` is true.
- **Cause, as far as the bytes show:** the runner (`runner.rs`) appends and
  syncs each record before it increments `records_written`, so records 4–6 were
  on disk at some point. At finalisation it hashes `records.jsonl` as it then
  stands into `records_sha256`. The committed `session.json` pins
  `121905ec…69fe50`, which is the sha256 of the 4-line file in git. The file had
  therefore already lost records 4–6 on the producer's host before the session
  was sealed. How it lost them is not recorded.
- **Recovery attempted (2026-11-05), none found:**
  - Git: only 1f6c453e ever wrote this `records.jsonl`, and no `exec/`
    sidecars were committed.
  - S3 bucket `setec-astronomy-artifacts`: the producing branch's resume
    checkpoints (`checkpoints/crypto/claude__curve25519-index-calculus-pw5d6r/`,
    01:00 and 01:36 UTC) predate the ladder run and do not contain the session.
    No key under `runs/`, `lab/`, `lab-results/`, `validation/`, `reports/` or
    `research/` names the study, the session id or the producing branch.
- **Citable as:** the 4 sealed records only, as an interrupted prefix of its
  plan. This is what the study's own commit message and PROTOCOL Amendment 2
  rely on ("interrupted after 4 of 8 runs"). The counters for seq 4–6 are
  unverifiable and must not be cited.
- **Disposition:** pinned as defective. Approved by the repository owner
  (aburan28) on 2026-11-05, choosing this over withdrawal or leaving
  `evidence_audit_pr` red.
