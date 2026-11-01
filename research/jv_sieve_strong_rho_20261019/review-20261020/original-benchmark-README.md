# Joux–Vitse sieve on E(F_{p⁶}) against a strong rho

Visual copies for the [source study](../../research/jv_sieve_strong_rho_20261019/README.md) (weak curve class only, never prime order; the cover's Jacobian has genus 3). The study directory holds the frozen protocol, raw runs, `summary.json` and the analysis; the files here are byte-for-byte copies checked in [copies.json](copies.json).

- `run-report.pdf`: the report (every number from `summary.json`).
- `ratio_vs_l`: sieve over strong rho (negation map + batched inversion + distinguished points), paired by instance, harness and tuned prices, S and S⁺.
- `exponent`: total F_p multiplications against ℓ for both; fitted slopes.
- `rho_factorial`: what the negation map and batched inversion each buy.
- `dense_vs_sparse`: sparse FGLM in the sieve's descent, paired.
