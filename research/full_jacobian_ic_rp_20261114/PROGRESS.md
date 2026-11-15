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
## Amendment 1 (2026-11-15, before any data): field-operation unit

- Old PROTOCOL.md sha256: 565d57faaf2633ecd6522aafb1ae318b3bf9157b30479c97507fc3d26fec50cc
- New PROTOCOL.md sha256: see frozen.sha256 (`sha256sum -c frozen.sha256`); recorded below.
- No `full_jacobian.json` or `run.log` in this directory at amendment time (the run was blocked on the checkpoint gate; no data).
- Code: `CantorTally`, `ec_add_tally`, `TalliedJac`, `TalliedEc`, `rho_full_ec_tallied` in full_jacobian_ic.rs; `--inv-weight` (frozen 10), `--field-microbench`, `field_unit` reports, `verdict` (field) and `verdict_group_ops` in the binary; IC arm rows `lower_bound: true` with `lower_bound_note`.
- Tests: 11 pass (`cargo test --release --lib chained_transfer::full_jacobian`), including `old_counters_replay_the_2026_11_13_p211_row`.
- Smoke runs (output in the scratchpad, not here): `--select-only`; p = 211 and p = 211,419,821 with `--rho-seeds` 2-4 (arms ran; A/B certified, ok=true).
- render.py: field-unit tables, Cantor/E1 per-operation ratio, inversion-weight sensitivity, both verdicts.
- [ ] benchrec init/run, render, RESULTS.md remain blocked as before (checkpoint gate).
- New sha256 (full): 820e4e188369fd668af95483ec934287435b93be6288c6d662b7be3843390899
- Re-frozen once more by the main session (2026-11-15) to correct the smoke disclosure in Amendment 1 (two smokes, at p = 211 and at p = 211, 419, 821); no other change. Hash before that edit: 885cd712dff7016e96c8b43ff73d8e728f04962407926e2d9d9c327d66e65313.
