# Native hashed-ECDL performance comparison

Status: S3 checkpoint upload/readback verified on 2026-10-30. Native Iso Lab
image/execution, profiling and the admitted comparison remain pending.
User request: speed up Certicom challenge work.

The code and bounded correctness receipt are in crypto PR #2891 and its paired
cryptanalysis change. The implementation alone does not complete the requested
performance iteration. Follow research/hashed_ecdl_speed_20261029/PROTOCOL.md
without weakening its baseline/candidate, full-panel, holdout, accounting or
admission requirements. S3 push/gate failed; no experiment has been launched.

Next action: verify the native image/submission route, restore/pass the gate on
the execution host, obtain a digest-pinned native tensor image, profile, then run
the frozen A/A and matched A/B panel with replay and durable S3 evidence.
Retain all failed runs. Do not
use the deployed ecbench fixture or unisolated local timing as a replacement.
Both challenge solves and the independent general audit remain open.

2026-10-30 follow-up: the native `hashed-ecdl-panel` preparer now specifies all
60 development and 24 holdout inputs, with independent bounded curve/orbit
checks and expected scalars kept outside solver inputs. See
[the preparation and policy addendum](../docs/hashed-ecdl-panel.md).
Manual panel preparation is implemented; no scientific panel was executed.
The checkpoint gate still reports missing upload transport. Native image
construction, profiling, phase accounting and admitted matched measurements
remain pending, with frozen evidence and solve/audit requirements unchanged.

2026-10-30 03:13 UTC update: checkpoint upload succeeded. Downloaded archive,
resume notes, all five payload files, both release binaries and 76 source Git
blobs verified; the checkpoint gate passed. This supersedes the earlier upload
blocker, preserving its historical failures. See
[the immutable S3 receipt](../docs/hashed-ecdl-s3-checkpoint.md).
Image/execution and measurement admission remain separate requirements.

2026-10-30 hash-prime follow-up: checked auxiliary prime facts now survive curve
construction and are reused by factor-base setup/replay. An explicit fixed s
in the source interval avoids uncertified random-modulus setup when supplied
with its proof. Official p/r/target/s preflight passes the original work policy;
the default candidate stream, exact fallback and recursion guard are preserved.
See [the new correctness report](../research/hashed_ecdl_hash_prime_20261030/README.md).
The new fixed-modulus input is explicit and must not silently replace the frozen
default comparison panel. No runtime improvement is established.
