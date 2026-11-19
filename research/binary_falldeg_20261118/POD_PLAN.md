# Pod plan — binary Koblitz fall-degree study

Not run here. Pod cells need the owner's price approval and go through
`isolab-eks` (AGENTS.md §9a); this session stopped at the local 4-vCPU
ceiling. Projections are from the pilot fit and are **low confidence**.

## Why a pod

The pilot ceiling (`n ≤ 23`, `m ≤ 3`, `cmax = 4`) kept `d_F ∈ {2,3,4}`. The
prior binary program first saw `d_F ≥ 5` at `m = 3`, `ℓ ≥ 7`
(`research/fall_degree_bounds_20261008/`). To decide the two open questions —
(a) whether the weak "structured (polynomial-window) `d_F` ≈ 1 below random"
signal persists or grows, and (b) whether any Frobenius-orbit effect appears
only beyond toy size — the HKY closure must reach `cmax = 5` (and `6` where it
fits) at `m = 3`, `ℓ = 7`, which needs memory the container does not have.

## Cost model (from the pilot)

The dense degree-`c` closure has `~C(n_vars, ≤ c)` columns, `n_vars = m·ℓ +
(m−2)·n`, dense bit-rows. Memory ≈ `columns² / 8` bytes at worst (echelon);
rows are truncated at their pivot so the realised fill is lower. Pilot
`m=3, n=15, ℓ=5` (`n_vars = 30`, `cmax = 4`, `C(30,≤4) = 31,931` cols) ran in
~minutes on 4 vCPU. Scaling to the pod cells (order-of-magnitude):

| cell | n_vars | C(n_vars,≤5) | C(n_vars,≤6) | projected |
|:--|--:|--:|--:|:--|
| m=3 n=21 ℓ=7 | 42 | ~8.5e5 | ~5.2e6 | `cmax=5` ~0.5–2 h / ~16–32 GB; `cmax=6` ~1 day / 128–256 GB |
| m=3 n=25 ℓ=7 | 46 | ~1.4e6 | ~9.4e6 | `cmax=5` ~1–4 h / 32–64 GB |
| m=3 n=27 ℓ=9 | 48 | ~1.7e6 | ~1.2e7 | `cmax=5` ~2–6 h / 48–96 GB |
| m=2 n=25 ℓ=12 | 24 | ~5.3e4 | ~1.9e5 | minutes (fits locally; include as control) |

`n = 21, 25, 27` are composite with rich 2-cyclotomic structure, so a
`τ`-invariant orbit base exists at `ℓ = 7, 9` (H-avail): `n=21 ℓ=7` =
`GF(2^7)` (subfield); `n=25 ℓ=7`, `n=27 ℓ=9` mix cosets (non-subfield).

## Frozen cells

- **Ladder P1 (orbit effect at `m=3`, composite `n`):** arms
  `orbit,random,subspace`, `m=3`, `(n,ℓ)` = `(21,7), (25,7), (27,9)`,
  `cmax=5`, `--ffd-max 6`, `--dres-max 6`, `--keep 6`.
- **Ladder P2 (structured-vs-random at larger `m=2`):** arms
  `random,subspace`, `m=2`, `(n,ℓ)` = `(23,11), (25,12), (29,14)`,
  `cmax=5`, `--keep 6` (tests whether the 1-degree window gap grows in `n`).
- **A/A arm:** two `random` sub-arms at a shared cell with different master
  seeds, `--keep 12`, to calibrate the noise floor the pilot could only bound.

## Frozen command (per cell, on the pod image built from this commit)

```
export CARGO_TARGET_DIR=$PWD/target
cargo build --release --example binary_falldeg_arms
RAYON_NUM_THREADS=<pod vCPU> HKY_CHUNK=<fit to RAM> \
  target/release/examples/binary_falldeg_arms run \
    --cells 3:21:7,3:25:7,3:27:9 --seed 1262698074 \
    --keep 6 --cmax 5 --ffd-max 6 --dres-max 6 \
    --arms orbit,random,subspace --crosscheck > out/P1.jsonl 2> out/P1.err
```

Per-cell wall budget 86,400 s; memory request per the table (round up to the
pool's node sizes). Spot reclaim with no checkpoint loses the cell (record as
censored, never dropped). Fill `.resume/RESUME.md` before launch
(`scripts/checkpoint.sh init`). Verify the tested commit and the image digest.

## What the pod run would decide

