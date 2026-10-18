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
