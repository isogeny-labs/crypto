# Certicom-131 rho step-rate parity on one CPU core (2026-10-23)

Follow-up to [the 2026-10-22 baseline](../certicom131_rho_ic_bench_20261022/README.md): the same exact
ECCp-131 and ECC2-131 groups, now with vector arithmetic, measured against the ECC2K-130 client in one
isolated, interleaved session (7 rounds, A/A control, 59 records, none contended). Read
[`report.pdf`](report.pdf) (source `report.typ`); every value is in [`results.json`](results.json).

| arm (one core, same session) | median M steps/s | ratio to ECC2K-130 client |
|---|---:|---:|
| ECC2K-130 client | 10.3 | 1.00 (A/A drift 8%) |
| ECCp-131 AVX-512 x8 (new) | 15.06 | 1.46 (11.8x the old 32-bit arm) |
| ECC2-131 bitsliced on the client's field code (new) | 11.46 | 1.11 (parity within the drift) |
| ECCp-131 old 32-bit / ECC2-131 old clmul (baseline arms) | 1.28 / 9.02 | 0.12 / 0.87 |

Negative results kept: a scalar 3x64 Montgomery ECCp-131 arm (4.89 M/s) and a 3-product carry-less
ECC2-131 arm (6.92 M/s, slower than the old arm). No index-calculus improvement this round; the report
records the phase split of the remaining gap. Classification: engineering (rho step rate). Not an admitted
`benchmarks/README.md` or leaderboard row (validation gate not met). All gates passed (field oracle, step
oracle against the header arithmetic, on-curve, slot-count independence, zero exceptional events); the
first attempt, with a degenerate start lane, is retained in `raw_attempt1_degenerate_start/`.

Reproduce: `g++ -O3 -std=c++17 -march=native -mpclmul -o walk131 walk131.cpp`,
`g++ -O3 -std=c++17 -march=native -o pv131 pv131.cpp`,
`g++ -O3 -std=c++17 -march=native -mpclmul -I../../ecc2k130/include -o bs131 bs131.cpp`,
build `ecc2k130-cpu` and the old `bench131`, then `./run_parity.sh` into an empty `raw/` and
`python3 analyze_parity.py`.
