---
title: "Protocol: k-sum table against the Table B recipe on Table B's random-binary curves"
date: 2026-11-15
status: registered before any study data (hashes in REGISTRATION.sha256, registered-at.txt)
---

# Protocol: k-sum table against the Table B recipe on Table B's random-binary curves `[pcc:unverified]`

**Track: generic decomposition (bounded by sqrt(r); cannot beat rho or BSGS).** Nothing here moves the algebraic track.
Goal (owner, 2026-11-15): lower the counted cost (GAE, S = GAE/sqrt(r)) of the index-calculus incumbent on at least one row of
tables A-C by a verified, measured margin. This study is the binary-field twin of `research/ic_tableb_ksum_20261115`
(prime rows). The k-sum oracle runs on binary curves since commit d87eb39548 (`KSumOracle` generic over the group).

## 1. Lead L1 (binary) and sources

Table B (`docs/ic/LEADERBOARD.md`) prices a folded, walked, base-at-the-family-optimum, m = 2 pair-table recipe in counted GAE on
13 curves, 5 of them random-binary (n = 15, 18, 21, 24, 27; frozen 2026-09-22 ledger). **L1 asks whether a k = 4 or k = 5 sum table
costs less than the strictest pair-table incumbent, measured in the same ecbench session on the same curves, targets and seeds, at
every pricing of unpriced native work in the §5a band.** The prime twin found 0.43-0.69x on three prime rows (unverified).

*Sources checked (KB review unavailable; the research-kb MCP server is not loaded here, and the lead atlas is encrypted and unread):*
`research/ic_tableb_ksum_20261115/` (PROTOCOL, README, make_specs, analysis, band, results), `docs/ic/LEADERBOARD.md` Table B,
`docs/ic/leaderboard.json` (char2 rows), `docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json` (curves),
`src/cryptanalysis/ecbench/methods.rs` (`binary-subspace`, `binary-multiplier` factor bases), commit d87eb39548.

## 2. Design

**Curves** (`binary_explicit`, parameters from the Round-5 ledger, modulus = 2^n + sum 2^t over `polynomial_low_terms`; `ecbench plan`
resolves each to its Table B ICV1 slug, checked before registration, `curves.mjs`):

| role | Table B row | curve | log2 r | Table B recipe columns (dim) | Table B S (rho S) |
|:--|:--|:--|--:|:--|:--|
| secondary | `icv1-f2m15-t131-ed7256f0` | random-binary-n15-b524b | 13.99 | 15 (5) | 26.31 (2.01) |
| secondary | `icv1-f2m18-t393-d2c04152` | random-binary-n18-b6507 | 15.00 | 15 (5) | 21.30 (2.01) |
| primary | `icv1-f2m21-t2591-de54d72e` | random-binary-n21-b1b6f3b | 20.00 | 31 (6) | 24.39 (1.36) |
| primary | `icv1-f2m24-tm3047-2d0c0e8c` | random-binary-n24-b5fc9da | 21.00 | 67 (7) | 27.24 (1.18) |
| primary | `icv1-f2m27-t12543-569dca8b` | random-binary-n27-b845462 | 24.41 | 127 (8) | 36.93 (0.97) |

Table B's S values are before-marks from another harness and pricing pins; they are never divided into this session's S.

**Arms** (one spec per curve and replicate; same curve, targets and algorithm seed per round for every arm):

- `rho-neg` (matched reference, `rho.negation`), `rho-neg-aa` (A/A control), `bsgs-neg` (context).
- Incumbents, `ic.pipeline`, `oracle = mitm:m=2,negation_folded=1`, `targets = walk` (default):
  `inc-tb` = `binary-subspace:dimension=D` with D the dimension whose column count matches Table B's recipe (table above);
  `inc-n<N>` = `binary-multiplier:size=N,mult=1` at three sizes bracketing the sizing pilots' optimum (tuned incumbents):
  n15 {15, 18, 21}; n18 {21, 26, 30}; n21 {43, 62, 87}; n24 {56, 67, 80}; n27 {127, 150, 178}.
- Candidates, `oracle = ksum:m=k`, `binary-multiplier:size=N,mult=1`: `ksum4` N = 7, 7, 10, 12, 16 and `ksum5` N = 7, 7, 9, 9, 12
  (n15, n18, n21, n24, n27), frozen from the sizing pilots below. Primary candidate: `ksum5`; `ksum4` is a second candidate
  tested with the same rule and reported, never used to rescue a failed `ksum5` row.
- `max_trials` 13528399621 on every IC arm (a cap, as in the pooled study).

