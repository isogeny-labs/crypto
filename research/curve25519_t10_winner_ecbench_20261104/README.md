# T12: the T10 winner end to end (Curve25519 k = 20, 2026-11-04)

**Class:**
- An end-to-end ecbench measurement: full discrete-log solves, counted
  units, priced across the §5a band.
- An algebraic-track component result. Rho remains ahead, and there is no
  leaderboard row.

Protocol: [PROTOCOL.md](PROTOCOL.md), frozen at `91996ac7ef` before data.
Addendum C (restart-safe chunking) was frozen at `4eddc74edc` before any
chunk data.

## Question

Wave-3 lead T10 modelled the F4 work per solve, columns × per-target cost ÷
coverage, over (t, G, seed). The best configuration was **t = 5, G = 4, seed
1307**: 18.65× below the incumbent (t = 6, G = 4, seed 0) and 9.28× below
the T9 best (t = 6, G = 4, seed 1308). Does that hold end to end?

## How it was run

- Arms:
  - `rho-neg`;
  - `proj-a` (the generic baseline);
  - `pkm-iso-t6-g4` (seed 0, incumbent);
  - `pkm-iso-t6-g4-s1308` (T9 best);
  - `pkm-iso-t5-g4-s1307` (T10 winner).
- The PKM arms all use `pkm-semaev:m=2,mode=full,guess=4`.
- 16 public targets, 2 measured rounds (`alternate_measured`).
- Binary: `/home/user/ctarget/release/ecbench`, hash in
  `ecbench/binary.sha256`.
- The container restarted about every 40–60 min, and ecbench cannot resume a
  session. Under Addendum C the frozen spec ran as **16 complete one-target
  sessions** (`ecbench/chunks/c00`–`c15`, `scripts/chunked_run.py`; chunk c
  uses `target_seed` 2026110411 + c).
  - All 16 are `complete`, with **160/160 runs verified**.
  - Chunk c12 was killed once by a process time limit. It was re-run from
    scratch, and the killed copy is kept in `ecbench/chunks/interrupted/`
    with its `session.json` renamed.
  - The first unchunked attempt never started: it waited on
    `/tmp/crypto-bench.lock` until a restart killed it. It has no records.
- Analysis:
  - `ecbench band` over all 16 chunk dirs (`ecbench/band_pooled.{txt,json}`);
  - paired ratios by `scripts/pooled_compare.py` (`ecbench/pooled_compare.jsonl`).
    It reproduces `ecbench compare` to 6 decimals on the single complete T9
    session.

## Results

S = (counted GAE + w × unpriced native operations) / √r, with √r = 362.2.

| arm | runs | S (w=0) | S (w=0.1) | S (w=1) |
|:--|--:|--:|--:|--:|
| rho-neg | 32/32 | 1.544 | 1.654 | 2.651 |
| proj-a (generic baseline) | 32/32 | 12.633 | 13.625 | 22.554 |
| pkm-iso-t6-g4, seed 0 (incumbent) | 32/32 | 30.906 | 1,735,052.9 | 17,350,250.9 |
| pkm-iso-t6-g4, seed 1308 (T9) | 32/32 | 16.631 | 781,646.7 | 7,816,316.9 |
| **pkm-iso-t5-g4, seed 1307 (T10 winner)** | 32/32 | 26.404 | **100,277.2** | **1,002,534.5** |

Paired ratios, winner ÷ comparator (K = 16 targets, linearised t, 95%):

| comparator | w = 0 | w = 0.1 | w = 1 | targets where the winner is cheaper (w = 0.1) |
|:--|:--|:--|:--|--:|
| seed 0 (incumbent) | 0.854 [0.721, 1.013] | **0.0578 [0.0466, 0.0716]** | **0.0578 [0.0466, 0.0716]** | 16/16 |
| seed 1308 (T9) | **1.588 [1.289, 1.955]** | **0.128 [0.097, 0.169]** | **0.128 [0.097, 0.169]** | 16/16 |
| proj-a | 2.09 [1.78, 2.45] | 7.36e3 [6.04e3, 8.96e3] | 4.45e4 [3.60e4, 5.49e4] | 0/16 |
| rho-neg | 17.1 [13.3, 22.0] | 6.06e4 [4.55e4, 8.07e4] | 3.78e5 [2.79e5, 5.13e5] | 0/16 |

**Band ranks:**
- winner < seed 0 at every weight (band point verdict; the paired w = 0
  interval [0.721, 1.013] includes 1);
- winner vs seed 1308 is unresolved inside the band, because the order flips
  at w = 0;
