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

Limiter at A≈0.97: "near the roofline; what remains is the algorithmic axis."
Next ticks: raise **floor:m=3** (A≈0.37) — still the highest e2e-relevant
constant-factor arm's optlevel gap; declare intensity / tighten Bloom+lookup.

Stop the `/loop` when the best probe arms are stable within the noise floor
across two ticks, or when the binding roof is memory and intensity cannot rise
without algorithm change.

## Claim hygiene

Attainments are `[pcc:unverified]` assertions. They are **not** speedups,
not G3, not security claims.
