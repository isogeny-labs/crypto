# DRAFT pre-registration: the `m = 4` refutation degree at `ℓ = 4, 5, 6`, chained and FGHR-symmetrised, at surplus 0

**DRAFT. NOT FROZEN.** Written 2026-10-18 for card
`XP-20261018-m4-solving-degree-equivariant`. It freezes only when M0, M1 and M2 of
[`DESIGN.md`](DESIGN.md) are merged, the binaries' sha256 are recorded in §9, and the
commit that adds the frozen text is recorded in `card.json` (`protocol.blob`). Until
then any section may change. After the freeze, §8 applies.

Labels: *recorded* (a committed row), *derived* (arithmetic here or in the README),
*predicted* (stated before any run of this protocol).

## 1. Questions

- **Q1 (arm A, the deciding regime).** On the chained `S₃` system at `m = 4` and
  surplus `S = n − 4ℓ = 0`, does the MutantXL closure refute at degree 4 at
  `ℓ = 4` and `ℓ = 5` (and `ℓ = 6` on host (b))? Every `m = 4`, `ℓ ≥ 4` reading so far
  is at `S ≤ −3` (*recorded*: DREG Results 12–13, m4l6 A1).
- **Q2 (arm A, external F4).** What maximum F4 step degree does msolve reach on the
  same kind of draws, at `(13, 4)`, `(13, 5)` (committed, `S = −3, −7`) and at P1, and
  does it agree with the closure degree draw by draw?
- **Q3 (arm B, FGHR).** On the FGHR-symmetrised `S₅` system (`N_B = 4ℓ − 3` unknowns),
  what is the closure refutation degree `D_B` at `ℓ = 4, 5, 6` (`S = 0`), and is it flat
  in `ℓ` or rising by about one per step?
- **Q4 (arm B, XL framing).** The same question for the plain Macaulay (XL) degree,
  where reachable. This is the analogue of the anchor note's "levels at 7 vs follows
  `ℓ + 4`"; it is reported, and it is not the deciding read (Result 11: XL is 2–3
  degrees pessimistic against the closure on 92/92 draws).
- **Q5 (instrument).** Do the `C₃` Faugère–Svartz blocks reproduce the plain closure
  exactly (E1–E5), and at what CPU and memory ratio?

## 2. Cells

`N = 2n + 4ℓ` (arm A), `N_B = 4ℓ − 3` (arm B), `N = 4ℓ` (arm B0). Memory: dense
closure `0.222·cols²` bytes; `F₄` block two ninths of that (README §5.1).

| id | arm | `(n, ℓ)` | `N` | `S` | degrees built | columns at the top degree | projected peak | host | draws |
|---|---|---|--:|--:|---|--:|--:|---|---|
| Q1c | A, msolve | (13, 4) | 42 | −3 | msolve, unbounded | ≈ 124k (degree 4) | 2.8–4.2 GB | a | committed 06 exports, seed 20261014, unsat 0–3 |
| Q2c | A, msolve | (13, 5) | 46 | −7 | msolve, unbounded | ≈ 179k | 5.8–12.6 GB | a | same |
| Q3c | A, msolve | (15, 6) | 54 | −9 | msolve, unbounded | ≈ 343k | 21–88 GB | a if the S0 probe projects < 72 h, else b | committed m4l6 draws 42, 214 |
| P1-A | A | (16, 4) | 48 | 0 | closure 3–4; msolve | 213,053 | 10 GB; msolve 8–21 GB | a | fresh, 4 unsat + 1 sat |
| P2-A | A | (20, 5) | 60 | 0 | closure 3–4; msolve per probe | 523,686 | 61 GB; msolve 50–314 GB | a (closure), a or b (msolve) | fresh, 4 + 1 |
| P3-A | A | (24, 6) | 72 | 0 | closure 3–4 (needs M3); msolve per probe | 1,091,059 | 264 GB | b | fresh, 4 + 1 |
| P1-B | B | (16, 4) | 13 | 0 | XL and closure `d_sys`..13 | ≤ 8,192 | < 0.1 GB | a | fresh, 4 + 1 |
| P2-B | B | (20, 5) | 17 | 0 | XL and closure `d_sys`..17 | ≤ 131,072 | ≤ 4 GB | a | fresh, 4 + 1 |
| P3-B | B | (24, 6) | 21 | 0 | plain up to 600k columns (`D ≤ 8`); `C₃` blocks up to 1,100,000 (`D ≤ 10`) | 1,048,576 at `D = 10` | 54 GB (blocks) | a | fresh, 4 + 1 |
| E1-B | B | (28, 7) | 25 | 0 | blocks up to 2,500,000 columns (`D ≤ 8`) | 1,807,781 | 161 GB | b, only if G1 and G3 pass | fresh, 4 + 1 |
| E2-B | B | (32, 8) | 29 | 0 | blocks up to `D ≤ 7` | 2,182,396 | 235 GB | b, only if G1 and G3 pass | fresh, 4 + 1 |
| P1-B0 | B0 | (16, 4) | 16 | 0 | XL and closure 12..16 | ≤ 65,536 | ≤ 1 GB | a | fresh, 4 + 1 |

