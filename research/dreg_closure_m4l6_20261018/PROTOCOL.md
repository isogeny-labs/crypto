# Pre-registration: the (m, ℓ) = (4, 6) cell, the (4, 7) cell and degree 5 at ℓ = 7, on a large-memory host

Registered 2026-10-18 by the commit that adds this file, before any cell below
has run. Successor of
[`research/dreg_closure_driver_20261016/RESULTS.md`](../dreg_closure_driver_20261016/RESULTS.md)
("Successor": the cell that separates the two survivors is m = 4, ℓ = 6; the
open question for the cost rows is whether degree 5 holds past ℓ = 7). Owner
request 2026-10-18: run these on a big node.

Every statement carries one label: *recorded* (a committed row), *derived*
(a calculation shown here) or *predicted* (stated before the runs).

## 1. Questions

Two explanations survived the driver study (*recorded*, its RESULTS.md §
"Successor"): the MutantXL closure degree rises above 4 **at ℓ = 7** whatever
`m`, or **once m·ℓ ≥ 21**. And degree 5 itself is unmeasured above N = 25.

- **Q1.** Does m = 4, ℓ = 6 (m·ℓ = 24) refute at degree 4? "m·ℓ ≥ 21" predicts
  it rises; "ℓ = 7" predicts it holds.
- **Q2.** Does m = 4, ℓ = 7 refute at degree 4? "ℓ = 7" predicts it rises at
  both m; "m·ℓ" (28) also predicts a rise. A refutation at 4 refutes both.
- **Q3.** Does the closure refute at degree 5 on the committed m = 3, ℓ = 7
  draws that did not refute at 4? If yes, the ECC2K-130 "degree 5 held to
  n = 131" row (2^71.13, `RESEARCH_ECC2K130_DESCENT_DEGREE.md` point 7) keeps
  measured support at ℓ = 7; if no, the degree at ℓ = 7 is at least 6 and that
  row loses its support.

## 2. Cells

Unknowns N = (m − 2)·n + m·ℓ; surplus S = n − m·ℓ (*derived*, `examples/dreg_ladder.rs`
header). Columns C(N, ≤D) (*derived*). Memory is the dense closure's; the scale
study recorded peak RSS ≈ 3 × C(N,≤4)²/8 bytes (*derived* from its 5.8 GB at
124,314 columns), so the projections below use 3·cols²/8.

| id | m | (n, ℓ) | N | S | m·ℓ | degree built | C(N, ≤D) | projected RSS | draws |
|---|--:|---|--:|--:|--:|--:|--:|--:|---|
| A1 | 4 | (15, 6) | 54 | −9 | 24 | ≤ 4 | 341,055 | 44 GB | fresh, seed 20261018 |
| A2 | 4 | (16, 6) | 56 | −8 | 24 | ≤ 4 | 395,011 | 59 GB | fresh, seed 20261018 |
| B1 | 4 | (17, 7) | 62 | −11 | 28 | ≤ 4 | 597,619 | 134 GB | fresh, seed 20261018 |
| B2 | 4 | (18, 7) | 64 | −10 | 28 | ≤ 4 | 679,121 | 173 GB | fresh, seed 20261018 |
| C1 | 3 | (19, 7) | 40 | −2 | 21 | 5 | 760,099 | 217 GB | committed: `research/dreg_closure_scale_20261016/draws/m3-19-7.jsonl` |
| C2 | 3 | (21, 7) | 42 | 0 | 21 | 5 | 974,982 | 357 GB | committed: `.../draws/m3-21-7.jsonl` |

- A and B answer Q1 and Q2 at degree 4 only (`--to 4`). C answers Q3 at
  degree 5 (`--to 5`); the scale study already built degree 4 on those draws.
- The surplus at A and B is as negative as the driver study's (4,5) cells
  (−9, −8) or one to three lower; unsatisfiable draws were found there, and
  none were found at S = −14. If the ladder finds fewer than four
  unsatisfiable draws in `--max-draws 4096`, the cell runs on what it found
  and the count is recorded.
- C2 runs only on a host whose cgroup memory limit is at least 420 GB; on the
  256 GB host it is not attempted, and that is recorded, not a failure.

## 3. Instrument

Unchanged from the driver study except the caps, which the larger host allows:

