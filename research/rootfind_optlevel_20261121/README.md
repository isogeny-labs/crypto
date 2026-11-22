# Rootfind optlevel (2026-11-21)

How close the prime rootfind **probe** kernels run to this host's
`modmuladd_p62` ceiling. Engineering only — not a speed claim, not a
benchmark / leaderboard row (`docs/OPTIMIZATION_LEVEL.md` §0).
`[pcc:unverified]`, shared 4-vCPU VM, tier **T-timing** (no PMU).

## Host

- Fingerprint `d0efb1838c03e9e3` (see `optlevel/probe.txt`, `optlevel/calibration.json`)
- `modmuladd_p62` peak ≈ **9.45×10⁸** ops/s/core (scalar ISA best)
- Flags: `VIRTUALIZED`

## Measure

Probe-only timed region (prepare excluded). Ops = charged Montgomery
`muls + sqrs` on the probe path. Example:

```bash
tools/optlevel/run.sh calibrate --out research/rootfind_optlevel_20261121/optlevel/calibration.json
cargo build --release --example rootfind_optlevel_probe
tools/optlevel/run.sh measure --label "…" --repeat 5 \
  --out research/rootfind_optlevel_20261121/optlevel/<slug>.measurement.json \
  -- target/release/examples/rootfind_optlevel_probe \
     --arm kmitm:m=4 --mode ifma --bits 32 --size 32 --targets 4096 --seconds 1.2
tools/optlevel/run.sh score --calibration …/calibration.json \
  --measurement …/<slug>.measurement.json --out …/<slug>.record.json
```

## Baseline records (tick 0)

| Arm / mode | Rate (ops/s) | A_single | Grade | Limiter |
|---|---:|---:|---|---|
| floor:m=3 @ A=64 | 3.54×10⁸ | **0.375** | B/C | core below compute roof |
| subtract:m=3 @ A=64 | 3.40×10⁸ | 0.359 | C | same |
| kmitm:m=4 IFMA @ A=32 | 2.34×10⁸ | 0.248 | C | same |
| kmitm:m=4 lanes8 scalar | 1.71×10⁸ | 0.181 | D | same |
| kmitm:m=4 single | 1.46×10⁸ | 0.155 | D | same |

All `INTENSITY_UNKNOWN` (no `bytes_declared`). Noise floors in the
`*.record.json` files. Roofline: `optlevel/roofline.svg`.

## Improvement loop

Goal: raise attainment toward the binding roof without changing the search
(same charged `S` / same verdicts).

### Tick 1 (landed)

1. IFMA: replace 8× Fermat lane-inv with one batch Mont inv of the product.
2. IFMA: Bloom-reject inside the vector callback (smaller candidate Vec).
3. Auto-disable IFMA only above ~500k left entries (raised after the win band grew).
4. Charge honest IFMA Montgomery muls (lane-ops), not the old `3·|F|·lanes` estimate.

| Metric | Tick 0 | Tick 1 |
|---|---:|---:|
| kmitm:m=4 IFMA A_single | 0.248† | **0.966** |
| kmitm:m=4 IFMA grade | C | **A** |
| wall IFMA vs scalar8 (bits32 A32 m=4) | 1.38× | **1.64×** |
| wall IFMA vs scalar8 (bits32 A32 m=3) | 1.47× | **1.83×** |

† Tick 0 IFMA A used approximate mul charges; not comparable to tick 1's honest
lane-mul count. Wall ns/target is the comparable speed metric.

### Tick 2 (landed) — floor lever 2

`rootfind-floor:m=3` gained `decompose_lanes` (scalar 8-wide) + IFMA lockstep
(canonical Bloom; same IFMA backend as kmitm with floor dx convention).

| Arm / mode | A_single | Grade | vs tick 0 |
|---|---:|---|---|
| floor:m=3 single | 0.406 | B | 0.375 → |
| floor:m=3 lanes8 | 0.483 | B | new |
| floor:m=3 IFMA | **0.922 / 0.925** | **A** | new; two ticks agree within noise |

Limiter on floor/kmitm IFMA: "near the roofline; what remains is the algorithmic
axis."

### Tick 3 (landed) — A_full + scalar prefetch

- Software prefetch on scalar lane FB walks (kmitm + floor): **no gain** within
  noise (lanes still D / B).
- Concurrent copies (`--scale copies:1,4`) for preferred IFMA arms:

| Arm | A_single | A_full | Grade |
|---|---:|---:|---|
| kmitm:m=4 IFMA | 0.957 | **0.954** | **A** |
| floor:m=3 IFMA | 0.925 | **0.921** | **A** |

Preferred IFMA probes are at the compute roof on this host (single and full).
Remaining non-A arms are **fallbacks**: scalar lanes (need SIMD — IFMA is the
fix) and `subtract:m=3` (same search as floor; floor is the optimized path).

### Tick 4 — intensity + subtract status

Declared `--working-set-bytes 524288 --bytes-per-op 16` for kmitm IFMA
(tables fit in L3 → `in_cache: true`, DRAM traffic charged as 0). Still
**A = 0.958**, binding roof **compute**. `INTENSITY_UNKNOWN` cleared for that
record (`kmitm_m4_ifma_intensity_tick4.record.json`).

`rootfind-subtract:m=3` is the **ledger reference** (same search / same `S` as
floor). Floor is the optimized wall path (lever 1+2). Optlevel attainment for
the m=3 pair-table probe is the floor IFMA row, not subtract's C-grade
single-target path.

## Still open (loop continues)

1. Scalar-lane headroom on non-IFMA hosts (T-timing cannot split overhead vs
   dependency chains without a PMU). Optional — IFMA is preferred when present.
2. Optional: port IFMA onto `rootfind-subtract` itself for arm-id parity
   (functionally redundant with floor).

**Stop criterion:** preferred production probes (kmitm IFMA, floor IFMA) stay
grade A with intensity declared and A_full ≈ A_single. Met on this host.

### Tick 5 (loop stop) — reconfirm

Smoke remeasure with intensity declared: kmitm IFMA **A=0.939**, floor IFMA
**A=0.920**. Stop criterion still holds. Unsubscribed `loop-rootfind-optlevel`.


## Claim hygiene

Attainments are `[pcc:unverified]` assertions. They are **not** speedups,
not G3, not security claims.
