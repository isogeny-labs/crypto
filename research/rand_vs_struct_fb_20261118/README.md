# Random vs structured factor base: does structure lower the fall degree, or only the cost of writing the base down? (2026-11-18)

**Every number here is `[pcc:unverified]` pilot data.** No pcc claim, no
registered study result, no leaderboard or `benchmarks/` row. Small prime, m = 2,
Kummer tower, local 4-vCPU box. Preregistration frozen before the data:
[`PREREGISTRATION.md`](PREREGISTRATION.md) (commit `8ced8eddb6`),
[`protocol.json`](protocol.json). Raw rows and per-run load in [`raw/`](raw/)
(`raw/rows.jsonl`, `raw/progress.txt`, `raw/flat.tsv`). Binary SHA-256:
[`binary.sha256`](binary.sha256). Figure: [`fig_falldegree.svg`](fig_falldegree.svg),
[`fig_falldegree.pdf`](fig_falldegree.pdf).

## Question

We have deep last-fall-degree (`d_F`) data for the structured `2^k`-subgroup
("tower") factor base, but no clean random baseline. The one prior random run
(`research/cover_image_fb_20261112/`) was **degree-floored**: a random base of
size `|F|` expressed by the single membership polynomial `∏_{f∈F}(x_i − f)` has
degree `|F|`, so its F4 solving degree `D` is floored at `≈ |F|+2` regardless of
any summation structure. **Does the structure of the factor base lower `d_F`, or
does the structured base only win because it is cheaper to write down?**

## Instrument and arms

`examples/pkm_tower_pilot.rs` (built by [`harness/`](harness/); binary SHA in
`binary.sha256`). Same prime P1 = 2013265921, same `S₃` summation, same engine per
column, Kummer tower, m = 2. Three arms, each a factor base of the **same size**
`|F| = 2^t` at every `t`:

- **tower** — structured `2^k`-subgroup base, membership = the tower **chain**
  (`m·t` low-degree relations).
- **randbase** — random base of the same size, membership = one univariate
  polynomial `∏(x_i − f)` of degree `|F|` (two unknowns).
- **null** — random coefficients on `S₃`'s support + the **same tower chain**
  membership (isolates the summation polynomial from the membership presentation).

**Metric.** Primary, same instrument on every arm: F4 solving degree `D`
(`solving_degree_max`). True index-calculus `d_F` (HKY closure `last_fall`, the
degree at which `1 ∈ V_{F,c}`, on inconsistent random targets) is a cross-check on
the two arms that admit it. **The HKY engine cannot express randbase** — its ring
is the tower quotient with `n − m·t` free variables and randbase has `n = 2`
non-tower variables, so the run panics (`HkyRing::new` underflow, verified exit
101). That a random base has no tower-quotient presentation is part of the
asymmetry under test; randbase's `d_F` is therefore read as F4 `D`, the one
instrument that can represent it. Across the two arms that admit both engines,
**F4 `D` equals HKY `d_F` exactly** (table below), so `D` is a faithful `d_F`.

## Results — matched-size table

`|F| = 2^t`. Degree shown is F4 `D` and, where available, HKY `d_F` (identical).
randbase column shows the 3 independent A/A draws (seeds 11/22/33). Excess over
floor = `d_F − |F|`.

| t | \|F\| | tower `D`/`d_F` | tower excess | randbase `D` (3 draws) | randbase excess | null `D`/`d_F` |
|--:|--:|:--:|--:|:--|--:|:--:|
| 2 | 4 | 4 / 4 | **0** | 6, 6, 6 | **+2** | 4 / 4 |
| 3 | 8 | 4 / 4 | −4 | 10, 10, 10 | **+2** | 4 / 4 |
| 4 | 16 | 4 / 4 | −12 | 18, 18, 18 | **+2** | 4 / 4 |
| 5 | 32 | 4 / 4 | −28 | 34, 34, 34 | **+2** | 4 / 4 |
| 6 | 64 | 5 / 5 | −59 | 66, 66, 66 | **+2** | 5 / 5 |
| 7 | 128 | 5 / 5 | −123 | 130, 130, 130 | **+2** | 5 / 5 |
| 8 | 256 | 5 / 5 | −251 | 258, 258, 258 | **+2** | 5 / (5)¹ |
| 9 | 512 | 5 / — | −507 | 514, 514, 514 | **+2** | 5 / — |
| 10 | 1024 | 5 / — | −1019 | 1026, 1026, 1026 | **+2** | 5 / — |
| 11 | 2048 | —² | — | 2050 (1 draw, ceiling) | **+2** | — |

¹ One of the two HKY `null` draws at t = 8 was budget-censored (did not reach
`1 ∈ V_{F,c}` within the closure budget under heavy load); the other gave 5,
matching F4 `D`. ² tower/null not re-measured at t = 11; the recorded tower value
stays flat beyond t = 10 (below).

**Recorded tower values beyond the fresh range** (from
`research/pkm_falldegree_unblock_20261107/`, cited): the Kummer m = 2 tower `d_F`
median is bounded (3→6 over N = m·t = 2…22, i.e. t = 1…11), staying `≤ 6` out to
t = 11+ in the cap runs — flat while `|F| = 2^t` grows to 2048+.

