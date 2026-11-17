# F1 implementation spec

For the executor. Every research choice is made in [`PROTOCOL.md`](PROTOCOL.md); this file says
exactly what to run, where, and what to check. If anything here seems to need a choice, stop and
write the question in `DEVIATIONS.md`; do not decide it.

Follow-up directory (write only here): `D=/home/user/crypto/research/g2plus_followups_20261117/F1_L03_base_constant_lab_instrument`.
No git commands that change state. Do not edit, rebuild or run git in `/home/user/crypto-c0b657b`.

## 1. Files (already present; do not edit after freeze)

| path | what | notes |
|---|---|---|
| `PROTOCOL.md`, `SPEC.md`, `PILOT.md`, `frozen.sha256` | frozen | `sha256sum -c frozen.sha256` must print OK for all three before anything runs |
| `f1.py` | gates, analysis, self-test; Python 3 standard library only | written and tested by the coordinator; sha256 recorded in PILOT.md §0 |
| `tools/rt.py` | runs a command, prints `LABEL exit=N cpu_s=X maxrss_kb=Y` | copy of `pilot/rt.py` |
| `ref/instances_L03.jsonl` | the nine instance lines | byte copy of `g2plus_leads_20261116/L03_base_rebalance/instances.jsonl`, sha256 `91ddea89649a852f9b8c73e6a7affa4e2adde701624db0700e282f0846517148` |
| `pilot/` | pilot outputs | never read by `f1.py analyze` |

No code is copied from yesterday's lead directories (only the data file above). No new Rust.

## 2. Instrument

`B=/home/user/crypto-c0b657b/target/release/genus_ladder`. Before the first run:
`sha256sum $B` must print `50e5986e8e80ee98d7d80c9cbedc169f7b732c84d571bee9b74acbd123a030ff`.
If not, stop (VOID, write `DEVIATIONS.md`).

## 3. Formulas (implemented in `f1.py`; listed so a reader can check them)

- `nb(p, cc) = min(ceil(cc * float(p) ** (1.0 - 1.0/3.0)), p)`.
- Bands: lo `(I, kappa) = (1, 1)`, mid `(10, 9)`, hi `(30, 9)`.
- IC run: `FPM = cantor_mul + I*cantor_inv + rel_mul + kappa*la_mul`;
  relation part `cantor_mul + I*cantor_inv + rel_mul`; LA part `kappa*la_mul`.
- Rho run: `FPM = cantor_mul + I*cantor_inv`.
- Over a list K of keys `(curve_seed, seed)`: `G(K) = 1 − Σ_K FPM(cc=1) / Σ_K FPM(cc=2)`;
  shares and IC/rho are ratios of sums over the same K.
- Bootstrap `hboot`: `random.Random(20261117)`, 4000 resamples; per resample, for each of the
  `n_c` curves draw a curve with replacement, then draw that curve's seed list (length 8) with
  replacement; statistic on the concatenated key list; sort; lo = value at index
  `int(0.025*3999 + 0.5)` = 100, hi = index `int(0.975*3999 + 0.5)` = 3899.
- A/A: within the cc = 2 main file, numerator = even seeds re-keyed `(c, s−1)`, denominator =
  odd seeds `(c, s)`, band mid.
- Gate G9: `|G_AA_mid point| ≤ 0.05` at every prime (no bootstrap).
- Band labels (`analyze`): `band_robust` = every `G_{lo,mid,hi}.lo > 0` at every prime;
  `sign_flip_at_lo` = primes with `G_lo.point < 0`. If a gate fails, `verdict = "void"` and the
  rule's outcome is kept as `verdict_if_gates_passed` (reported, never used as the verdict).
- Decision (`decide`): supported iff `G_mid.point ≥ 0.15 and G_mid.lo > 0.0` at 8209, 16411,
  32771; else killed iff `G_mid(32771).point < 0.05`; else inconclusive. Any gate false →
  `void`.

Counters are the instrument's; nothing is added or moved. Counter placement in the binary
(read-only reference): `rel_mul` includes the character table and the factor-base scan;
`cantor_*` covers jump set-up, walk, restarts and the first certificate multiplication (the
second is taken back out); `la_mul` covers row build, peel and Wiedemann of every attempt
(`genus_ladder.rs` 1276-1530).

## 4. Run procedure

```sh
D=/home/user/crypto/research/g2plus_followups_20261117/F1_L03_base_constant_lab_instrument
B=/home/user/crypto-c0b657b/target/release/genus_ladder
cd $D && sha256sum -c frozen.sha256 && python3 f1.py selftest   # must print "selftest: 47 checks passed"
mkdir -p raw analysis
COMMON="--genera 3 --curves 3 --seeds 8 --max-h 8 --threads 1 --rho-cap 3e7 --max-trials 2e8 --seed-search 600"
run() { # $1 label, rest = args after "run"
  L=$1; shift
  echo "$(date -u +%FT%TZ) $B run $*" >> raw/COMMANDS.log
  python3 tools/rt.py "$L" $B run "$@" >> raw/RUNTIME.log 2>&1
}
```

Stage 1 (primary). Up to two at a time, e.g. the two cc values of one prime in parallel:

```sh
for P in 8209 16411 32771; do
  run p${P}_cc2.0_main --primes $P $COMMON --arms cR --cc 2.0 --out raw/p${P}_cc2.0_main.jsonl &
  run p${P}_cc1.0_main --primes $P $COMMON --arms cR --cc 1.0 --out raw/p${P}_cc1.0_main.jsonl &
  wait
done
```

Stage 2 (determinism A/A):

