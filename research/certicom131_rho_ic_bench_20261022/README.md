# Certicom-131 benchmark: rho and index calculus (2026-10-22)

Baseline measurement of this repository's code for the three 131-bit Certicom
targets (ECC2K-130, ECC2-131, ECCp-131) on one x86 CPU core (Xeon 2.1 GHz KVM
guest, no GPU). Read [`report.pdf`](report.pdf) (source `report.typ`);
every value is in [`results.json`](results.json), raw records in [`raw/`](raw/).

- Rho: exact expected work from the group orders (2^60.81, 2^64.83, 2^64.92),
  measured step rates (ECC2K-130 client 13.2 M/s; ECC2-131 pclmul batched
  walk 8.50 M/s; ECCp-131 32-bit Montgomery batched walk 1.23 M/s), and the
  derived core-years. Batching one inversion over 128 walks gave 20x (ECC2-131)
  and 24x (ECCp-131).
- Index calculus: `ic boundary --seed 131` ladders (24 instances, all verified)
  against the matched rho, and 25 exact-curve ECCp-131 relation probes (0
  natural relations, rank 0).
- Not shown: any GPU rate, any solve of a published target, any IC result at
  131 bits. Classification: baseline measurement, no improvement claimed.
  Not an admitted `benchmarks/README.md` or leaderboard row (validation gate
  not met: no replay certificate, no validator report, no section 5a band).

Reproduce: build `ecdlp-nist`, `ic` (release), `make -C ecc2k130 ecc2k130-cpu`,
`g++ -O3 -std=c++17 -march=native -mpclmul -o bench131 bench131.cpp`, then
`./run_all.sh` into an empty `raw/`, and `python3 analyze.py` (needs matplotlib).