## Readings

- **randbase `d_F` is purely the floor.** randbase `D = |F| + 2` at every size,
  and the **excess over the floor is a size-independent constant `+2`** (the two
  `+2` is the two-unknown `S₃` contribution, not structure). randbase carries
  **no information beyond its size**: its fall degree is the degree of the single
  membership polynomial plus a constant.
- **A/A spread = 0.** Three independent random draws of `F` (different seeds, hence
  different bases) give **identical `D`** at every cell (e.g. 258, 258, 258 at
  t = 8). As predicted, a random base's `d_F` is deterministically pinned at the
  floor; the draw of `F` moves it not at all.
- **The structured base's `d_F` is bounded far below `|F|`.** tower `d_F` is flat
  at 4–5 while `|F|` runs 4 → 1024; the excess `d_F − |F|` runs 0 → **−1019**. The
  structured membership is the chain, which has no single polynomial of degree
  `|F|`, so no floor applies.
- **Structure lives in the membership presentation, not the summation polynomial.**
  The **null** arm — random coefficients on `S₃`'s support but the **same tower
  chain** membership — has the **same `d_F` as the tower** (4–5) at every size.
  Randomizing the summation polynomial changes nothing; replacing the chain
  membership with a single degree-`|F|` polynomial (randbase) is what forces the
  floor. So "structure" here means the **low-degree chain presentation of
  membership**, and that is what keeps `d_F` low.

## Crossover

The random base's membership polynomial has degree `|F| = 2^t`; the structured
base's full fall degree is `d_F = 4–5`. The membership degree of the random base
first **exceeds** the structured base's full `d_F` at

> **`|F| ≈ 4 → 8` (t = 2 → 3)**: at t = 2 they are equal (`|F| = 4 = `tower `d_F`);
> by **t = 3** the random base needs a **degree-8** polynomial merely to *write
> down* its factor base, while the structured base of the same size is fully
> *solved* at **`d_F = 4`**. From t = 3 on, the random base costs strictly more to
> **express** than the structured base costs to **solve**, and the gap doubles
> each step (`|F|/d_F`: 2, 3.6, 6.4, 13, 26, 51.6 at t = 3…10).

## What this does and does NOT show

- **Does show** (at this scale, m = 2, Kummer, one small prime): the structured
  subgroup-chain factor base has `d_F` bounded well below `|F|` (flat 4–5), while a
  random base of the same size is floored at `d_F = |F| + 2`. The random base's
  `d_F` is **entirely** the cost of writing its membership down (a single
  degree-`|F|` polynomial) plus a constant `+2`; it carries no structural signal
  (A/A spread 0, excess constant).
- **The honest answer to the question:** the degree-floor confound makes the *raw*
  `d_F` comparison uninformative on its own (randbase's large `d_F` is trivially
  its membership degree, not "lack of structure beating structure"). The real
  signal is the **excess over the floor**: randbase's excess is a constant `+2`
  and the tower's excess is large and negative. So within this instrument the data
  supports: **the structured presentation keeps the fall degree bounded, and a
  same-size random base's fall degree is just the cost of expressing the base.**
  The experiment **cannot separate** "structure lowers the intrinsic difficulty of
  the summation ideal" from "the chain is simply a cheaper *encoding* of the same
  membership", because both live in the same presentation — the null arm shows the
  summation polynomial is irrelevant, so the entire effect is in how membership is
  written. Whether a *different* low-degree encoding of a *random* base (not a
  single degree-`|F|` polynomial) would also stay low is **not tested** and is the
  obvious next control.
- **Does NOT show:** any relation-phase cost, any `S`, any rho comparison, anything
  at cryptographic size, anything for m ≥ 3, or that the tower's low `d_F` yields
  an index-calculus advantage. No security claim. Clock times are context only
  (deterministic metric; box shared with 3 workers, load 3.9–6.9).

## Local ceiling reached

randbase under F4 stays feasible far past the task's `k ≤ 5` guess (two unknowns
keep the matrices narrow): single-draw wall times t=8 0.4 s, t=9 1.9 s, t=10 11 s,
**t=11 69 s** (ceiling for the 3-draw A/A set at t = 10; one t = 11 point taken).
tower/null `d_F` via the reference HKY engine to t = 8 (t = 8 ≈ 32–204 s under
load); tower/null F4 `D` to t = 10. Recorded tower values cited beyond.

## Reproduce

```bash
export CARGO_TARGET_DIR=/home/user/crypto-wt/rand-vs-struct/target
cd research/rand_vs_struct_fb_20261118/harness
cargo build --release --example pkm_tower_pilot          # ~46 s, ~143 MB
cd ../../.. && bash research/rand_vs_struct_fb_20261118/run.sh
```

Build note: `src/cryptanalysis/f4_fp.rs` at commit `2d918fed3b` had a
merge-duplication (`VisitReport` fall-degree fields and
`F4Options::recording_degree_falls` each declared twice); repaired by a pure
dedup in the freeze commit (PREREGISTRATION §7).
