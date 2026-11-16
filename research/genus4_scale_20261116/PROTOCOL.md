# Genus 4: more curves at p = 8209, a larger prime p = 16411

Frozen 2026-11-16 before any measured run (`frozen.sha256`, `binary.sha256`).
Every number is `[pcc:unverified]` until a validator replays it.

## 1. Question

`research/genus4_rho_refs_20261116` found arm w below the cheapest rho at
w = 0, 0.1 and 1 at p = 8209 under the owner's primary reading (AGENTS.md §5a,
2026-11-16: every emitted native operation on both sides; reading Q3), on 3
curves × 3 targets. Two limits: 3 curves, and one prime. This study adds 3 new
curves at p = 8209 and 3 curves at p = 16411, with the same instrument.

## 2. Instances

- p = 8209: the next three admitted curves after seed 407 (`h ≤ 2`, the
  r-bound admission rule; seeds 408 onward, in order).
- p = 16411: the first admitted curves (`h ≤ 2`) in seed ranges 1–2500 (two
  curves) and 2501–5000 (one curve), searched by two processes in parallel.
- 3 targets per curve (target seeds `cs`, `cs + 1000003`, `cs + 2000006`),
  solver seeds 1–2, `--calibrate`.

## 3. Arms

- w (double large prime + prefilter + coordinate Wiedemann + capped unit-T
  walk) and R (negation rho) on every target.
- P (plain rho) on the first target of each curve (seeds 1–2), to check that
  negation rho is still the cheapest rho. If P is cheaper than R in a reading
  at a prime, the reference for that reading and prime is P's cost scaled to
  all targets by the per-curve P/R ratio, and this is reported.

Commands (binary `binary.sha256`, the rho-reference study's build):

```
# search + measure, one invocation per process
$BIN run --primes 8209 --genera 4 --curves 3 --targets 3 --seeds 2 --arms wR --cc 2.0 --max-h 2 \
  --threads 1 --rho-cap 1.2e8 --calibrate --curve-seeds 8209=408/409/.../4000 --out $d/p8209.jsonl
$BIN run --primes 16411 --genera 4 --curves 2 --targets 3 --seeds 2 --arms wR ... --curve-seeds 16411=1/.../2500 --out $d/p16411a.jsonl
$BIN run --primes 16411 --genera 4 --curves 1 --targets 3 --seeds 2 --arms wR ... --curve-seeds 16411=2501/.../5000 --out $d/p16411b.jsonl
# then P on the admitted seeds, target 1
$BIN run --primes P --genera 4 --curves K --targets 1 --seeds 2 --arms P ... --curve-seeds P=<admitted> --out $d/<p>_plain.jsonl
```

`--rho-cap` is 1.2e8 at 8209 and 5e8 at 16411 (expected negation steps there
about √(π r / 4) ≈ 2e8).

## 4. Gates

- G1: every log certified and equal to the planted value.
- G2: every admitted instance has `h ≤ 2` and `r` prime (the binary's
  admission), and the p = 8209 instances are distinct from seeds 363, 393, 407.

## 5. Readings and statistics

Readings Q1–Q4 and Qcal exactly as `research/genus4_rho_refs_20261116/PROTOCOL.md`
§3; **Q3 is the primary reading** (owner rule). Per prime: ratio of mean arm-w
cost to mean cheapest-rho cost; bootstrap over targets, then seeds (2000
resamples, 95%). At p = 8209 the six-curve pool combines these new rows with
the rho-reference study's rows (same binary, same rules), and the new three
curves are also reported alone.

## 6. Predictions (before data)

- P1: at p = 8209, new curves alone, w below cheapest rho at w = 0, 0.1, 1 in
  Q3 (upper bound < 1).
- P2: at p = 16411, w below cheapest rho at w = 0, 0.1, 1 in Q3; w = 1 point
  estimate in [0.05, 0.20] (8209's 0.142 times about `(2)^{-1/2}`).
- P3: Q1 at p = 16411, w = 1: point estimate in [0.20, 0.45].
- P4: P/R ≥ 1.2 in every reading at both primes (negation stays the cheapest).

## 7. Decision rule

The cell at a prime is reported as below rho across the §5a band (primary
reading) when w's upper bound is below 1 at w = 0, 0.1 and 1 in Q3 against the
cheapest rho; other readings are reported as sensitivities. Positive control
of a known asymptotic effect; not an ECDLP result.

## 8. Deviations

Local run on a shared host (no AWS identity, so not isolab-eks); a validator
replay and the quartic-test build run on the same host in parallel (counts are
deterministic; timings in `--calibrate` are informational except in Qcal); no
S3 checkpoint or raw upload; no Conductor client.
