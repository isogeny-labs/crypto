# Raw evidence

The new raw replay and relation transcripts are retained in
[genus4-audit-evidence.tar.xz](https://chatgpt.com/library?file_id=libfile_fde3_f-cc681a41c7bc8191867a23a7e07119cc_7882a560ed1081919cf1037d42f78352&account_id=personal).
The archive is 175,196 bytes. SHA-256:

`08d46016f4ae98a980b9dc3d43599138caec8ffb1bfe3cd09d05dc1ec9327b82`.

| Member | SHA-256 |
|---|---|
| original-replay.jsonl | 62a0fbf07231dc94b8e8c96f1bc45072c3d25a0e0d84a4ef97c83da6a2e4300f |
| lean-rho-combined-replay.jsonl | bd9d1749308cc334ff2fa38d413453a60ec4ca3e3477699026428719ec7935d4 |
| relations-8209-363-w1.jsonl | 5ca5891c47843e566f08276d356788b55fdccbfb408e2d2c4bd4093d207723d6 |

Extract with `tar -xJf genus4-audit-evidence.tar.xz`, then run
`sha256sum -c MANIFEST.sha256`. The archive contains data only; the Rust
source and compact audit receipts are in this repository. The original
producer input is preserved at its historical Git URL in REPORT.md; the
archive does not overwrite or re-label it.

The large raw transcript is not committed as an oversized JSONL. This
owner-scoped archive is not a public CI artifact-store upload and does not
satisfy the lab's S3/signed-validation gate. No VALIDATION.json or leaderboard
promotion is created. A validator with account access can retrieve the bytes;
others can regenerate them from the pinned source and public frozen input.

## Status boundaries

All 144 original rows replayed with matching retained counters and all scalars
checked independently. All 18 window-free plain-rho rows match the registered
plain walk's retained counts, with zero window compares. One 900-second
censored attempt is retained in out/lean-rho-rest-status.txt; the one missing
case completed under the explicit single-worker retry. See out/CHECKS.md.

PR head advanced to cde94b769c86520803778262cf980e7fe1d0aad9 during this
audit, adding producer B/Q entry points. This audit preserves the original
frozen source identity and makes no claim to have measured the producer's new
Brent-only negation arm or its in-progress scaling study.