**Replicates.** 3 sessions per curve (`B<bits>-r<1..3>`), 48 public targets per curve, 3 measured rounds, 1 warm-up, order
`alternate`, distinct master and target seeds per session (`make_specs.mjs`): 15 sessions. One session at a time, `--cpus none`,
isolation L0 (counted cost only; wall time is not claimed).

**Binary.** `/home/user/crypto/target/release/ecbench`, `cargo build --release --locked --bin ecbench` at commit
2476332d22 (tree clean); sha256 in `BINARY.sha256`. **Route.** Local, at the experiment director's explicit instruction (the default
isolab-eks route is not used; counted cost does not depend on the host). Each session is replayed by `ecbench verify --replay-all`
with the same binary on the same host (not a second host class; a weaker check than the prime twin's).

**Sizing pilots, disclosed.** Before registration, three sizing runs were made with different target and master seeds
(`pilot/`): (a) the main session's single-curve n24 run (24 targets, 1 round; incumbent sizes 48-137, `ksum:m=3/4/5`); (b) pilot 1
(`pilot/make_pilot.mjs`: all five curves, 24 targets, 1 round, grids of incumbents 0.5-2x Table B columns, ksum4 N 5-16,
ksum5 N 5-14); (c) pilot 2 (`pilot/make_pilot2.mjs`: refined grids, 48 targets, 2 rounds). They ran without a benchmark record
(sizing only; no row, claim or leaderboard entry uses them); their band outputs are in `pilot/`. Pilot values are not study data.
Because the sizes were chosen by minimum observed S on a pilot, the chosen sizes carry a winner's-curse advantage of order of
the pilot noise (a few percent) for both the incumbents and the candidates.

## 3. Registered tests (AGENTS.md §5a band: w = 0, 0.1, 1 GAE per unpriced native operation)

Let `I(w)` be the incumbent arm (`inc-tb` and the three `inc-n*`) with the lowest pooled mean S at weight w (the strictest
incumbent; chosen on the same data, which can only favour the incumbent). For replicate `i` let `I_i(w)` be the cheapest
incumbent in that replicate alone. For a candidate `c` in {`ksum5`, `ksum4`}, `R_w = cost(c) / cost(I(w))`, paired by target and
round, via `ecbench compare --dir <3 sessions> --weight w --resamples 4000 --seed 20261002` (pooled), and per replicate
`cost(c) / cost(I_i(w))` from that replicate's session alone. The tested upper end is the larger of the linearised-t 95% interval
and the bootstrap interval printed by `compare`.

**L1 holds for a candidate on a row** if the upper end of the 95% interval of `R_w` is below 1 at all three weights, pooled **and**
in each of the 3 replicates, **and** the gain exceeds the A/A noise: with delta the largest max/min ratio of one arm's mean S
(w = 0) across the three replicate sessions, over `inc-tb`, `rho-neg`, `ksum5`, `ksum4`: `1 - max_w R_w(upper) > delta - 1`.
(`rho-neg` and `rho-neg-aa` share seeds within a session, so their counts are identical and carry no noise information.)

**L1 is falsified for a candidate** if on every primary row (n21, n24, n27) the pooled upper bound is at or above 1 at some weight.
One primary row holding is the goal's done-state; all rows are reported either way.

**Predictions** (from the pilots, which are not independent of the sizes): `R_0` of `ksum5` about 0.65 (n21), 0.65 (n24), 0.45
(n27); about 1.0 or above on n15 and n18, where the pilots show no gain. A prediction is held if the observed `R_0` lies within
+/-0.25 of it.

**Also reported:** `ecbench band --ranks` per row (order of rho-neg, candidates, incumbents), the gap to matched rho
(S of the candidate over S of `rho-neg`, at each weight) as the distance remaining, the ratio against `bsgs-neg`, the ratio of
`inc-tb` to the strictest incumbent (how far Table B's recipe base is from the tuned incumbent), Table B's recorded S as a
before-mark.

**Not claimed:** wall time, scaling beyond these curves, an algebraic-track result, a security statement, any ranking against
tables A and C, any leaderboard row (no `VALIDATION.json`, no second-host replay certificate, no independent validator: the
validation gate is not met by this study).

## 4. Landing

Study directory, `benchmarks/20261115-tableb-binary-ksum/` (report.pdf, graph SVG and PNG), `benchmarks/records/` record
(`python3 tools/benchrec.py`, raw data in Git, S3 upload recorded as not done where it fails). No push, no leaderboard or
`benchmarks/README.md` change from this session.