- Arm A at degree 5 is not built anywhere (`C(48, ≤5)` = 1.9M columns, ≈ 0.8 TB):
  a non-refutation at degree 4 is the censored value `≥ 5`.
- The arm-A and arm-B P-cells use the **same draws** `(V ∋ 1, x_R)`; each arm keeps
  its own first four unsatisfiable and first satisfiable draw indices. The two arms'
  factor bases differ as sets (`x(P) ∈ V` against `u(P) ∈ V`), so they are matched by
  cell and sampler, not draw by draw.
- S0 may run P1-A and P2-A on `dreg_ladder --m 4` draws before M1 exists; those rows
  are labelled `sampler=dreg_ladder` and are not pooled with the shared-draw rows.

## 3. Draws and seeds

- Fresh cells: seed **20261019**; cell seed `20261019 ^ (n << 40) ^ (ℓ << 32) ^
  (4 << 48) ^ (0xB << 56)`; draw `k` from `StdRng::seed_from_u64(cell_seed)` in order:
  `V` uniform among `ℓ`-dimensional subspaces of `F_{2^n}` that contain 1 and are not
  Frobenius-stable; `x_R` uniform in `F_{2^n} \ {0, 1}`. Irreducible polynomial:
  `find_irreducible_sparse(n)`.
- A draw is unsatisfiable only by exact count (arm A `chained_solution_count`; arms B,
  B0 enumeration). `--max-draws 512` per cell; if fewer than four unsatisfiable draws
  are found, the cell runs on what it found and says so.
- Committed cells: the 06 exports (seed 20261014, `ic_next10_06_chain_export
  --unsat-index 0..3`) and m4l6's `draws/m4-15-6.jsonl` draws 42 and 214, unchanged.

## 4. Instruments and caps

- **Closure (primary)**: `m4_formulations --engine closure` (arm A delegates to the same
  `closure_refutation_scan` `dreg_closure` uses), `--from 3` (arm A) or `--from d_sys`
  (arms B, B0), `--to` per §2, `--max-cols 600000` on host (a), `1,400,000` on host (b).
  One process per draw, `prlimit --as` = the cell's projection × 1.5 capped at 100 GB on
  host (a), `timeout` 72 h. A process ended by the cap or the timeout is recorded with
  the degrees it completed; it is never re-run (§8).
- **XL (secondary)**: the plain rank at each built degree, reported by the same scan.
- **msolve (external, AGENTS.md §15)**: v0.6.5, `-t 1 -v 2 -g 2` on host (a) (`-t 16`
  allowed on host (b); threads do not change the degree sequence, which control C4
  checks), field equations appended, DRL with the exporter's variable order, address-
  space cap per §2 and at most 100 GB on host (a), wall 72 h. `D_F4` = maximum `deg`
  over all F4 rounds; `GB = [1]` required for an unsatisfiable draw to count as
  refuted.
- **`C₃` blocks**: `--engine closure-c3` only after G2, and on a draw where the plain
  closure also fits only as the E-check, never in place of it.

## 5. Controls

