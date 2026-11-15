# Hilbert early stop in the Joux–Vitse sieve descent, p = 1511 (2026-11-12)

Lead: `LEAD-C-jv-sieve-hilbert-descent`, preregistered in
`research/leads_20261111/lane_C_prime_ext_cover.md`. Run at the owner's request
("chase down next leads"). All results are `[pcc:unverified]`: local L0 runs,
unvalidated.

**Verdict: killed by its own preregistered falsifier.** The emitted native counter
rises by more than 1.10×. The counted multiplications fall as predicted, but the
ordering does not hold across the §5a band. The plain sieve row is unchanged.

## What was run

- **Code change** (`src/cryptanalysis/jv_sieve.rs`), opt-in through
  `JV_SIEVE_HILBERT=N`:
  - the descent's F4 options come from `opts_for_mode(24, 600.0, StopMode::Hilbert(N))`
    instead of `opts_for(24, 600.0, stop)`;
  - the report gains `descent_hilbert`, `descent_hilbert_applied`,
    `descent_hilbert_fallbacks` and `descent_f4_native_ops`.

  With the variable unset, behaviour is unchanged.
- **Control (plain arm).** Seeds 17, 20, 27 and 32 were re-run with the committed V2
  command on the patched binary. Every counted field reproduces the committed
  records in `research/jv_sieve_negrho_20261102/results/inputs/ic_1511_sparse.json`
  exactly: total, descent digest, relations.
- **Hilbert arm.** Seeds 17–32, the same command with `JV_SIEVE_HILBERT=16`.
- **Scripts.** `run.sh`, `compare.py`; output in `results/paired.txt` and
  `results/paired.json`.

## Results

| quantity | prediction | measured (16 paired seeds) |
|:--|:--|:--|
| descent digest, relations, solution identical | required | 16 / 16 |
| descent F4 multiplications, ratio | 0.54 [0.50, 0.60]; kill if > 0.70 | **0.407** (0.406–0.409) |
| total counted multiplications (S), paired geometric mean | 0.87 [0.82, 0.93]; kill if > 0.95 | **0.856** (per seed 0.709–0.969) |
| S+ (with the emitted additions) | 0.90 [0.85, 0.95] | 0.961 |
| Hilbert fallbacks | kill if > 25 % | 11 of 20,405 |
| **descent native F4 operations** | **kill if > 1.10×** | **1.139×** (seeds 17, 20, 27, 32: 1.1386–1.1389) |

The plain arm already stops F4 at the staircase (`--stop 64`). The Hilbert rule
saves more F4 multiplications than that (0.41×), but its Hilbert-function checks add
native operations.

## §5a band

Cost in group additions = counted muls / `c_add_e` + w × descent native ops. The
other unpriced counters are identical in both arms.

| seed | w = 0 | w = 0.1 | w = 1 |
|--:|--:|--:|--:|
| 17 | 0.819 | 0.944 | 1.096 |
| 20 | 0.709 | 0.927 | 1.101 |
| 27 | 0.791 | 0.939 | 1.097 |
| 32 | 0.832 | 0.947 | 1.095 |

Hilbert/plain is below 1 at w = 0 and w = 0.1 and above 1 at w = 1. The order
flips inside the band, so the comparison is **unresolved**, not a win.

## Reading

- On the sieve route the descent's native F4 work is large next to its counted
  work. At p = 1511 it is about 6.7e7 native operations against about 1.0e7
  group-addition-equivalents of counted multiplications per run. Any change that
  trades multiplications for checks therefore has to be priced at w = 1 to rank.
- On the Nagao arm (`research/jv_nagao_hilbert_20261112/`) the same stop *lowered*
  the native counter (0.983×), against a baseline with no stop at all. So that
  result holds across the band.
- The plain sieve row is unchanged.
- The patch stays in, opt-in only. A cheaper Hilbert check, such as an incremental
  Hilbert-function update, would be the lever to revisit this lead.
