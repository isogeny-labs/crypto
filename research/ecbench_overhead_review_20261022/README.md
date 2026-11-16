# ecbench harness overhead review (2026-10-22)

A performance review of the code every ecbench measurement runs through,
and two changes that came out of it. Report: [`REPORT.pdf`](REPORT.pdf)
(also `report.html`), figure: [`solve-time.svg`](solve-time.svg).

## Changes

| change | file | effect |
|:--|:--|:--|
| Parse `docs/curves/registry.json` (4.7 MB) once per process (`OnceLock`) instead of on every registry lookup | `src/cryptanalysis/ecbench/workload.rs` | about 7 ms less per measured child; 3.5–4 ms less inside every timed solve window (see below) |
| `GenericOutcome::count` allocates a counter key only on its first use, not on every call (several per BSGS/kangaroo step) | `src/cryptanalysis/ecbench/generic.rs` | 1.8–2.8 ms less per BSGS solve at 32 bits |

Neither changes a counted result. 2,376 records from the A/B and attribution
sessions are identical to the old binary's outside timing (`summarize.py`);
every headline session re-verifies under the *other* binary with all measured
runs replayed (`verify/`, 1,044 replays, all reproduced); the §5a band is the
same from both binaries (`band/`; the printed tables are in `band/raw/`).

## Headline (old binary vs this change)

`sessions/`: two specs, three sessions per binary, alternated, `--cpus auto
--allow-busy` (below L2). Median over 18 (20-bit) and 45 (32-bit) measured
runs per arm, warm-up excluded.

| spec | arms | solve window new/old | measured child new/old |
|:--|:--|--:|--:|
| `specs/smoke-prime-20.json` (20-bit prime-order curve, every method) | rho ×4, BSGS ×3, kangaroo | 0.03–0.07 | 0.75–0.79 |
| | `ic.pipeline` (mitm) | 0.96 | 0.81 |
| `specs/generic-32.json` (32-bit curve) | rho ×4 | 0.69–0.81 | 0.80–0.85 |
| | BSGS ×3 | 0.64–0.68 | 0.79–0.83 |
| | kangaroo | 0.77 | 0.83 |

Per-arm numbers: `results.json` (`headline_medians`), recomputed independently
by `validator.py` into `validator-report.json`.

## Why the timed solve window moved

Nothing in the solve path reads the registry. Callgrind counts the same
instructions inside the solve window for both binaries (1.110 M vs 1.112 M on
a 20-bit `rho.negation` solve); before the window the old child executes
193 M instructions and the new one 149 M (the second parse). The old binary
drops its freshly parsed copy of the registry just before the solve, and the
3.5–4 ms it then loses inside the window is wall time with no extra
instructions and no system calls (strace). Disabling glibc fast bins
(`GLIBC_TUNABLES=glibc.malloc.mxfast=0`) did not remove it; the exact
memory-system cause was not isolated.

Consequence for existing records: solve wall times recorded before this
change carry a 3.5–4 ms offset on this host, which dominates the window on
small curves (20 bits: 4 ms of a 0.3 ms solve). Counted costs (S, GAE, field
operations) are unaffected, and so is every comparison made in the counted
unit.

## Attribution

`attribution/`: one binary per change (records gzipped), two sessions each.
The registry change accounts for the solve-window and most of the
process-time gain; the counter change for the extra BSGS gain at 32 bits. A
third candidate, a word extended-Euclid inverse in
`prime_fast::inv_mod_u64` (2.3–2.9× faster per inversion than the binary
algorithm, `inversion-microbench/`), moved no ecbench arm and was dropped.

## Reviewed and not changed

- `ic_boundary::invmod` (the inverse in every counted prime-curve group
  operation) already uses word Euclid; the suggested binary replacement is
  2.3–2.9× *slower* on this host (`inversion-microbench/results.txt`).
- Montgomery-batching the Subtract/MITM oracle inversions or the serial rho
  walks would change the `invs`/`field_ops` tallies, so it is not an
  overhead fix.
- Linear-algebra row scans (`IncrementalGauss::add_row`,
  `StructuredGauss::add_row`) and the Hasse-interval stepping in
  `PrimeCurve::prime_order` are candidates for later, measured separately.

## Reproduce

```sh
ecbench run --spec specs/generic-32.json --out OUT --allow-busy   # each binary
ecbench verify --dir OUT --replay-all --exit-code
python3 summarize.py && python3 validator.py
```

Host: Intel Xeon @ 2.10 GHz (family 6 model 207), 4 vCPU cloud container,
L0/below-L2 isolation. Binaries: main `412b60289` (sha256 `dc68e91b…`),
this change `03d7abd4e` (sha256 `65356a79…`). Engineering change to harness
overhead; not an index-calculus result.