- **C1, positive (the instrument can see a rise).**
  - Closure: replay the first unsatisfiable committed `m = 3`, `(19, 7)` draw (`research/dreg_closure_scale_20261016/draws/m3-19-7.jsonl`) at degree 4: it must **not** refute (*recorded*: 8/8 such draws saturate at 4 without `1`, Result 12).
  - msolve: the committed satisfiable `(11, 4)` draw 0 must need degree 5 for its full
    basis (*recorded*, 06 §1 E).
  - Arm B: four random Boolean systems per P-cell with arm B's `N_B`, `n`, and per-equation
    term counts by degree (uniform random monomials, seeds 1–4). If they resolve at the
    same degree as the real draws, arm B's reading is "instrument blind at this cell".
- **C2, soundness (null).** One satisfiable draw per (arm, cell): XL, closure, blocks and
  msolve must not refute it (`GB ≠ [1]`). Any refutation voids the cell until explained.
- **C3, equivariance identity.** E1–E5 of DESIGN §4.4 on every draw where both engines
  fit; zero mismatches or the block rows are void.
- **C4, A/A.** msolve rerun on draw 0 of Q1c and P1-A: identical degree sequence and
  largest matrix. The closure is deterministic; one repeat at P1-A must give identical
  ranks and word counts.
- **C5, reproduction (gate G0).** As DESIGN §2: 06's `(11, 4)` msolve readings and one
  committed `(12, 5)` closure refutation reproduce before any new cell runs.

## 6. Readouts and decision rule

**Per draw.** `D = d` when the instrument refutes at `d` having built `d − 1` without
refuting; `≥ d + 1` when degree `d` was built and saturated without `1` and `d + 1`
was not built; `≥ d` when a cap or timeout ended degree `d` (no reading at `d`);
for msolve, `≥ d` when a productive step at degree `d` was logged before a cap.

**Per cell.** The median of the unsatisfiable draws, censored values entered at
their bound; the cell is **censored** if two or more of its draws are censored below
the value that would separate the readings below, and **inconclusive** if it has
fewer than two unsatisfiable draws.

**Arm A (Q1), at each P-cell:**

| outcome | reading |
|---|---|
| every draw refutes at 4 | **A4**: degree 4 holds at `S = 0` at this `ℓ` |
| three or four draws saturate at 4 without `1` | **A≥5**: surplus 0 raises the degree at this `ℓ` |
| otherwise | inconclusive, draws named |

- A4 at P1 and P2 (and P3 if run): the closure degree-4 line extends from `S ≤ −3`
  to `S = 0` through `ℓ = 5` (or 6). The DESCENT_DEGREE degree-4 row (`2^68.43`) gains
  support to that `ℓ`; it stays contradicted for `ℓ ≥ 7` by Result 13's survivors
  until a cell there is read.
- A≥5 at any P-cell: the degree-4 row is withdrawn at `S = 0` from that `ℓ`; the
  degree-5 row (`2^71.13`, `+10.32` bits) becomes the optimistic case with no
  measured support at `S = 0` (degree 5 cannot be built there); the next measurement is
  DESCENT_DEGREE §9's guess-then-degree-4 count at that cell.

**msolve (Q2):** reported per draw beside the closure. `D_F4 > D_closure` or
`D_F4 < D_closure` on any draw is an instrument disagreement, reported with both
traces; it does not change arm A's reading, which is the closure's.

**Arm B (Q3), over P1–P3** (and E1, E2 if run). Let `Δ₁ = D_B(P2) − D_B(P1)`,
`Δ₂ = D_B(P3) − D_B(P2)`. A censored `P3` with bound `≥ D_B(P2) + 1` counts as
`Δ₂ ≥ 1`; with a lower bound it makes the arm inconclusive.