- draws: `examples/dreg_ladder --cells n:ℓ:3 --m 4 --unsat 4 --ffd-max 3
  --seed 20261018 --max-draws 4096` (exact solution count; XL to degree 3
  only). Fresh draws are served from the host and committed before any
  closure measurement starts; the host waits for the tag
  `draws-m4l6-20261018` on `origin` before it measures.
- closure: `examples/dreg_closure --to D --max-cols 1100000 --draw K file`,
  one process per draw, under `prlimit --as=A` with
  `A = 0.92 × (cgroup memory limit)` bytes, and `timeout 43200` (12 h) for
  degree 4, `timeout 144000` (40 h) for degree 5.
- `F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000`, `RAYON_NUM_THREADS` =
  the host's cores (recorded).
- control: the first satisfiable draw of each fresh cell under
  `--satisfiable` (soundness: neither XL nor the closure may refute it).
- host: `run.sh host` records commit, binary sha256, rustc, cgroup limit,
  `A`, lscpu, before any measured run. A process the kernel or `prlimit` ends
  for memory is recorded as an allocation failure with the degrees it built;
  it is not evidence about the degree and is not re-run.
- scoring: `examples/dreg_score` where it applies; otherwise the per-degree
  JSON fields `closure_refuted`, `saturated`, `columns`, `closure_products`,
  `closure_word_ops`, `secs` are read directly.

## 4. Decision rule, fixed now

Per cell, over its unsatisfiable draws (four unless fewer were found):

| cell | all draws refute at the built degree | no draw refutes (closure saturated) | mixed or capped |
|---|---|---|---|
| A (Q1) | "m·ℓ ≥ 21" **refuted**; "ℓ = 7" survives | "ℓ = 7" **refuted**; "m·ℓ" survives | inconclusive; name the draws |
| B (Q2) | both explanations refuted; a new one is needed (an m-dependent threshold) | consistent with both | inconclusive |
| C (Q3) | degree at ℓ = 7 is exactly 5 for m = 3; the degree-5 row keeps measured support at ℓ = 7 | degree ≥ 6 at ℓ = 7; the degree-5 row loses support; the slope from ℓ = 6 to 7 is ≥ 2 degrees | inconclusive |

A control draw that refutes voids the cell's reading until explained. No
cell's result is a cost row, an improvement or a claim about ECC2K-130; the
cost-model consequence is written as a derived update to
`RESEARCH_ECC2K130_DESCENT_DEGREE.md` in the result PR.

## 5. Predictions (before the runs)

1. A refutes at 4 on every draw ("ℓ = 7" survives). Confidence about 0.6: the
   m = 4 line held at 4 through N = 42 with m·ℓ = 16, and the driver study
   could not reject either survivor.
2. B does not refute at 4. Confidence about 0.6.
3. C1 refutes at degree 5 on all four draws. Confidence about 0.7: at (19, 7)
   the degree-4 closure reached rank 94,230 of 102,091 columns before
   saturating (*recorded*, scale RESULTS.md), which is close.
4. Peak RSS within 1.5× of the projections in §2.

## 6. What was run before this protocol

Nothing at any cell above. Read: the three `dreg_closure_*` studies and
`examples/dreg_closure.rs`, `examples/dreg_ladder.rs`. The binaries are built
on the measuring host from the commit that adds this file (or a later commit
on the same branch that only adds host records); `run.sh host` records the
sha256.

## 7. Host

A RunPod CPU pod (memory-optimised flavour, 32 vCPU, 256 GB) launched by
`pod/launch.py`; its boot script is `pod/boot.sh`. The cgroup limit, not
`/proc/meminfo`, is the memory the process can use; `run.sh` reads the cgroup
limit. Isolation: a shared cloud host, level L0 by the repository's tiers;
wall times are descriptive, the counted fields (`columns`, `closure_products`,
`closure_word_ops`) are the measurement.

## 8. Amendment before any run (2026-10-18, owner budget: 50 dollars total)

Registered before any cell ran. The owner capped total spend at 50 dollars,
which buys about 11.5 hours of the cheapest 256 GB host (cpu3m, 32 vCPU,
3.52 dollars per hour) plus a small host for the PKM msolve cell (§9). The
dense closure is single-threaded (`inherited_f4.rs` uses no rayon), so the
32 vCPU buy only the memory. Measured closure times at 124k and 150k columns
were 200–730 s (*recorded*, scale and driver studies); at the work exponent
2.2–2.3 in columns (*recorded*, scale RESULTS.md Q6) the projections are:
A1 ≈ 1 h and A2 ≈ 1.5 h per draw, B1 ≈ 4 h, B2 ≈ 5 h, C1 ≈ 7 h or more.

