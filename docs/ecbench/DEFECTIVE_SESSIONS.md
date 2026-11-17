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
- **Defect:** `session.json` claims 7 records and a stop at seq 6, but
  `records.jsonl` holds only seq 0–3. The raw fields, with record statuses as
  the runner wrote them:

  ```text
  session.json   records_written 7, last_seq 6, stop {interrupt, seq 6, signal 15}
                 status_counts {verified: 4, exhausted: 1, timeout: 1, interrupted: 1}
  records.jsonl  seq 0 rho-neg verified, seq 1 inc verified,
                 seq 2 iso-t3 exhausted, seq 3 bits-t3 verified
  ```

  Verify reports four problems: `record_count`, `status_counts`, `last_seq` and
  `stop`. Every check on the 4 records themselves passes, and `records_bound`
  is true.
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

## `research/ecc2k130_followups_20261116/F3-m83-counted/raw/sessions/smoke-k0n61`

- **Session:** `ECBS1hbc0ef04c4f34`, original capsule source commit
  `54ccbfa7ed830a0d0882310f71d49eeb89eaed1a`; original artifact manifest
  `02bdbb73473e56d42dba5269498d4d3dbc031b90`.
- **Defect:** the capsule declares 24 records and SHA256
  `2aa97366f892f7d1a2f4c887e9e5543397ff6d047016048932efaace40e92843`,
  while the manifest retains 16 records (sequences 0 through 15), SHA256
  `5c937b749f9bb20f6541b19c9ed9e21c16f2392e103ee3b47e0995a272c629fb`.
  Native verify fails exactly `last_seq`, `record_count`, `records_hash` and
  `status_counts`. The five original files and producer receipt are unchanged.
  [pcc:unverified]
- **Recovery scope:** reachable Git path history, registered local worktrees,
  canonical and legacy expected-digest object keys, object versions and the
  producing branch checkpoint prefix did not yield the original 24-record
  object. This bounded search does not establish absence elsewhere. See the
  preserved [proposal](defect-proposals/20261116-f3-m83/proposal.json) and
  [open custody record](../../problems/ecbench-f3-original-record-custody.md).
  [pcc:unverified]
- **Citable as:** the 16 retained records only, with this capsule-integrity
  defect disclosed. The declared 24-record completion and any aggregate
  requiring the missing eight records remain unsupported. The historical
  producer receipt is an assertion, not a substitute for those bytes.
- **Disposition:** pinned as defective, never integrity-passed. The repository
  owner explicitly approved the exact proposed disposition in this chat on
  2026-11-16, after reviewing PR #4439 and the five-file/four-failure scope.
  The separate [decision receipt](defect-proposals/20261116-f3-m83/decision.json)
  records the approval digest; the inactive proposal remains historical.
  Every shard continues hashing all five original files and running verify;
  changed bytes or a changed failure set fail CI. Original-data recovery
  remains open.