- proj-a and rho-neg are ahead of every PKM arm at every weight.

## Predictions

| | prediction | outcome |
|:--|:--|:--|
| Q1 | the winner verifies 32/32; the replay audit reports 0 problems | **held**: 32/32; `ecbench verify --replay 1` on each of the 16 chunks: 160 records verified, 16 replays reproduced, 0 problems (`ecbench/verify/`) |
| Q2 | S(winner)/S(seed 0) ≤ 0.15 at w = 0.1 and 1, upper bound < 1 | **held**: 0.0578, upper 0.0716 |
| Q3 | S(winner)/S(seed 1308) ≤ 0.25 at w = 0.1 and 1, upper bound < 1 | **held**: 0.128, upper 0.169 |
| Q4 | the order flips at w = 0: S(winner)/S(seed 1308) ≥ 1.2 | **held**: 1.588 |
| Q5 | every PKM arm stays behind proj-a and rho-neg at w = 0.1 and 1 | **held** |

**Against the T10 model:**
- Winner ÷ seed 0 measured 0.0578 against a modelled 0.054 (1.08× with the
  exact model).
- Winner ÷ seed 1308 measured 0.128 against 0.108 (1.19×).
- At w = 0, winner ÷ seed 1308 measured 1.59 against the columns/coverage
  model's 1.45.

## Reading

- Choosing t and G jointly with the seed, all of them free parameters of the
  PKM isogeny arm, makes the best algebraic arm **17.3× cheaper than the
  incumbent** and **7.8× cheaper than the T9 seed** at w ≥ 0.1. It is
  cheaper on every one of the 16 targets.
- The gain comes from the F4 work per target: t = 5 with G = 4 solves 16
  small branch systems per target for 52,816 mul-adds in total per target
  (measured 51,207 per trial in this session), against about 710,094 for
  t = 6, G = 4.
- At w = 0 (F4 free), the smaller t = 5 base needs more trials per relation
  (columns/covered 0.105 against 0.072), so seed 1308 is ahead there. This is
  the order flip predicted in Q4.

## Costs: per-curve precomputation, charged and amortised

- The seed search for the winner was 2,000 tower builds plus the [r]P
  selector at t = 5: 6.94·10⁶ group operations, which is 1.91·10⁴ in S.
- The T10 G sweep was an engine measurement. It is a design-time cost, not
  part of a solve, and is not charged.

| | S (w=0) | S (w=0.1) | S (w=1) |
|:--|--:|--:|--:|
| winner, search amortised (as measured) | 26.4 | 1.003e5 | 1.003e6 |
| winner, search charged to one target | 1.92e4 | 1.194e5 (0.069× seed 0) | 1.022e6 (0.059×) |

Charged to a single target, the search loses at w = 0 and still wins
clearly at w ≥ 0.1. Seed 1308's own search (t = 6) is 1.35·10⁷ group
operations, so charging both searches favours the winner further.

## Distance remaining

- At w = 0.1 the winner is **6.06·10⁴× behind rho-neg** and 7.36·10³×
  behind proj-a. T9's best was 4.73·10⁵× behind rho-neg on its own targets.
- At w = 0 it is 17.1× behind rho-neg.
- Rho remains ahead on every track.

## Deviations

- Addendum C chunking, as described above.
- `--allow-busy`: the host is shared, and S is a counted unit, so no
  wall-time claim is made.
- Conductor and the AWS CLI are absent, so there is no S3 checkpoint.
- The research-visuals graph and PDF are not yet produced.

## Validator errata (applied)

Independent validation: pass_with_errata (`validator-report.json`,
`replay-certificate.json`: `verify --replay 2` on c00, c09, c12, 6/6
identical, 0 problems; all 160 discrete logs re-verified with the
validator's own curve arithmetic; every band, ratio, interval and distance
reproduced to ≤ 1e-14).

- E1: "16 systems of about 52,816 mul-adds each" corrected; 52,816 is the
  per-target total.
- E2: the search alone is 1.91·10⁴ in S, not 1.92·10⁴.
- E3: the §5a per-arm unpriced native counters are listed in
  `validator-report.json`; the band table above already prices them at
  w = 0, 0.1 and 1.
- E4: the session records `git_commit`/`git_dirty` as null; the binary is
  bound by its hash (`ecbench/binary.sha256`) only.
- E5: the w = 0 "winner < seed 0" verdict is a band point verdict; the
  paired interval includes 1.
- E6: model ratio 1.07× → 1.08× with the exact model.
- E7: no research-visuals graph or PDF for this study yet (disclosed).
