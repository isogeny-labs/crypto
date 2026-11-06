# Protocol: Cheon comb against matched rho on 14 sizes, 36 to 62 bits

Frozen before any session of this study runs. Amendments are appended with a
date and never edit this text.

## 1. Question

In [`research/cheon_ecbench_20261022/`](../cheon_ecbench_20261022/) and
[`research/cheon_ecbench_medium_20261106/`](../cheon_ecbench_medium_20261106/),
the Cheon comb arms and matched `rho.negation` were measured on six
medium-tier sizes (36 to 56 bits, 4-bit steps) with 8 planted targets each.

The fitted cost exponents (S against r) have overlapping 95 % intervals:

| arm | exponent |
|:--|:--|
| comb n−1 | 0.450 [0.406, 0.473] |
| rho n−1 | 0.496 |
| comb n+1 | 0.428 [0.389, 0.460] |
| rho n+1 | 0.513 |

So no exponent separation is established.

**Question:** with more sizes, a wider size range and more targets, do the
exponent intervals of comb and matched rho separate, in each family?

This is a cost of the DLP with auxiliary inputs (a counted static-DH oracle).
It makes no plain-ECDLP or index-calculus claim.

## 2. Design

**Curves.** `curves.json` is written by
`cheon_p224_shape gen --bits 36,38,40,42,44,46,48,50,52,54,56,58,60,62`
(`examples/cheon_p224_shape.rs`). The `--bits` option is added in this
study's commit; the construction is unchanged:

- p = next_prime(3·2^(B−2));
- a random (a, b) is drawn from the SplitMix64 stream seeded by (B, family);
- the first curve is kept with prime n, n ∓ 1 = d·q (q prime) and
  |log d / log n − ρ| ≤ 0.015, where ρ = 0.12943 (n−1) or 0.21106 (n+1).

There are 14 sizes per family. 62 is the largest size the generator accepts:
`prime_explicit` needs p < 2^62, so 64 bits is impossible.

**Check already done (P0, below).** Before this protocol was frozen, the
generated 36 to 56-bit curves were compared field by field with the curves of
`research/cheon_ecbench_20261022/spec-{minus,plus}.json`. All 6 of 6 per
family are equal, and all 12 equal `research/cheon_p224_shape_20261022/curves.json`.

**Specs.** `spec-minus.json` and `spec-plus.json`:

| | value |
|:--|:--|
| arms | `rho-neg` = `rho.negation` (reference), and `cheon-comb` = `cheon.pminus1_comb` with d = `shape:0.12943:0.015`, or `cheon.pplus1_comb` with d = `shape:0.21106:0.015` |
| targets | 24 per curve, `target_seed` 20261022 |
| measurement | 1 round, 0 warmup, alternate order, seed 1, L0, timeout 3600 s |

That gives 14 × 24 × 2 = 672 runs per session.

**Running.**

```sh
ecbench run --spec spec-minus.json --out sessions/minus-20261106 --cpus 2 --lock <scratch>/cheon-wide-minus.lock --allow-busy
ecbench run --spec spec-plus.json  --out sessions/plus-20261106  --cpus 3 --lock <scratch>/cheon-wide-plus.lock  --allow-busy
```

- **Concurrency.** The two sessions run at the same time on separate CPUs,
  each with its own lock, so they bypass the shared host lock.
- **Why this is safe here.** These are L0 counts-only sessions: operation
  counts are deterministic, and the medium study's P1 found 272 of 272 counts
  identical on re-run. No wall time is admitted, and every report states this
  deviation.
- **Audit.** Each session is then audited with
  `ecbench verify --dir <session> --replay-all --out sessions/<name>.audit.json`.

## 3. Analysis (fixed now)

**Fits.** `fit.sh` runs `ecbench bound fit` for each session, arm and w ∈ {0, 0.1, 1}:

- whole session, no `--tier` and no `--curve`, fit version 2;
- `--oracle` set to `static_dh:n-1:d=shape(0.12943,0.015)` or
  `static_dh:n+1:d=shape(0.21106,0.015)`;
- `--weight w` for w > 0.

That gives 12 records, `docs/bounds/records/cheon-ecbench-wide-<family>-<arm>[-w<w>].json`.
The frontier is then rebuilt with `ecbench frontier build`.

**Exponent.** The quantity compared is the fitted exponent α of `S` (the
record's `fit.alpha` and `fit.alpha_ci95`), at each weight.

## 4. Predictions and decision rules

- **P0 (curves).** Met before freezing: see §2.
- **P1 (positive control).** For targets 0–7 on the six sizes shared with the
  medium study, each run's counts equal the medium session's run with the same
  arm, curve and target index:
  - the counts compared are outcome, `cost.total_gae`, `S`, oracle queries,
    oracle server work and every per-phase count;
  - a mismatch is reported per run, and does not stop the study.
- **P2 (fits).** All 12 fits are accepted with no admissibility reason, and
  procedure 2 ranks them.
- **P3 (decision rule, per family).** "Exponent separated" means the lower end
  of rho's α interval is above the upper end of comb's α interval at all three
  weights. Otherwise the result is "not separated". Both outcomes are reported
  with the intervals.
- **Predictions, written before data:**
  - n+1 separates; n−1 is uncertain, about even odds;
  - rho α lies in [0.45, 0.55];
  - comb α lies in [0.40, 0.47] for n−1 and [0.36, 0.45] for n+1.
- **Also reported:** the gap to matched rho as a pooled paired ratio of `S`
  over the 12 cells (2 families × 3 weights × q ∈ {d, √r}), as the original
  report did; the per-size rho gate; and every timeout.
- **Stop rule.** A run that times out stays in the tables and the fits treat
  it as `ecbench bound fit` does. No size, target or arm is added or removed
  after any data is seen.

## 5. Outputs

- `sessions/` holds the two sessions and their audit receipts.
- `REPORT.md` and `report.pdf` give the outcome of P0–P3, the α intervals and
  the gap table.
- `alpha.svg`, a graph of α with its intervals per arm, family and weight,
  is drawn from the records.
- The run report goes in `reports/`.
- The 12 records, plus the rebuilt `docs/bounds/frontier.json` and `FRONTIER.md`.
- A `benchmarks/` row and `bench.record/v1` record.
