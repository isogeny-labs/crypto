# Rootfind 10× wall + cost-exponent probe (2026-11-21)

`[pcc:unverified]`. Public synthetic planted-log fixtures only. No security,
key-recovery, asymptotic sub-rho, or deployed-curve claim.

## Questions

1. **Wall:** ~10× further cold planted-log e2e vs `rootfind-subtract:m=3`
   beyond the floor / early kmitm breakthroughs?
2. **Exponent:** does any arm move rootfinding **cost growth** — not only a
   constant factor? Two measurements:
   - **G3** (PROTOCOL): slope of log₂(charged adds/target) vs log₂(|Fₓ|)
     below the pair-table floor of 1 (fixed bits, vary |F|).
   - **E2e cost vs bits:** slope of log₂(e2e S) and log₂(wall) vs log₂(bits)
     with each arm’s best measured |F|.

## Method

```bash
cargo run --release --example rootfind_10x_exponent -- \
  --bits 28 --sizes 40 --multi 8 --repeats 7
```

Grid sweeps for `best_recipe_bits_ladder.json` vary bits ∈ {20,24,28,32} and
|F| ∈ {16…64}; three cold repeats; verified recovered logs.

## Results

### 1. ~10× wall — **pass**

| Evidence | Figure |
|---|---|
| Paired cold wall, bits=28 \|F\|≈40, `kmitm:m=4` vs subtract | **≈13.7–14.1×** (7–9 reps) |
| Best-recipe wall, bits=32: `kmitm:m=5` @\|F\|=32 vs best subtract | **≈25.3×** |
| Best-recipe wall, bits=32: `kmitm:m=4` @\|F\|=64 vs best subtract | **≈20.8×** |

Relative wall advantage **grows with bits** (best-recipe ≈1.3× @20-bit →
≈5× @28-bit → ≈21–25× @32-bit), so this is not a fixed constant.

### 2a. G3 (oracle adds vs \|Fₓ\|) — **fail** (fail-closed)

Miss-heavy ladders (bits=28 and bits=32): slopes ≈0.99–1.00 on the
pair-table floor. Classification for G3: **engineering**. No PROTOCOL
advance.

### 2b. E2e cost exponent vs group bits — **moved**

Best |F| per arm (from the measured grid). Slopes of log₂(cost) vs
log₂(bits):

| Arm | wall slope | charged-S slope |
|---|---:|---:|
| `subtract:m=3` | ≈15.8 | ≈8.07 |
| `floor:m=3` | ≈15.7 | ≈7.92 |
| **`kmitm:m=4`** | **≈9.9** | **≈2.05** |
| **`kmitm:m=5`** | **≈8.6** | **≈1.32** |

kmitm’s e2e charged-S grows far more slowly with group bits than subtract at
matched best-|F| recipes. Absolute S ratios vs best subtract rise from
~2–8× (20–24 bit) to ~19–42× (28 bit) to ~42–89× (32 bit). That is a
**changed e2e cost exponent vs problem size**, not a constant-factor win.

This is **not** a G3 pass and **not** a sub-rho / security claim. It is a
fail-closed measurement that the planted-log pipeline’s charged cost law vs
bits moves when switching subtract:m=3 → kmitm:m=4/5 with recipe retuning.

## Claim hygiene

- Primary wall claim: cold e2e, paired or best-recipe, synthetic curves.
- G3 remains fail-closed on the pair-table floor.
- Exponent claim = e2e S/wall vs bits (best |F|), explicitly separated from G3.
- No ledger promotion as asymptotic IC-vs-rho advance.
- Replay: example binary + `SCOREBOARD.json` / `best_recipe_bits_ladder.json`.

## Files

| File | Role |
|---|---|
| `SCOREBOARD.json` | Canonical gates + slopes |
| `best_recipe_bits_ladder.json` | Full best-|F| grid + slopes |
| `bits_cost_ladder.json` | Fixed \|F\|=40 bits ladder |
| `probe_a40.txt` / `probe_a40_m5.txt` | Paired 10× wall campaigns |
| `probe_b32_g3.txt` | G3 fail-closed @ bits=32 |
| `e2e_s_ladder.json` | S vs \|F\| @ bits=28 (secondary) |
| `examples/rootfind_10x_exponent.rs` | Harness |

## Pinned recipe panel (re-run)

`--panel recipe` on `examples/rootfind_pipeline_e2e.rs` (5 reps, bits=32):

| Arm | Median wall | vs subtract@64/walk |
|---|---:|---:|
| subtract:m=3@64/walk | 1.55 s | 1× |
| floor:m=3@64/walk | 0.81 s | ~1.9× |
| **kmitm:m=4@64/walk** | **0.090 s** | **~17×** |
| **kmitm:m=5@32/walk** | **0.068 s** | **~23×** |
| kmitm:m=4@64/lanes8 | 0.33 s | ~4.7× |
| kmitm:m=5@32/lanes8 | 0.24 s | ~6.5× |
| ifma:m=3@64/batch16 | 1.18 s | ~1.3× |

PLAN lever 2: scalar 8-wide `decompose_lanes` landed; AVX/IFMA lockstep for
kmitm still open. G3 still fail-closed (`G3_ATTEMPT.md`). Still `[pcc:unverified]`
(no isolated runner in this environment).
