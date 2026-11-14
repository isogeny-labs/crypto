# ECC2K-130 open-items sweep, 2026-11-14 — shared lane rules

Owner request: chase all seven "what I think is open" items in parallel.
Lanes L1..L7, one directory each under this folder. These rules bind every lane.

## Host and isolation (4 vCPU / 15 GB, shared by seven lanes)
- At most 1 core and 1.8 GB RSS per process; wrap runs in `prlimit --as=2000000000` and `timeout`.
- Primary metrics are **counted units** (field mult/sqr/inv, group adds, SAT conflicts and
  propagations, Macaulay row ops, bit ops). They are invariant to host contention. Wall time
  is a practicality note only and never decides a verdict (seven lanes share the CPU).
- No paid compute, no pods, no EKS (budget exhausted, no AWS credentials in this container).
- Rust builds: `flock $SP/cargo.lock env CARGO_TARGET_DIR=$SP/cargo-target cargo build --release ...`
  with SP=/tmp/claude-0/-home-user/e01afaa4-f4f8-52ad-ba5a-6c307eb9b47e/scratchpad. Never build
  into a repository `target/` concurrently with another lane.
- PyPI packages (e.g. python-sat) may be pip-installed if needed; no building third-party code
  from GitHub sources.

## Procedure (in order)
1. Prior-art audit: read the prior study directories named in your brief (README/PROTOCOL/
   RESULT/REPORT), plus `research/ecc2k130_leads_survey_20261023/leads.json`, and grep
   `research/` and `/home/user/cryptanalysis` for your topic. research-kb and the lead atlas are
   unavailable in this container; say so, do not claim novelty.
2. State the **residual** question your item still leaves open after prior work, or declare
   CLOSED-BY-PRIOR with exact `repo:path` citations and stop there (a valid outcome).
3. Freeze `PROTOCOL.md` before any measured run: question, baseline arm, test arm(s),
   positive control, negative control, A/A or determinism check, counted metric, pre-registered
   prediction, decision rule (IMPROVES / NO-CHANGE / NEGATIVE / INCONCLUSIVE), falsifier.
   Record its sha256 in `RESULTS.md`. Amendments after data are appended and dated, never edits.
4. Run at toy scale (Koblitz y^2+xy=x^3+1 or the binary curves the existing code supports;
   prefer prime extension degree n with 2 primitive mod n, mirroring n=131). Same targets,
   same instrument and seeds on every arm; interleave arms.
5. Write `RESULTS.md`, `results.json`, at least one SVG graph, raw data under `raw/`
   (with `SHA256SUMS`), and `REPORT.md` (PDF if pandoc/typst/chromium works). Code lives in the
   lane directory; to change shared code, copy it into the lane directory.
6. Every claim tagged `[pcc:unverified]`. Gap to matched rho reported as distance remaining,
   priced at 0, 0.1 and 1 GAE per unpriced native op (AGENTS.md §5a) where an e2e number exists.

## Do not
- Commit, push, open PRs, edit leaderboards, `benchmarks/README.md`, or other lanes' dirs.
- Relabel or re-account an existing result and call it an improvement.
- Report a stage ratio as an end-to-end speedup.

## Return (in-message, also as `SUMMARY.md` if writing files works)
Verdict under the frozen rule; key counted numbers with uncertainty; what prior work already
settled (citations); what is genuinely new here; files written; open threads.
