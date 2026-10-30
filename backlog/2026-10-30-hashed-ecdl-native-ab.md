# Native hashed-ECDL performance comparison

Status: blocked on existing S3 checkpoint transport and authenticated native
Iso Lab execution/image route. User request: speed up Certicom challenge work.

The code and bounded correctness receipt are in crypto PR #2891 and its paired
cryptanalysis change. The implementation alone does not complete the requested
performance iteration. Follow research/hashed_ecdl_speed_20261029/PROTOCOL.md
without weakening its baseline/candidate, full-panel, holdout, accounting or
admission requirements. S3 push/gate failed; no experiment has been launched.

Next action: use the existing authenticated runner supplied by the user, push
the complete resume checkpoint and pass its gate, obtain a digest-pinned image
for the native tensor binaries, profile, then run the frozen A/A and matched
A/B panel with replay and durable S3 evidence. Retain all failed runs. Do not
use the deployed ecbench fixture or unisolated local timing as a replacement.
Both challenge solves and the independent general audit remain open.