- If `m=3, ℓ=7` reaches `d_F = 5` and the orbit/random/subspace medians still
  agree within the calibrated A/A floor → **"no Frobenius-orbit effect"** on
  `d_F` up to the first fall past 4 (the pilot's reading, extended).
- If the window `d_F` gap over random grows with `n` → a real (non-Frobenius)
  low-degree-subspace effect worth a note; still not a cost or security claim.
- A censored `cmax=5` closure reads **undecided**, never **flat**.

---

## Amendment A1 (2026-11-18): instrument build, calibration, staged pod job

Added 2026-11-18 **before any pod data exists**. The frozen sections above
and `PREREGISTRATION.md` are not edited; where this amendment disagrees with
the "Cost model" table, this amendment's numbers replace those projections
for planning only. Nothing here is a study row; all numbers are
`[pcc:unverified]`.

### A1.1 Frozen binary

- Source: worktree `wt/binary-ladder` at `e0ac67a2b5`, no source change
  (the root crate built as-is; the earlier `f4_fp.rs` duplication fix was
  already merged).
- `CARGO_TARGET_DIR=$PWD/target cargo build --release --example binary_falldeg_arms`,
  `rustc 1.97.0 (2d8144b78 2026-07-07)`, profile `release`, **no**
  `RUSTFLAGS`, no `target-cpu`/`target-feature` (no `.cargo/config.toml` in
  the repo or `~/.cargo`); SIMD only through runtime detection.
- `target/release/examples/binary_falldeg_arms`, 7,368,384 bytes,
  **sha256 `2ed36ccad8a7a3a903792d601704e7afa2eeb254954be3af4be12bb131ad0d54`**.
- `ldd`: `libgcc_s.so.1`, `libc.so.6`, `ld-linux-x86-64.so.2` only. Highest
  versioned symbol `GLIBC_2.34`, so it runs on glibc ≥ 2.34 (Ubuntu 22.04's
  2.35 and later). Build log `raw/build-20261118.log`.

### A1.2 Local gates and exact pilot reproduction (4-vCPU container)

- G1 `cargo test --release --example binary_falldeg_arms`: **4 passed**
  (`raw/gate-G1-test-20261118.log`).
- `binary_falldeg_arms selftest`: **`[selftest] OK`**, exit 0
  (`raw/gate-selftest-20261118.log`).
- Reproduction: `run --cells 2:15:6,2:21:7,3:9:3,3:15:5 --seed 1262698074
  --keep 3 --cmax 4 --ffd-max 5 --dres-max 5 --arms orbit,random,subspace
  --crosscheck` → `raw/repro-20261118.jsonl`. All **36 rows identical** to
  `raw/pilot.jsonl` on every field except `secs`, `loadavg`, `threads`
  (same `d_F`, `d_ff_proxy`, `d_res`, per-degree closure and Macaulay
  rows/cols/ranks, bases, targets, kept-draw counts):
  `raw/repro-compare-20261118.txt`.
- **Disclosure (pilot provenance).** The pilot rows were produced with
  `--ffd-max 5 --dres-max 5` (their Macaulay profiles stop at degree 5 and
  `d_res_at_least = 6`), not the `6`/`6` written in `README.md`. A first
  reproduction with `6`/`6` (`raw/repro-ffd6-20261118.jsonl`) matched every
  `d_F`, `d_ff_proxy`, `d_res`, closure dimension, basis and target, and
  differed only in the extra degree-6 Macaulay profile entry and
  `d_res_at_least` (7 vs 6). The pod keeps the frozen `6`/`6`.

### A1.3 Findings that change the frozen ladder (before any pod data)

1. **`3:21:7` runs at `ℓ_eff = 8`, `n_vars = 45`, not 42.** With the orbit
   arm requested, the instrument's greedy orbit search (`orbit_invariant_basis`,
   seed `FNV(master,"ell_eff",…)`) lands on dimension 8 (probe: `ell_eff=8,
   kept 1/1 inconsistent draws in 24 attempts`). All three arms are matched to
   8, so the cell is the prior program's `(21,7)` diagonal plus one.
2. **`3:25:7` has no orbit arm** (true H-avail): the 2-cyclotomic cosets mod
   25 have sizes 1, 4, 20, so `τ`-invariant dimensions are
   `{1,4,5,20,21,24,25}`, none in `[7,10]`. The binary skips the whole cell
   when `orbit` is requested ("no orbit subspace in [7, n) … (H-avail)").
   The job runs it as `random,subspace` (as the pilot did for prime `n`).
3. **`3:27:9`: the orbit arm is skipped by the instrument, not by algebra.**
   A dimension-9 `τ`-invariant subspace exists (`GF(2^9)` itself; cosets
   mod 27 have sizes 1, 2, 6, 18). The greedy search samples orbits of
   8,000 random elements of `GF(2^27)`, which almost surely generate
   modules of dimension ≥ 18, so it never finds 9 and reports H-avail. This
   is an **instrument limitation**, recorded here, not fixed (instrument
   frozen). The job records the skip and runs `random,subspace`.