Within that budget the run order is fixed as:

1. `gen` for the A cells only (`FRESH="4:15:6 4:16:6"`), `--max-draws 4096`.
2. `measure`: A1 and A2 side by side (103 GB projected), draws 0 and 1 first,
   then draws 2 and 3; each process under `timeout 14400` (4 h).
3. `deg5`: C1, draw 0 only, under `timeout` set to the host's remaining
   lifetime minus 40 minutes; C2 is not attempted (257 GB host).
4. The B cells are **not run** in this budget; they stay registered in §2 and
   §4 for a later run with their own host record.
5. The host ends itself 45 minutes after its last run so results can be
   fetched, and in any case at its lifetime cap (11.5 h). A process the
   lifetime cap ends is recorded as a host loss, not as evidence.

The decision rule of §4 is unchanged; a cell read on fewer than four draws
says so in its row. The predictions of §5 stand.

## 9. PKM m = 3, N = 21 under the same budget

The frozen f4_fp_tower protocol (`research/pkm_lead1_m3n21_20261016/PROTOCOL.md`)
needs a 192 GB host for 14–60 h on 32 threads (*derived* there), beyond this
budget. It is **not run** here. Instead the msolve instrument of
`research/ic_next10_20261014/03-pkm-degree/` (positive control 16/16; stopped
at `m = 3`, `N = 21` by its 45-minute wall inside the second degree-7 step at
2.6 GB) is rerun on the committed `m3_t7` systems with a long wall, in
`research/pkm_m3_n21_msolve_20261018/` under its own protocol note. Its
reading follows item 3's rule: a productive degree-8 step is H0 (growth
resumes) from a lower bound; a finished run gives `D`; a wall stop with no
degree-8 step is inconclusive. It does not replace the f4_fp_tower cell.

## 10. Amendment 2026-10-18 11:30Z: A1 runs on the draws found, A2 deferred

Recorded before any closure measurement. On the first host (RunPod
2×RTX 4090, 200 GB, booted 06:22Z) draw generation for A1 (m4-15-6,
S = −9) ran 06:29Z–11:30Z at about 70 s per draw and found 2 unsatisfiable
draws in 251 (draws 42 and 214), against the prediction in §4 that S = −9
would reach `--unsat 4` within about 150 draws. At that rate A1 alone would
have needed about 8.5 h more of generation and A2 (m4-16-6) had not started,
so the host's 12 h cap would have expired before any measurement.