```sh
run p8209_cc2.0_rerun  --primes 8209  $COMMON --arms c --cc 2.0 --out raw/p8209_cc2.0_rerun.jsonl &
run p16411_cc2.0_rerun --primes 16411 $COMMON --arms c --cc 2.0 --out raw/p16411_cc2.0_rerun.jsonl &
wait
run p32771_cc2.0_rerun --primes 32771 $COMMON --arms c --cc 2.0 --out raw/p32771_cc2.0_rerun.jsonl
```

Stage 3 (secondary; p = 32771 optional, run last):

```sh
for P in 8209 16411 32771; do
  for CC in 0.5 1.5 3.0; do
    run p${P}_cc${CC}_main --primes $P $COMMON --arms c --cc $CC --out raw/p${P}_cc${CC}_main.jsonl
  done
done
```

(The `--cc` text and the file name must match exactly: `0.5`, `1.0`, `1.5`, `2.0`, `3.0`.)

Stage 4 (gates, analysis, hashes):

```sh
python3 f1.py gates raw analysis/gates.json
python3 f1.py analyze raw analysis/analysis.json analysis/tables.md
sha256sum raw/*.jsonl > raw/data.sha256
sha256sum $B > raw/binary.sha256
```

Each `run` must exit 0 (see `raw/RUNTIME.log`). A non-zero exit or a missing `done` row is a
G3 failure; do not rerun with other parameters. If a process is killed by the host, record it in
`DEVIATIONS.md`; a verbatim re-run of the identical command is allowed once (the instrument is
deterministic, G4), and is recorded.

## 5. Output schemas

- `raw/p{P}_cc{CC}_{main|rerun}.jsonl`: the binary's own lines (`config`, `instance`, `cell`,
  `run`, `done`), unmodified. Size is about 1 KB per run (pilot: 1.4 KB for two runs plus
  header), so under 50 KB per file.
- `raw/COMMANDS.log`: one line per command, UTC timestamp then the command; hook refusals
  verbatim.
- `raw/RUNTIME.log`: `LABEL exit=N cpu_s=X maxrss_kb=Y` per process (budget record only; never
  a metric).
- `analysis/gates.json`: `{"all_pass": bool, "gates": {Gk_name: {"pass": bool, "detail": …}}}`.
- `analysis/analysis.json`: `{"per_prime": {"8209": {"G_lo"|"G_mid"|"G_hi": {point, lo, hi,
  pairs, pairs_cc1_cheaper}, "AA_mid": {point, lo, hi, pairs}, "cells": {"<cc>": {cc, nb, runs,
  mean_trials, mean_relations, mean_core_rows, mean_core_cols, mean_attempts, max_attempts,
  mean_cantor_mul, mean_cantor_inv, mean_rel_mul, mean_la_mul, mean_la_add, fpm_lo|mid|hi,
  rel_share_lo|mid|hi, la_share_lo|mid|hi, ic_over_rho_lo|mid|hi: {point, lo, hi},
  G_mid_vs_cc2 (cc ≠ 2)}}, "rho_fpm_mid", "argmin_cc_mid_secondary", "secondary_complete"}, …},
  "decision": {"verdict", "rule", "band_robust", "sign_flip_at_lo", ["verdict_if_gates_passed"]}, "gates_all_pass"}`.
- `analysis/tables.md`: generated by `f1.py analyze`.
- `RESULTS.md` (executor writes by hand from `analysis/tables.md`): verdict first, with the band
  label exactly as PROTOCOL §8 words it, then the G
  table at all bands, A/A, per-cc table, IC/rho as "distance remaining", every gate, every
  deviation; every number tagged `[pcc:unverified]`; negatives stated plainly.

## 6. Unit tests (`python3 f1.py selftest`, expected output `selftest: 47 checks passed`)

| test | input | expected |
|---|---|---|
| T1 | `nb_expected` at (8209: 0.5, 1, 1.5, 2, 3), (16411: 1, 2), (32771: 1, 2, 3) | 204, 407, 611, 814, 1221, 646, 1292, 1025, 2049, 3073 |
| T2 | record `cantor_mul 100, cantor_inv 2, rel_mul 30, la_mul 5` | FPM lo 137, mid 195, hi 235; rho mid 120 |
| T3 | synthetic files: cc = 1 runs have `cantor_mul`, `la_mul` × 0.6 | G_mid = 1 − (2400+1000+900+5400)/(4000+1000+900+9000) = 0.348993…, interval collapses to the point, G_AA = 0, verdict supported |
| T4 | `decide` on hand-made G tables | (0.30, 0.20) everywhere → supported; one prime 0.14 → inconclusive; one prime lo −0.01 → inconclusive; 32771 at 0.049 → killed; 32771 at exactly 0.05 → inconclusive |
| T5 | `gates` on the synthetic set | every gate passes except G6 (synthetic trials ≠ 42975) |
| T6 | one fault injected per case | wrong fb_cols → G2 fails; uncertified run → G3; rerun trials + 1 → G4; rho cantor_mul + 1 in cc = 1 file → G5; instance order + 1 → G1; rerun file deleted → G0; even-seed cantor_mul doubled in one cc = 2 file → G9 |
| T7 | bootstrap twice on the same noisy set | identical tuples; lo ≤ point ≤ hi; point in (0.45, 0.52) |

## 7. What the executor must not do

Change any threshold, seed, prime, cc value, band or file name; drop a run or a curve; rerun
with other parameters; use wall time; add a leaderboard or `benchmarks/` row; run git commands
that change state; edit `/home/user/crypto-c0b657b`.