4. **The frozen `cmax = 5` P1 is roughly two orders of magnitude more
   expensive than the frozen table said.** The same closure code
   (`closure_par`, verbatim from `hky_par`) has pod measurements in
   `research/fall_degree_bounds_20261008/`:
   - degree 4, not falling: `n_vars = 42` 137 s (16 thr, m7a) / 90.8 s
     (32 thr, EPYC 9575F); `n_vars = 48` ~780 s (16 thr); `n_vars = 54`
     ~510 s (16 thr); max RSS of those E10 jobs 26.7 and 31.2 GB (they
     included `n_vars = 60` degree-4 closures, 523,686 columns);
   - degree 5 at `(3,21,7)`, `n_vars = 42`: **37,800 s per draw on 32
     threads**, full rank 974,982 columns (E14 r2).
   At degree `c` the closure that falls holds an upper-triangular dense
   basis of `N_c = Σ_{k≤c} C(n_vars,k)` columns, about `N_c²/16` bytes:
   `n_vars = 45`: `N_5 = 1,385,980` → 120 GB; `n_vars = 46`: 1,550,201 →
   150 GB; `n_vars = 54`: 3,505,051 → 768 GB. Time scales as `N_c^{2.5…3}`.
   So one degree-5 row at `n_vars = 45` costs **~25–30 h on 32 vCPU** and
   **130–190 GB**; the frozen P1 (`keep 6` × 3 arms × 3 cells at `cmax = 5`)
   is ~10³ pod-hours and `3:27:9` at degree 5 cannot fit 256 GB at all.
5. Local calibration of `3:21:7` (`cmax 4`, keep 1, 3 arms) was attempted on
   the shared 4-vCPU container (`raw/calib-S1-3-21-7-20261118.*`); load
   average rose to 14–22 from other workers; it hit its 2,700 s cap with
   **0 rows** (the first arm's draw search plus degree-4 closure had not
   finished; rc 124). Its wall time is therefore **not** used for the time
   bands, which rest on the E10/E14 pod timings above. Its peak RSS
   (2.62 GB, during the degree-4 closure at `n_vars = 45`) is used for the
   degree-4 memory band. `2:29:14` (keep 1, cmax 5) ran in 362 s with
   peak RSS 0.30 GB; both arms read `d_F = 4` (`raw/calib-P2-2-29-14-20261118.jsonl`,
   `raw/calib-summary-20261118.txt`).

### A1.4 Staged pod job (`pod/job.sh`)

One binary process per cell, cheapest first; each cell has a wall budget and
a memory guard (RSS > cgroup/MemTotal limit − 8 GB, or MemAvailable < 4 GB →
SIGTERM, then SIGKILL after 10 s, row `censored_mem`); budget overrun →
`censored_time`; the whole-job cap `TOTAL_BUDGET_H` → `censored_total_budget`
(rerun on resume). The binary's per-(draw, arm) rows stream to
`rows/<cell>.jsonl` (stdout is line-buffered, so finished draws survive a
stop). Resumable; dry-run runs `2:23:11` keep 1 only.

**Stage order and predictions (32 vCPU, EPYC-class; band = plausible range,
written before any pod data):**

