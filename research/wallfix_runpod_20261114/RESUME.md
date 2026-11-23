# Resume: wallfix RunPod A/A+ABAB (2026-11-14)

Owner request: "run on RunPod for now" (isolab-eks blocked: monthly budget 1605.11 > 1300).

- Goal: wall time of three merged WALL-class crypto PRs, arm A = merge^1, arm B = merge commit.
  - #3850: A 5382b439bc, B 43e68a1d56 (specs prime, koblitz, tensor)
  - #3991: A 6745ba0a5f, B 94c682e021 (prime)
  - #4009: A 2c1dc30e16, B 21f21a45a3 (koblitz, tensor)
- Pod: RunPod CPU `bdr-wallfix` (cpu5c, 8 vCPU, AMD EPYC 9575F shared host, 60 GB). Kit:
  `research/runpod_kit_20261112/rp.sh` with the Authorization header taken from `$RUNPOD_API_KEY`.
- Source: `mksrc.sh SHA` = `git archive` of Cargo.toml + every path it references; tarball sha256 in `run.md`.
- Build on the pod: rustc 1.100.0, `cargo build --release --locked --bin ecbench`, shared target dir.
- Run: `run.sh` (session ABAB, 3 rounds, counterbalanced), then `ecbench verify --replay-all` on every session.
- Specs: `specs/*.json` (isolation_required L0; otherwise the director's isolab drafts).
- Record: `benchmarks/records/20261114T1934Z-wallfix-runpod-abab.json`.
- To resume: recreate the pod, re-upload tarballs and scripts, rerun `run.sh` (it skips finished sessions).
- Always `rp.sh delete bdr-wallfix` at the end.
