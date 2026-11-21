# Rootfind 10× wall + exponent probe (2026-11-21)

`[pcc:unverified]`. Public synthetic planted-log fixtures only. No security,
key-recovery, asymptotic sub-rho, or deployed-curve claim.

## Question

Can the stacked floor / kmitm engineering (PLAN levers 1–5) deliver ~**10×
further** cold planted-log e2e wall vs `rootfind-subtract:m=3`, and does any
arm **move the G3 exponent** (slope of log₂(charged adds/target) vs log₂(|Fₓ|)
below the pair-table floor of 1)?

## Method

Example: `examples/rootfind_10x_exponent.rs`

```bash
cargo run --release --example rootfind_10x_exponent -- \
  --bits 28 --sizes 40 --multi 8 --repeats 7
```

- **Cold e2e:** independent prepare+walk per planted DLP; same seed per rep
  across arms for paired wall ratios.
- **Shared prepare:** secondary (amortized multi-DLP); not the primary 10× claim.
- **G3 ladder:** fixed bits=28, vary |Fₓ| ∈ {8…64}; drop cells with hit-rate
  >25%; require slope ≤ floor − 0.15 to flag `below_floor`.

## Results (host: 4 vCPU Xeon, release)

### 10× wall — **pass** (paired)

Focus cell **bits=28, |F|≈40**, 7–9 cold repeats, verified recovered logs:

| Arm | Median wall | Median charged S | Paired median wall vs subtract |
|---|---:|---:|---:|
| `subtract:m=3` | ~0.175–0.226 s | ~887–1021 | 1× |
| `floor:m=3` | ~0.10–0.11 s | same as subtract | ~2.2× |
| **`kmitm:m=4`** | **~0.0186 s** | **~46.5** | **~13.7×** (9-rep campaign ~14.1×) |
| `kmitm:m=5` | ~0.107 s | ~30.9 | ~1.5× (lower S, heavier prepare) |
| `kmitm:m=4,balanced` | ~0.24 s | ~1312 | ~0.95× |

Primary claim: **`rootfind-kmitm:m=4` (probe-favoring) is ≥10× cold wall vs
`rootfind-subtract:m=3` at this recipe**, with paired median ≈13.7–14.1× and
all reps verified. Median-of-medians alone is noisier (subtract variance);
paired ratios are the authoritative wall figure.

Artifacts: `probe_a40.txt` (9 reps), `probe_a40_m5.txt` (7 reps + m=5).

### G3 exponent — **fail** (fail-closed)

| Arm | adds slope vs log₂\|Fₓ\| | below floor (−0.15)? |
|---|---:|---|
| subtract / floor m=3 | ≈0.999 | no |
| kmitm:m=4 | ≈0.994 | no |
| kmitm:m=5 | ≈0.956 | no (early-exit flattening at high hit-rate cells; still ≥0.85) |
| kmitm:m=4,balanced | ≈2.045 | no (enumeration floor) |

Classification: **engineering**. No G3 pass; the pair-table floor still binds
oracle charged-adds growth.

### Secondary: e2e charged-S vs |F| (not G3)

`e2e_s_ladder.json` — cold median S vs A at bits=28:

| Arm | slope log₂(S) vs log₂(A) |
|---|---:|
| subtract / floor m=3 | ≈ −1.04 |
| kmitm:m=4 | ≈ −2.15 |

kmitm’s e2e S falls faster with |F| because the **different search** (m=4
left-heavy) yields relations sooner — a pipeline yield effect, **not** a G3
pass and **not** a sub-rho claim. Absolute S at A=40 is ~15–22× lower than
subtract; that is why wall moves, not a changed oracle exponent.

## Claim hygiene

- 10× is a **wall-time** engineering result at a fixed synthetic recipe.
- G3 remains on the floor; do not promote ledger rows as exponent advances.
- Multi-target / shared-prepare numbers stay secondary.
- At **bits=32** (miss-heavy ladder, `probe_b32_g3.txt`) every arm sits on the floor (slopes ≈1.000 / 0.993); the bits=28 m=5 dip to 0.956 was hit-rate flattening, not a G3 pass.

Replay: rebuild the example and re-run the command above.

## Files

| File | Role |
|---|---|
| `probe_a40.txt` | 9-rep cold focus (12× median-of-medians, paired ~14×) |
| `probe_a40_m5.txt` | 7-rep + m=5 + paired JSON |
| `e2e_s_ladder.json` | A∈{16…64} e2e S / wall ladder |
| `cold_a_sweep.json` | earlier A sweep |
| `probe.txt` / `probe_v2.txt` | earlier multi-bit probes (hit-rate confounded; superseded) |