- The 251 A1 draws are committed as `draws/m4-15-6.jsonl` (+ `.log`,
  `sha256.txt`) and tagged `draws-m4l6-20261018`; A1 is measured on its two
  unsatisfiable draws (42, 214), side by side under §8, exactly as §8 planned
  for "draws 0 and 1 of the unsatisfiable list". The generator's
  `--max-draws` is thus effectively 251 for A1 ("the cell runs on what it
  found", §5).
- A2 (m4-16-6) joins the B cells and C2 as deferred: no draws, no
  measurement, no result in this run.
- The first host was terminated at 11:30Z without measuring; a second host
  with the same shape is launched with `FRESH=4:15:6`, which skips
  generation (the committed file exists) and waits for the tag as before.
- Nothing else in §§1–9 changes. The unsatisfiable-draw rate at S = −9
  (2/251, 95 % Clopper–Pearson 0.1 %–2.8 %) is itself a recorded
  observation against §4's expectation and is reported with the result.

Addendum 11:45Z: every push of the tag `draws-m4l6-20261018` from the
authoring session was refused by its git proxy ("remote end hung up" on
`refs/tags/*`; branch pushes succeed), so the pre-registration anchor is the
commit that adds `draws/m4-15-6.jsonl` and `draws/sha256.txt`
(`fc1eea79`, pushed 11:33Z) and the host's gate is that committed checksum
file verifying in its clone. The tag may be added later from a local
checkout; it must point at `fc1eea79` or a descendant that leaves `draws/`
unchanged.

## 11. Amendment A3 (2026-10-18 21:40Z, before any draw ≥ 251 of A1 exists): a decisive A1 count, parallel draw generation, C1 on a host above 280 GB

Registered by the commit that adds this section. No draw of the A1 stream
beyond the committed 251 (draws 0–250, `draws/m4-15-6.jsonl`) has been
generated anywhere, and no closure has run since §10's host. Owner request
2026-10-18 (rented-hardware leads; `research/ic_leads_rented_20261018/`).
Labels as in the header: *recorded*, *derived*, *predicted*.

### 11.1 Why

§4 row A was read on 2 unsatisfiable draws (42, 214) because only 2 turned
up in 251 (*recorded*, RESULTS.md). "m·ℓ ≥ 21" is refuted by any one
refuting draw, so that reading stands. What survives, "ℓ = 7", predicts more
than the 2 draws can show: **every** unsatisfiable A1 draw refutes at
degree 4. Two out of two bounds the share of A1 draws that need degree 5 or
more only by 78 % (one-sided 95 %, `1 − 0.05^{1/2}`). A3 makes that bound
small enough to read. Generation was the bottleneck: one process, about 70 s
per draw (*recorded*, `draws/m4-15-6.jsonl` `secs`).

### 11.2 A1 extension (cell A1 unchanged: m = 4, n = 15, ℓ = 6, N = 54, S = −9)

- **Draws.** The same stream as the committed file (seed 20261018, cell seed
  as in `examples/dreg_ladder.rs`), fixed range **draws 251 to 3250**
  (3,000 fresh draws). The range is fixed before generation and does not
  depend on what it finds: there is no stop at a count. *Derived*: at the
  recorded rate 2/251 the range holds about 24 unsatisfiable draws; at the
  Clopper–Pearson ends of that rate (0.1 %–2.8 %), 3 to 84.
- **Parallel generation.** `dreg_ladder --shard I/J --first-draw 251
  --max-draws 3251`, `J` = the host's vCPUs, one process per shard
  (`run.sh gen-a3`). Every process samples every draw of the stream
  (`ladder_sample`, cheap) and measures only its residue class, so draw `k`
  is exactly the draw the sequential run gives at `k`; the merged file,
  sorted by draw, is the sequential file up to `secs`.
- **Replay control (gate).** Before any fresh draw, the same shards
  regenerate draws `0 … J−1` and must equal the committed lines on every
  field but `secs` (`run.sh gen-a3` compares; a mismatch stops the host with
  nothing fresh made). Pre-run check on the authoring container (4 shards):
  draws 0–7 and 40–43, including unsatisfiable draw 42, identical
  (`research/ic_leads_rented_20261018/tests/shard_replay_20261018/`).
- **Selection (fixed now).** The first **16** unsatisfiable fresh draws in
  draw order are measured; if fewer than 16 exist in the range, all of them.
  The first satisfiable fresh draw is the control (`--satisfiable`).
- **Anchor.** §10 gated measurement on the committed draws. A3 replaces that
  gate with the fixed seed, range and selection rule above, which leave no
  choice once generation starts: the host records the merged file's sha256
  in `runs/queue.log` and `draws/a3-sha256.txt` before the first closure,
  checks it again before measuring, and the file is committed with the
  result. (The pods carry no git credentials.)
- **Closure.** As §3, `--to 4`. At most `P = ⌊A / 39 GB⌋` closures at once,
  each under `prlimit --as = A / P` (*recorded*: 25.8 GB peak RSS per A1
  draw, 24 min each, alone), `timeout 14400` each.

### 11.3 C1 on a larger host

C1 (m = 3, (19, 7), degree 5) failed to allocate at 169 GB resident under the
184 GB cap (*recorded*); its projection was 217 GB. A3 runs it only on a host
whose cgroup limit is at least **280 GB** (1.3 × the projection; `A` = 0.92 ×
limit ≥ 258 GB), on draw 0 first (`DEG5_DRAWS=1`), timeout 12 h. Draws 1–3
run only in a later launch with `DEG5_DRAWS=4` and their own host record.
Below 280 GB it is not attempted, and that is recorded. §4 row C applies;
read on one draw, a non-refuting saturated closure already shows degree
≥ 6 on that draw, and a refutation reads as "degree 5 on draw 0 only".

### 11.4 Decision rule for the A1 extension (fixed now)

Over the measured fresh unsatisfiable draws. `R` = refute at degree 4,
`Z` = closure saturated at degree 4 without refuting, `X` = capped,
allocation failure or timeout (not evidence about the degree).

| outcome | reading |
|---|---|
| control refutes | the cell's reading is void until explained (as §4) |
| `X > (R + Z + X) / 4` | inconclusive; the counts are recorded |
| `Z ≥ 1` | "ℓ = 7" in its strong form (degree 4 on every unsatisfiable (4, 6) draw) is **refuted**; the degree at (4, 6) is mixed; `Z / (R + Z)` is reported with its 95 % Clopper–Pearson interval |
| `Z = 0`, `R + 2 ≥ 14` | **decisive**: "ℓ = 7" survives; the share of unsatisfiable A1 draws needing degree ≥ 5 is below `1 − 0.05^{1/(R+2)}` (95 %, pooled with draws 42 and 214; 15.3 % at R = 16) |
| `Z = 0`, `R + 2 < 14` | consistent with "ℓ = 7", not decisive; the count is recorded |

No row is a cost row, an improvement or a claim about ECC2K-130; the
consequence for `RESEARCH_ECC2K130_DESCENT_DEGREE.md` stays a derived update
in the result PR.

### 11.5 Predictions (before the runs)

1. The replay control passes (the pre-run check did).
2. 15 to 35 unsatisfiable draws in the range (the 2/251 rate); confidence 0.6.
3. `Z = 0` and the decisive row; confidence 0.6, as §5.1.
4. Generation at 32 vCPU: 1.5 to 3.5 h for 3,000 draws (70 s per draw per
   process, 32 processes, an SMT slowdown of 1 to 2×).
5. C1 draw 0 refutes at degree 5 (§5.3 unchanged, confidence 0.7), peak RSS
   below 260 GB.

### 11.6 Hosts and cards

`research/ic_leads_rented_20261018/closure_a3/LAUNCH.md` (A1 extension:
cpu3m, 32 vCPU, 256 GB) and `research/ic_leads_rented_20261018/closure_c1/LAUNCH.md`
(C1: a 2 × L40 GPU pod, used for its 286 GB of host memory; the GPUs are
idle). A2, B1, B2 and C2 stay deferred.

### 11.7 Disclosure after the freeze (plumbing tests only)

After the commit that froze §11 (`ee7a5d972`), a local test of `pod/boot.sh a3` on the 4 vCPU,
15 GB authoring container ran the whole job with `A3_FIRST=251 A3_END=255`
(`research/ic_leads_rented_20261018/tests/boot_a3_20261018/`). It computed
draws 251–254 of the frozen range, all satisfiable, with no closure on any
unsatisfiable draw. The control closure on draw 251 was killed by the host's
memory (exit 137). Those files are kept as test records, not in `draws/`, so
the pod generates the whole range itself. Draws 251–254 are fixed by the
seed, and the selection rule (§11.2) reads only draw order, so this changes
nothing in what will be measured.

### 11.8 Host change for A3 (2026-10-24, before any draw ≥ 251 is kept)

On the owner's request A3 moves from RunPod to the `crypto-eks-highmem` ARC
runner on the Karpenter Spot EKS cluster (`taskq/deploy/eks-karpenter/arc/`),
dispatched by `.github/workflows/dreg-a3-eks-highmem.yml` from `main`. Nothing in
§11.1–§11.5 changes. `run.sh` already sizes both stages to the host: `J` = nproc
shards (16 there) and `P = ⌊A / 39 GB⌋` closures at once with `A` = 0.92 × the
pod's 120 GiB limit (P = 3, each under about 39.5 GB; the recorded A1 peak was
25.8 GB). The workflow pushes the draw file and `a3-sha256.txt` to
`results/dreg-a3-eks-20261024` before any closure runs, which is the §11.2 anchor.

The stopped RunPod host of 2026-10-18 left 473 shard lines for draws 251–730
(`research/ic_leads_rented_20261018/snapshots/6sh5qyoyusv6ux/draws/a3-shards/`).
They are not the A3 draw file. The fresh EKS file must equal them line for line
on every field but `secs`; a mismatch is reported with the result and voids the
run as §11.2's replay control would.
