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
