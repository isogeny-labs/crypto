# Wide target batching for prime index calculus (2026-10-28)

Lead `ICP-prime-ic-constant-engineering` (lead atlas, crypto-autoresearcher): step many target walks in lock-step so that
one field inversion serves a whole round. Round 3 measured 16 lanes; this study asks whether 64, 256 and 1024 lanes do
better. Report: [`REPORT.pdf`](REPORT.pdf) (source `report.html`, figure `wall-ratio.svg` / `.png`). Benchmarks entry:
[`benchmarks/20261028-ic-wide-batching`](../../benchmarks/20261028-ic-wide-batching/).

## Method

`ic.pipeline` with `mitm-prime-batch:m=2` and a base of 128/256/512/1024 abscissae at 20/24/28/32 bits; only `targets`
changes: `walk` (incumbent), `walk-prime-fast`, `walk-prime-batch-16/64/256/1024` and `walk-prime-serial-1024` (the same
1024 walks, additions one by one). `rho.negation` is the matched reference. Three sessions per size, every arm alternated
within each session, four targets and three rounds each. Ratios are paired geometric means over the same (session,
target, round).

## Results

Relation-phase wall time against the incumbent walk (below 1 = faster):

| bits | 16 lanes | 64 | 256 | 1024 | serial 1024 |
|--:|--:|--:|--:|--:|--:|
| 20 | 1.172 | 1.415 | 3.139 | 10.283 | 10.662 |
| 24 | 0.838 | 0.894 | 1.203 | 2.751 | 3.669 |
| 28 | 0.753 | 0.765 | 0.741 | 1.024 | 1.819 |
| 32 | 0.573 | 0.594 | 0.669 | 0.716 | 1.325 |

- Sixteen lanes is the knee; wider does not pay, and at 28 bits 1024 lanes is no faster than the walk.
- The shared inversion is the mechanism: the serial control is slower than the walk at every size.
- Counted S cannot fall with width (every lane's start-up and in-flight work is charged): paired S at 20 bits is
  1.06 (16 lanes) to 3.77 (1024 lanes).
- `walk-prime-fast` is about 1.5x slower than the plain walk at every size, with equal S. Worth a look; not changed here.

The lead's predicted 2x came from a regime where one inversion per target dominates; at 16 lanes it is already shared.
The wide-batching item closes as measured. The constant gain that exists is the 16-lane one.

## Validation

- Every run verified. `ecbench verify --replay-all` re-executes every measured run of all 12 sessions
  ([`replay-certificate.json`](replay-certificate.json), receipts in `verify/`): 1,152 replays, all identical.
- [`validator.py`](validator.py) recomputes all 72 headline ratios from the raw records without sharing code with
  `summarize.py` ([`validator-report.json`](validator-report.json): pass).
- The section 5a band of each size is in `band/` (tables in `band/raw/`).
- `python3 scripts/validation_gate.py evidence benchmarks/20261028-ic-wide-batching`: valid.

## Reproduce

```sh
python3 make_specs.py
OUT=sessions ./run_ladder.sh            # BIN=path/to/ecbench
for s in sessions/*/; do ecbench verify --dir $s --replay-all --exit-code --out verify/$(basename $s).verify.json; done
python3 make_certificate.py && python3 summarize.py && python3 validator.py
```

Host: Intel Xeon @ 2.10 GHz (family 6 model 207, AVX-512), 4 vCPU cloud container, `--cpus auto --allow-busy` (below L2). Binary: ecbench release build of branch `lead/ic-residual-lp-20261028` (sha256 in every receipt).
