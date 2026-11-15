# PROGRESS (agent U, O3 rerun with r bound to p)

Cargo prefix: flock /tmp/claude-0/-home-user/b3fd4014-06f7-5e0c-8338-36d52644c351/scratchpad/cargo.lock env CARGO_BUILD_JOBS=3 cargo ...
(conductor not installed in this container: `which conductor` fails.)

- [x] read 20261113 PROTOCOL/RESULTS/code
- [x] code (written, rustfmt'd; --select-only also added): `cover_select_min_r` in full_jacobian_ic.rs; `--min-r-frac`, `--max-prime-hops`, `--select-only`, `--protocol` in bin (default unchanged)
- [x] tests (8 pass) / rustfmt; crate-wide clippy -D warnings exit 0 (second try; first failed only in src/bin/ecbench.rs, agent X)
- [x] no smoke of arms; only --select-only (scratchpad/select*.jsonl); F chosen 0.4 (r/p 0.49-0.54 everywhere)
- [x] PROTOCOL.md written + frozen.sha256 (sha256sum -c OK)
- [x] RESUME.md filled; `scripts/checkpoint.sh gate` FAILS: no upload token / AWS profile -> run BLOCKED, reported to lead (STOP per task rule)
- [ ] benchrec init/run
- [ ] add-raw, metrics, render, report PDF, close, report
- [ ] RESULTS.md, lint-prose

Notes:
- PROTOCOL.md FROZEN (sha256 565d57fa...50cc). Selection table from `--min-r-frac 0.4 --stage2 25601,51203,102407 --select-only`.
- render.py adapted from 20261113.
- .resume/RESUME.md held another (finished, wide-sparse-la) effort; copy saved at scratchpad/RESUME.prev.md before overwrite.
- No S3 creds: no ~/.config/checkpoint/upload-token, no CHECKPOINT_UPLOAD_TOKEN, no aws CLI. Expect checkpoint push to fail -> STOP and report (task rule).
- Next (once a checkpoint token exists): see .resume/RESUME.md 'Next steps' (benchrec init -> run -> render -> RESULTS).