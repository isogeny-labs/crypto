# ECC2K-130 follow-ups, 2026-11-16 — shared lane rules

Owner request: run the three follow-ups from the 2026-11-14 sweep
(`research/ecc2k130_open7_20261114/README.md`) in parallel:
F1 cold SAT solve with warm heuristic state; F2 GF(2^n) w_cal instrument; F3 complete m=83
counted IC row against matched rho.

## Rules (bind every lane)
- **Rust only** for experiment code, harnesses, solvers and result generators
  (AGENTS.md "Implementation language: no Python"). Shell only for thin orchestration.
- **CPU partition (4 vCPU, 15 GB):** F2 owns core 3 exclusively (`taskset -c 3`), because its
  measurement is timing. F1 and F3 run only on cores 0–2 (`taskset -c 0-2`) and never on core 3.
  At most 2 GB RSS per process (`prlimit --as=2000000000`), with `timeout` on every run.
- Counted units decide F1 and F3. F2's quantity is a timing ratio. Its local numbers are
  provisional; the authoritative w_cal needs an isolated host (isolab-eks).
- **No paid compute.** On 2026-11-15 the EKS round was still blocked on fresh live budget.
  No pods, no workflow dispatch, unless the lane finds a recorded live budget and an approved
  route. F3 stops at "ready to launch" otherwise.
- Rust builds use a lane-private target directory under
  `/tmp/claude-0/-home-user/e01afaa4-f4f8-52ad-ba5a-6c307eb9b47e/scratchpad/lanes/<lane>/target`.
- Procedure: prior-art audit first (cite `repo:path`; research-kb and the lead atlas may be
  unavailable, so make no novelty claim). Freeze PROTOCOL.md before any measured run, with
  baseline, arms, positive, negative and A/A controls, prediction, decision rule and falsifier.
  Amendments are appended and dated. Same targets, seeds and instrument on every arm; arms
  interleaved.
- Outputs: PROTOCOL.md, RESULTS.md, results.json, at least one SVG, raw/ with SHA256SUMS.
  If writing Markdown into the repository is refused, write it under
  `.../scratchpad/lanes/<lane>/` and list the paths in the final message.
- Every claim is `[pcc:unverified]`. Run `python3 tools/pcc/pcc.py lint-prose <file>` on
  Markdown you write: CI rejects untagged claim words.
- Never delete or rewrite a file in raw/ to "restart" it in place. Write a new versioned file
  and record the reason in a dated amendment.
- Do not commit, push, open PRs, or edit leaderboards, `benchmarks/README.md` or other lanes'
  directories. The main session commits.
