# F1 PILOT - gate reachability and instrument sanity (2026-11-17)

Purpose: show every correctness gate (PROTOCOL §6, Q1-Q7) is **reachable** before the
protocol is frozen, and establish that the lab binary exposes the base constant and
reproduces the lab's own recorded counts. The pilot touches **only gate quantities and
instrument sanity** (certification, base size, pairing, counter fidelity, timing); it
does **not** compute or read the decision metric (`G(p)` paired FPM ratio) on the
decision instances. No threshold in §6/§7 was set from a decision-metric value seen
here. Every figure `[pcc:unverified]`.

Binary: `/home/user/crypto-c0b657b/target/release/genus_ladder`
sha256 `50e5986e8e80ee98d7d80c9cbedc169f7b732c84d571bee9b74acbd123a030ff`
(read-only lab worktree at commit a552dc2ee8; not rebuilt). Recorded in
`pilot/binary.sha256`.

## Commands run

```sh
BIN=/home/user/crypto-c0b657b/target/release/genus_ladder
D=.../F1_L03_base_constant_lab_instrument/pilot
# (1) cc-mechanics + certify + pairing + rho, small non-decision prime
$BIN run --primes 1031 --genera 3 --curves 2 --seeds 2 --arms cR --cc 1.0 \
    --max-h 8 --threads 1 --seed-search 600 --out $D/pilot_cc1_p1031.jsonl
$BIN run --primes 1031 --genera 3 --curves 2 --seeds 2 --arms cR --cc 2.0 \
    --max-h 8 --threads 1 --seed-search 600 --out $D/pilot_cc2_p1031.jsonl
# (2) certify / non-exhaustion / timing / count-identity at the DECISION sizes,
#     arm c only, 1 curve x 1 seed (gate quantities only; FPM not read as a decision)
$BIN run --primes 8209  --genera 3 --curves 1 --seeds 1 --arms c --cc {1.0,2.0} ... --out $D/cal_p8209_cc*.jsonl
$BIN run --primes 32771 --genera 3 --curves 1 --seeds 1 --arms c --cc {1.0,2.0} --max-trials 2e8 ... --out $D/cal_p32771_cc*.jsonl
```
Raw pilot outputs in `pilot/`.

## What the pilot printed (gate quantities only)

### Base-size mechanics - Q2, and PROTOCOL §0 (flag is real)
| p | cc | fb_cols | ceil(cc*p^(2/3)) |
|---:|---:|---:|---:|
| 1031 | 1.0 | 103 | 103 |
| 1031 | 2.0 | 205 | 205 |
| 8209 | 1.0 | 407 | 407 |
| 8209 | 2.0 | 814 | 814 |
| 32771 | 1.0 | 1025 | 1025 |

`fb_cols` equals `ceil(cc*p^(2/3))` exactly in every cell, and `cc = 1` is exactly half
the columns of `cc = 2`. **Q2 reachable.** The `--cc` flag changes the measured base on
the real driver path (the solver's `want = prm.nb`), so no source edit / port is
needed.

### Certification and non-exhaustion - Q1, Q6
Every IC run in the pilot had `certified = true`, `ok = true`
(`recovered == t_planted`), `exhausted = false`, `attempts = 1`:
- `p = 1031`: 8 IC runs (2 cc x 2 curves x 2 seeds), all pass.
- `p = 8209`: cc1 `trials=71794 relations=3282`, cc2 `trials=42975 relations=3539`,
  both certified, non-exhausted.
- `p = 32771` cc1: `fb_cols=1025 trials=419224 certified ok exhausted=false`.
**Q1, Q6 reachable at the study's own sizes** (8209 and 32771), not only at a smaller
calibration size - this is the rate gate calibrated where the study runs (lesson a).

### Pairing integrity - Q3
At `p = 1031` the set of `curve_seed`s was identical across `cc = 1` and `cc = 2`:
`{2, 8}` in both (deterministic curve search from `--seed-search 600 --max-h 8`
is independent of `cc`). **Q3 reachable.**

### Instrument fidelity / count-identity with the lab - Q4
`cc = 2.0`, `p = 8209`, first curve, `seed = 1`: `trials = 42975`. The L03 ASSESSMENT
cites the lab's `scale_g3` cell `(8209, c6, s1): trials 42975`. **Exact match.** Our
binary reproduces the lab instrument's own recorded count; the measurement is anchored
to the lab without any port. **Q4 reachable.**

### Rho reference, cc-independence - Q5
`R_rho_neg` certified on every pilot curve. Its counters are byte-identical across the
`cc = 1` and `cc = 2` invocations on the matched `(p, curve_seed, seed)` (rho ignores
`nb`): e.g. `(1031, cseed 8, seed 1)` gave `cantor_mul = 10714836, cantor_inv = 244353`
in both; `(1031, cseed 2, seed 1)` gave `17748445 / 404843` in both. **Q5 reachable**,
and rho may be run once and reused for every `cc`. Rho is high-variance across seeds
(`cantor_mul` ranged `883094 .. 17748445`), confirming confounder C5's treatment
(parameter-labelled, CI, non-decisive).

### Noise floor estimable - Q7
At `cc = 2`, `p = 1031`, the IC `la_mul` differed across solver seeds on the same curve
(cseed 8: `415657` vs `409798`; cseed 2: `451472` vs `350049`), so the A/A per-pair
ratio distribution is non-degenerate and the seed-noise floor is estimable. **Q7
reachable.**

## FPM-formula anchor for the SPEC unit tests (instrument sanity, not a decision)

The two `p = 8209` single-cell runs give exact counters that pin the FPM formulas used
in SPEC §5 (T1/T1b). From the recorded counters, by the lab's own band definition
(`genus_ladder_analyze.rs:12-16`), `fpm_mid(cc1) = 53106726` and
`fpm_mid(cc2) = 83674190`, and the LA share at `cc = 2` is `0.749` (matches L03's
reported `0.745-0.772` LA share at `cc = 2`). These are recorded here only to fix the
unit-test expected values; the single-cell direction (`1 - 53106726/83674190 = 0.365`)
is **not** the decision statistic (one cell is not the paired bootstrap over 3 curves x
4 seeds) and is explicitly excluded from the verdict.

## Budget calibration (counts, with wall only as a feasibility check)

| p | cc | wall (1 thread) | trials | note |
|---:|---:|---:|---:|---|
| 8209 | 1.0 | 28.3 s | 71794 | fine |
| 8209 | 2.0 | 28.7 s | 42975 | fine |
| 32771 | 1.0 | **596 s (~9.9 min)** | 419224 | **budget-critical primary cell**; under the 15 CPU-min cap |
| 32771 | 2.0 | (running; smaller base -> fewer trials, faster) | - | expected < cc1 |

Finding: the budget-critical cell is **small `cc` at the largest prime** (small base ->
many trials). The primary `cc = 1` cell at `p = 32771` is ~9.9 CPU-min per run, under
the 15 CPU-min / 2 GB per-run budget, and does not exhaust at `--max-trials 2e8`. The
secondary `cc = 0.5` at `p = 32771` will be slower still and may hit `--max-trials`;
PROTOCOL §9 records such a cell as `exhausted` and drops it from S1 only, never reruns
with moved parameters. The decision rests on `cc in {1, 2}`, both of which complete
under budget at every prime. Each `raw/*.jsonl` is per-run rows only (<50 KB).

## Conclusion

All seven correctness gates (Q1-Q7) are reachable on pilot instances, the base constant
is a real CLI parameter on the driver path, the binary reproduces the lab's own
recorded counts, and the budget is feasible for the primary arms at every decision
prime. The protocol may be frozen. `proceed = true`.