| outcome | reading | consequence (derived, README §5.4, `ℓ = 33`, `w = 1`, unconverted) |
|---|---|---|
| `Δ₁ = Δ₂ = 0` and `D_B ≤ 6` | **B-flat-low** | an FGHR row with oracle `2^28.1–2^32.5` against chained degree 5's `2^36.2`: added to DESCENT_DEGREE as a new optimistic row, re-optimised by `descent_degree_cost`; `K` and the symmetry constants become the next measurements |
| `Δ₁ = Δ₂ = 0` and `D_B = 7` | **B-flat-tie** | oracle `2^36.6`, level with chained degree 5: no row moves |
| `Δ₁ = Δ₂ = 0` and `D_B ≥ 8` | **B-flat-high** | no ECC2K-130 consequence; mechanism only |
| `Δ₁ ≥ 1` and `Δ₂ ≥ 1` | **B-follows** | FGHR does not stop the degree growing with `ℓ` at `m = 4`; the review's §3.5 symmetry arms are closed for `m = 4` on this formulation |
| exactly one of `Δ₁, Δ₂ ≥ 1` | **B-step** | inconclusive between flat and follows; E1 decides if G1 and G3 allow, else reported open |

**Arm B, XL (Q4):** the same `Δ` rule on the XL degrees, reported as "levels" or
"follows `ℓ + c`". It maps to the anchor note's "levels at 7" (`+21.30` bits) and
"`ℓ + 4`" (`+112.32` bits) rows only as an analogue: those rows are the chained
system's XL and are not re-priced from arm B.

**B0 (P1 only):** `D_B0` against `D_B` at P1, descriptive ("no chain" against
"symmetry").

**Q5:** E1–E5 pass/fail; CPU and peak-RSS ratios of blocks against the plain closure
at every cell where both ran, with a ratio column against msolve on the same systems.

No reading here is a cost row, an improvement or a claim about ECC2K-130's security;
consequences for the price rows are written as *derived* updates in the result PR.

## 7. Predictions (before any run of this protocol)

1. `d_sys = 9` (cubes of `w_i` survive in `S₅`'s invariant form). Confidence 0.55;
   `d_sys ≤ 7` 0.3.
2. P1-A refutes at 4 on every draw (0.65); P2-A likewise (0.5).
3. msolve `D_F4 = 4` on every draw of Q1c and Q2c (0.6), and equal to the closure degree
   on every draw where both finish (0.7).
4. Arm B: `D_B = d_sys` at P1 and P2 (0.4); B-follows (0.35); B-flat in some form (0.45).
5. `C₃` blocks: 2–4× CPU and 0.2–0.3× peak RSS against the plain closure at the largest
   shared arm-B matrix (0.5).
6. At least 80% of draws unsatisfiable at `S = 0` for both arms (0.6).

## 8. Inadmissible after the freeze

- Changing a cell, the seed, the draw sampler (uniform, `1 ∈ V`, not Frobenius-stable),
  the draw order or the unsatisfiable criterion (exact count only).
- Dropping a draw, adding draws to a cell after it has a reading, or re-running a draw
  in the hope of a different answer. A host loss (not a cap) is re-run whole with the
  same binary and flags, and the lost attempt's files are kept.
- Lowering a cap or timeout after a cell starts; reading a censored value as a degree.
- Changing the degree definitions of §6, msolve's flags or variable order, or the
  closure's round limit.
- Using a block-engine outcome before E1–E5 pass, or in place of a plain run that fits.
- Pooling the S0 `dreg_ladder` rows with the shared-draw rows; mixing arms within a
  reading.
- Reporting wall time as the metric (the counted fields are: columns, ranks, closure
  products and word operations, msolve matrix sizes), or any `n = 131` figure as
  measured.
- Amending §6 after any measured run of an affected cell. Amendments are allowed only
  before the affected cell runs, appended with their UTC time.

## 9. Disclosures and pins (filled at freeze)

- **Probes run while writing this draft: none.** On 2026-10-18 at 20:50Z host (a) was
  at load 44 on 32 vCPU with a multi-day job running from another checkout; the only
  binary built was `backlog` (to validate the card). Every size above is derived from
  committed measurements.
- Read, not run: the files cited in the README and DESIGN.
- At freeze: commit, binary sha256 (`m4_formulations`, `msolve_degree_log`, msolve),
  `rustc` version, host record (`lscpu`, `MemTotal`, cgroup limit).

## 10. Host and isolation

Host (a): 32 vCPU Xeon 6975P (AVX-512), 124 GB, 2 TB EBS, shared; at most four
concurrent single-threaded jobs and 100 GB in total; isolation tier L0. Host (b): a
512 GB host for the S3 rows, recorded the same way. Wall times are descriptive.
