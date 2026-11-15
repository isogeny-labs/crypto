# ic_gw_nrm_lz_20261115: normalised points (NRM) and scalar-Lanczos dot reuse (LZ_REUSE) on BLR1

All numbers `[pcc:unverified]`, L0 local run (EKS budget hold), counted cost only. Protocol frozen in commit
`851b8da629` before any unit ran. Protocol: `PROTOCOL.md`; units `units.tsv` (150); runner `run-local.sh`;
analysis `analyze.py` -> `analysis.json`, `tables.md`; `plot.py` -> `nrmlz.svg/png`; `gen-report.py` -> `REPORT.typ` -> `REPORT.pdf`;
raw units (gzip): `runs-nrmlz-20261115.tar.gz` (binary hashes in `binary.sha256` inside).

Binaries: REF = BLR1, sha256 `e236855d7fef58d1c6618b25c399e462c7099bcb1f634047a7258e5f5d074035`;
CAND sha256 `a7e4ad7a41e5a8a61466791af069af75914c2ed883c61b0d0f18d6ef67d23577` (rustc `--edition 2021 -O v3.rs --cfg combo`).

Result: gates G0-G3 pass. By the frozen rule BOTH is an improvement for DL, DL1 and D (all 30 cells, w_add 0, 0.5, 1,
also at every w_h). Full tables: `tables.md`. Not validated (no replay certificate / validator / new matched rho): no
leaderboard or `benchmarks/README.md` row.
