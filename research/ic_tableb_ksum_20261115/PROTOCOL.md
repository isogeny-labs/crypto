---
title: "Protocol: k-sum table against the Table B recipe on Table B's own prime curves"
date: 2026-11-15
status: registered before any evidence run (hashes in REGISTRATION.sha256, registered-at.txt)
---

# Protocol: k-sum table against the Table B recipe on Table B's own prime curves

**Track: generic decomposition (bounded by √r; cannot beat rho or BSGS).** Nothing here moves the algebraic track.
Goal (owner, 2026-11-15): lower the counted cost (GAE, S = GAE/√r) of the index-calculus incumbent on at least one
row of tables A–C by a verified, measured margin; outputs unchanged, or changed only under a new method id; stop when
one row improves beyond its A/A noise or three preregistered leads are falsified.

## 1. Lead L1 and why it is the first

Table B (`docs/ic/LEADERBOARD.md`, §18) prices a folded, walked, base-at-the-family-optimum, m = 2 pair-table recipe in
counted GAE on 13 curves. It is the only one of tables A–C in a counted unit (A and C are clock readings, and C is
pre-gate, 32-target and historical). Its rows come from the frozen 2026-09-22 ledger and were never re-measured against
a k-sum table. `research/prime_ksum_pooled_20261029` measured the k = 5 sum table at 0.22–0.70× of a tuned pair-table
incumbent on 16 *fresh* curves (validated with caveats), outside tables A–C. **L1 asks whether that gain holds on
Table B's own curves, against Table B's own base sizes, in the same session.**

*Sources checked (KB review unavailable):* the research-kb MCP server is not loaded and `scripts/research-kb` has no
catalog in this container; the lead atlas (`isogeny-labs/crypto-autoresearcher`, `knowledge/leads/ecdlp/`) is encrypted
(VLT1) and this session holds no vault key, so it was not read. Repository paths used instead:
`research/prime_ksum_pooled_20261029/` (README, PROTOCOL, VALIDATION.json), `docs/ic/LEADERBOARD.md` (Table B, "Phases,
as a share of S", "Prime-field baseline", "Prime-field k-sum tables"), `scripts/build_ic_leaderboard.py` (`ladder_rows`),
`docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json` (the curves), `src/cryptanalysis/ecbench/workload.rs`
(`prime_explicit`, `prime_roster`).

## 2. Design

**Curves** (`prime_explicit`, parameters copied from the Round-5 ledger; `ecbench plan` resolves each to its Table B
ICV1 slug, checked before registration):

| role | Table B row | curve | log₂ r | Table B |F| (signed) |
|:--|:--|:--|--:|--:|
| primary | `icv1-fp20-t1293-3c2999ba` | `bench-20bit` | 20.0 | 102 |
| secondary | `icv1-fp22-tm1267-a5acaf6d` | `generated-22bit-3290411` | 21.65 | 148 |
| primary | `icv1-fp24-t1577-77336155` | `generated-24bit-10935329` | 23.38 | 222 |

**Arms** (one spec per curve; every arm sees the same curve, targets and algorithm seed per round):

- `rho-neg` (reference: matched negation-map rho, A = 2), `rho-neg-aa` (control), `bsgs-neg` (context).
- Incumbents, `ic.pipeline` with `oracle = mitm:negation_folded=1`, `factor_base = prime-abscissa:size=N`:
  - `inc-tb`: N = |F|/2 of Table B's recipe (51, 74, 111);
  - `inc-rec`: the leaderboard's recorded baseline N (45 at 20 bits, 162 at 24 bits; none at 22 bits);
  - `inc-sel` (20 bits, N = 64) and `inc-w0` (24 bits, N = 115): the pooled study's other incumbent cells;
  - 22 bits: `inc-lo`, `inc-hi` at 0.71× and 1.41× of N = 74, so that the incumbent is not mis-sized.
- Candidates, `oracle = ksum:m=k`, base sizes frozen from the pooled study's train selection (selected on curves that
  are not these): `ksum5` A = 9 (20 bits), 11 (24 bits), 10 (22 bits, midpoint, secondary); `ksum4` A = 10, 14, 12.
- `max_trials` as in the pooled study: 1410236 (20 bits), 4243777 (22 and 24 bits; a cap, not a target), 13528399621 for k-sum.

**Replicates.** 3 sessions per curve with different master seeds and target seeds (`make_specs.mjs`), 48 public
targets per curve, 3 rounds, 1 warm-up, order alternating. 9 sessions in all.

**Binary.** One `ecbench` built `--release --locked` from the clean `git archive` of one named `origin/main` commit
(`mksrc.sh`, `build.sh` of `research/wallfix_runpod_20261114/`); its sha256 is the code identity, recorded in every
record. **Route.** isolab-eks is blocked (account budget exceeded); the owner's stand-in is RunPod, so the primary
sessions run on a RunPod CPU pod and every session is replayed with `ecbench verify --replay-all` with the producing
binary on a second host class (this container). The counted cost does not depend on the host; wall time is not claimed.

**Pilot, disclosed.** A 4-target, 1-round local sizing run on the same three curves (different seeds, a binary of main at
#3991) ran before this registration to check that the specs parse, the slugs match and a session takes seconds. Its values are
not used.

## 3. Registered tests (AGENTS.md §5a band: w = 0, 0.1, 1)

Let `I(w)` be the incumbent arm of a row with the lowest pooled mean S at weight w (the strictest incumbent: the choice is
made on the same data, which can only favour the incumbent). `R_w = cost(ksum5) / cost(I(w))`, paired by target and round,
`ecbench compare --weight w --dir <3 sessions>`, 4,000 resamples, seed 20261002.

**Predictions** (from the pooled study, different curves): `R_0` in [0.55, 0.95] at 20 bits and [0.30, 0.60] at 24 bits.

**L1 holds on a row** if the upper end of the 95% interval of `R_w` is below 1 at all three weights, pooled **and** in each
of the 3 replicate sessions alone, **and** the improvement exceeds the A/A noise: with `δ` the largest max/min ratio of an
arm's mean S across the three replicate sessions, over `inc-tb`, `rho-neg` and `ksum5` (same arm, same curve, different
seeds), `1 − max_w R_w(upper) > δ − 1`. (Within one session `rho-neg` and `rho-neg-aa` share seeds, so their counts are
identical and say nothing; the replicate spread is the noise floor.)

**L1 is falsified** if on both primary rows the pooled upper bound is at or above 1 at any weight. One primary row
holding is the goal's done-state; the other is reported either way. The secondary row and `ksum4` are reported, never
used to rescue a failed primary row.

**Also reported:** `ecbench band --ranks` per row (order of `rho-neg`, `ksum5`, incumbent), the gap to matched rho
(`ksum5` S over `rho-neg` S at each weight) as the distance remaining, the ratio against `bsgs-neg`, Table B's recorded
S for the row as a before-mark (different harness; never divided into this session's S).

**Not claimed:** wall time, scaling beyond these curves, an algebraic-track result, a security statement, any ranking against
tables A and C. If L1 is falsified, successor leads L2 and L3 are registered in their own protocols before their data.

## 4. Landing

Evidence directory with `VALIDATION.json` (second-host replay certificate, independent validator report, matched rho
reference with the §5a band); a section of `docs/ic/LEADERBOARD.md` after Table B, with the before-mark, built by
`scripts/build_ic_leaderboard.py`; benchrec record; benchmarks entry; PR with auto-merge.