| # | cell id | cells | arms | keep | cmax | pred. wall | band | pred. peak RSS | band |
|--:|:--|:--|:--|--:|--:|--:|:--|--:|:--|
| 1 | P2_2-23-11 | 2:23:11 | random,subspace | 6 | 5 | 2 min | 0.5–8 min | 0.1 GB | 0.05–1 |
| 2 | P2_2-25-12 | 2:25:12 | random,subspace | 6 | 5 | 5 min | 2–20 min | 0.2 GB | 0.1–1.5 |
| 3 | AA2a_2-25-12 | 2:25:12 | random (seed a) | 12 | 5 | 5 min | 2–25 min | 0.2 GB | 0.1–1.5 |
| 4 | AA2b_2-25-12 | 2:25:12 | random (seed b) | 12 | 5 | 5 min | 2–25 min | 0.2 GB | 0.1–1.5 |
| 5 | P2_2-29-14 | 2:29:14 | random,subspace | 6 | 5 | 25 min | 10–70 min | 0.4 GB | 0.2–2 |
| 6 | S1o_3-27-9 | 3:27:9 | orbit,random,subspace | 6 | 4 | <1 min (instrument skip, finding 3) | 0–5 min | <0.1 GB | — |
| 7 | S1_3-21-7 | 3:21:7 (ℓ_eff 8) | orbit,random,subspace | 6 | 4 | 1.5 h | 0.7–3.5 h | 4 GB | 2.5–8 |
| 8 | AA1a_3-21-8 | 3:21:8 | random (seed a) | 12 | 4 | 1 h | 0.5–2.5 h | 4 GB | 2.5–8 |
| 9 | AA1b_3-21-8 | 3:21:8 | random (seed b) | 12 | 4 | 1 h | 0.5–2.5 h | 4 GB | 2.5–8 |
| 10 | S1_3-25-7 | 3:25:7 | random,subspace | 6 | 4 | 1.2 h | 0.5–3 h | 5 GB | 3–10 |
| 11 | S1_3-27-9 | 3:27:9 | random,subspace | 6 | 4 | 1.2 h | 0.5–3 h | 14 GB | 8–32 |
| | **base total** | | | | | **~7 h** | 3.5–16 h | **max ~14 GB** | ≤ 32 |
| 12 | P1x_3-21-7 (opt-in `DEG5=1`) | 3:21:7 (ℓ_eff 8) | orbit,random,subspace | **1** | 5 | 27 h per arm reaching degree 5 | 15–45 h per arm; ≤ 3 arms | 160 GB | 130–220 |

- **Stage S (degree-4 screen) is the amendment's replacement for the frozen
  P1 at `cmax = 5`.** It answers the P1 question at the first degree where
  the arms can separate: the prior program saw every random `(3,21,7)`,
  `(3,24,8)`, `(3,27,9)` draw stop below 1 at degree 4 (`d_F ≥ 5`). A row
  `d_F = 4` versus `d_F_at_least = 5` per arm is the screen's reading; a
  censored `≥ 5` reads "undecided above 4", never "flat" (frozen rule).
- **A/A**: as frozen, two `random` runs differing only in the master seed
  (`1262698074` vs `1262698075`), `--keep 12`, at one `m = 2` cell
  (`2:25:12`, `cmax 5`) and one `m = 3` cell at the P1 dimension
  (`3:21:8`, `cmax 4`; `ℓ = 8` so it matches the `ℓ_eff` of row 7).
  Disclosure: with this binary a different master seed also changes the
  targets, so the A/A floor includes target variation (PREREGISTRATION §7a
  asked for basis seed only). That makes the floor wider, i.e. conservative.
- **Degree 5 (row 12) is opt-in and amended to `keep 1`.** The frozen
  `keep 6` × 3 arms is ~450 pod-hours; `keep 1` gives one degree-5 reading
  per arm (an arm that falls at 4 stops there and costs minutes). It needs
  `DEG5=1` and `TOTAL_BUDGET_H` ≥ ~100. Run it only after reading the
  screen, and only for a decision that needs it.
- `3:25:7` and `3:27:9` at `cmax 5` (frozen) stay behind `FULL_P1=1`;
  predicted 150 GB×(1.1–1.6) and ~60 h per row for `3:25:7`, and ≥ 768 GB
  (does not fit any RunPod CPU shape) for `3:27:9`. Not recommended.

### A1.5 Pod shape and cost cap

- **Shape: RunPod `cpu5m`, 32 vCPU / 256 GB** (the E14 r2 shape, EPYC 9575F,
  US$2.08/h). The base plan needs < 32 GB; 256 GB keeps the degree-5 opt-in
  on the same pod (160 GB predicted, guard at 248 GB). `cpu3m` 32 vCPU /
  256 GB at US$1.76/h works too, expected ~1.3–2× slower per closure on the
  older cores, which erases the price gap.
- **Budget cap (base plan): `TOTAL_BUDGET_H=12`** → at most 12 h ×
  US$2.08 = **US$25** (cpu3m: US$21), expected ~7 h ≈ US$15. Add ~0.5 h for
  setup and pulls.
- **Degree-5 opt-in (separate decision):** `DEG5=1 TOTAL_BUDGET_H=100`,
  up to 100 h × US$2.08 = US$208; expected 30–90 h (US$60–190) depending on
  how many arms reach degree 5.

### A1.6 Exact pod command

```
# upload: /workspace/code/job.sh and /workspace/code/binary_falldeg_arms (sha256 above)
TOTAL_BUDGET_H=12 bash /workspace/code/job.sh          # base plan, output /workspace/out
bash /workspace/code/job.sh dry-run                    # smoke test first, output /workspace/out/dryrun
```

Pull `/workspace/out/` (`cells.jsonl`, `rows/`, `progress.log`, `host.txt`,
`final_status`) after every cell; the pod disk is not durable.
