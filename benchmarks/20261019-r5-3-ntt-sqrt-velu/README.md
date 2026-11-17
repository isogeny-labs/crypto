# R5.3: three-prime NTT for √élu

Browse copy of the counted-work study. Full source, raw data, commands, and limitations: [`research/.../r5_3_ntt_sqrt_velu/RESULTS.md`](../../research/isogeny_fibers_20261010/round5/r5_3_ntt_sqrt_velu/RESULTS.md).

- NTT √élu exponent: 0.798 [0.775, 0.820] for 2^16..2^24; 0.745 [0.686, 0.804] for 2^20..2^24. The 0.5–0.6 hypothesis is not supported in this range.
- At ℓ=2^24: 2.21e7 vs 2.45e7 counted Meq (1.11×); the stated unit counts one NTT word product as one M. Contended wall timing puts the crossover at about 3072–4096 coefficients, beyond the count crossover of 512.
- Generic kernel-finding exponent was 2.582 (NTT) vs 2.587 (Karatsuba), effectively unchanged. Rational-kernel crossover remains beyond ℓ_max=2√p in the listed extrapolations.
- Synthetic toy instances only; single contended host, generic runs capped at ℓ=863; crossover values extrapolate many octaves. No sub-rho or deployed-curve claim.

Figures are included as SVG and PNG: `fig1` counted cost, `fig2` local exponent, `fig3` generic kernel finding, `fig4` threshold, `fig5` crossover.
