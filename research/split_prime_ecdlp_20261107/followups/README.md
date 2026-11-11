# Follow-ups to split_prime_ecdlp_20261107: P1, P2, P3 and the Lean obligations

Request (user, 2026-11-07, after #3654/#3655 merged): run every next step of the
study: proposals P1 (Riemann–Roch system with a Dickson-torus tower), P2
(twist-pure coset selection) and P3 (cross-trace covariate test of the seed
statistic) under frozen protocols on toy prime fields, and a Lean statement of the
obstruction.

| item | directory | outcome (frozen rules) | evidence |
|---|---|---|---|
| P1 RR-Dickson torus | `p1_rr_dickson/` | **rule 5**, stage-level positive at m = 3 (RR below T on D and widest at m3 t3, t4 on T24 and T31; D identical with the Kummer tower); rule 6 undecided [pcc:unverified] | [RESULTS](p1_rr_dickson/RESULTS.md), report PDF, record 20261107T2145Z |
| P2 twist-pure coset | `p2_twist_coset/` | **rule 4**, toy-scale gain at stage level; index-calculus gain (c_max/c_mean)^(m−1) ≈ 1.9 / 3.5 at t = 4, decaying with t [pcc:unverified] | [RESULTS](p2_twist_coset/RESULTS.md), report PDF, record 20261107T2144Z |
| P3 seed negative control | `p3_cross_trace/` | **rule 2**, null: 0 of 14 covariate tests significant; split-prime seed closed for the coordinate track at 2^24, 2^28; rho walk found defective [pcc:unverified] | [RESULTS](p3_cross_trace/RESULTS.md), report PDF, record 20261107T2144Z (P3) |
| Lean obligations | `aburan28/crypto-lean` PR #3 | seven declarations, standard axioms only (local build) [pcc:unverified] | `CryptoLean.SplitPrime.Obstruction` |
| Red team (P1, P2) | `red-team-p1-p2.md` | corrections folded into P1 and P2 RESULTS | |

### Round 2 (protocols `*/PROTOCOL-R2.md`, merged in #3693)

| item | outcome (frozen rules) [pcc:unverified] |
|---|---|
| P1 arm F and caps | Control met exactly; RR vs round 7's arm F: **trade at all 9 m = 3 cells** (one degree lower, 2.3–3.0× wider); RR vs T: dominates at t3, t4, trade at t5 |
| P1 m5 t4 (first m = 5 cell with a tower) | **undecided**: RR timed out at 3 h (D ≥ 6), T capped at 13 GiB (D ≥ 9); needs a larger host and about a day |
| P2 unplanted targets | **coset-independence holds exactly**: identical F4 work across cosets, primes and towers; the stage gain holds for the whole relation search |
| P3 rho rerun (walk v2) | **null**: 0 of 7 tests, every CI within ±0.19 %; walk defect resolved (A/A r 0.027 / 0.009) |


### Round 3 (protocol `p1_rr_dickson/PROTOCOL-R3.md`)

| follow-up | outcome |
|---|---|
| P1 m5 t4 on Iso Lab EKS, 56–60 GiB, 24 h | **still undecided** [pcc:unverified]: T capped at 56 GiB after 5.9 h (D ≥ 9, widest ≥ 1,305,345, up from 662,876); all three RR attempts were lost to Spot interruptions (15 min, 2 h 33 min, 52 min) in the degree-7 step and supply no bound. Needs RR on a host that is not reclaimed (On-Demand or a qualified resumable route). |

## Rules shared by the three protocols

- **Frozen before data.** Each protocol is committed before any claim cell runs.
  The instrument commands are pinned in each protocol's "Addendum A: instrument"
  in a second commit, after the builders report and still before any claim cell.
  Smoke tests at small primes and the reproduction of an earlier published
  measurement (P1's round7_rr calibration) are instrument validation, not claim
  data.
- **Counted metrics decide.** Every decision below reads an exact count
  (F4 solving degree, matrix dimensions, relation coverage by full enumeration,
  group operations to a solution checked by k·G = Q). Wall time is recorded and is
  descriptive only.
- **Route: this host, isolation L0, a stated deviation from AGENTS.md §9a.**
  The outcomes are deterministic functions of the frozen seeds, so they do not
  depend on host isolation, and a replay on any host reproduces them exactly;
  the precedent is `research/isogeny_class_ic_yield_20261102` (amendment A1,
  local L0, counted metrics only). Iso Lab would add an image build for three new
  standalone crates without changing any decided number. The experiment-director
  freezes this route in each task card or rejects it.
- **Gates before runs.** `.resume/RESUME.md` filled and pushed
  (`scripts/checkpoint.sh init|push|gate`); one `bench.record/v1` per run
  (`tools/benchrec.py`); task cards compiled by `experiment-director`, run by
  `executor-mechanical`, assessed by `experiment-director` (AGENTS.md "Model
  tiers").
- **Implementation.** Native Rust, standalone crates with their own empty
  `[workspace]` table; msolve 0.6.5 (Debian package 0.6.5-1build2, the version
  round7_rr used) as the F4 engine. No Python computes anything.
- **Scope of every outcome.** These are stage diagnostics (P1, P2) and a
  covariate test (P3) on toy fields. `S` and end-to-end cost stay unset; no cost ratio is reported.
  No leaderboard or benchmark row. No rho comparison except where P3 measures rho
  itself as an outcome.
- **Seeds.** Every seed is the first 8 bytes (little endian) of
  SHA-256("split_prime_ecdlp_20261107/<item>/<arm>/<index>").
- **Sources checked.** KB unavailable in this session (no `research-kb` MCP,
  `scripts/research-kb` has no executable). Paths read are listed per protocol.
