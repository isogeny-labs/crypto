# Replication V2: Joux–Vitse sieve against a strong rho (decisions withheld)

Visual copies for the [source study](../../research/jv_sieve_strong_rho_v2_20261020/README.md) (weak curve class only, never prime order; the cover's Jacobian has genus 3). Thresholds were fixed in a protocol committed before any run; the pre-registered acceptance rule A4 (rho A/A at p = 1009) failed, so every decision is withheld and the figures are descriptive. The files here are byte-for-byte copies checked in [copies.json](copies.json).

- `run-report.pdf`: the report (every number from `summary.json`).
- `ratio_vs_l`: sieve over strong rho, paired by seed, four prices.
- `exponent`: total F_p multiplications against ℓ for both.
- `rho_factorial`: negation map and batched inversion.
- `dense_vs_sparse`: sparse FGLM in the sieve's descent, paired.
