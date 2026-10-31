# PKM Kummer tower at P-224's 2-adic shape, priced against matched rho (2026-10-22)

Copies of the visual record of
[`research/pkm_p224_kummer_shape_20261022/`](../../research/pkm_p224_kummer_shape_20261022/README.md),
which is the source of truth: `report.pdf`, and each graph as SVG with a 150 dpi PNG render
(`d_vs_n`, `madds_vs_n`, `s_ratio_by_size`), all produced by
`research/pkm_p224_kummer_shape_20261022/render.sh` from the same SVG source.

Headline (as the report states it): F4 solving degree equal to the recorded value in 89 of 89
valued cells at eight Solinas primes and at P-224; whole-method index calculus on Solinas
curves 18.4 to 75.9 times matched rho at `w = 0` (`S`, ecbench, 20 to 28 bits); derived F4-arm
lower bound 1.9 x 10^9 times rho at 20 bits. No speedup is claimed.

**No row in `benchmarks/README.md`.** The validation gate
([`docs/VALIDATION_GATE.md`](../../docs/VALIDATION_GATE.md)) admits a row only with an
independent-host replay certificate and a validator report, which this session did not have
(one host, replays by `ecbench verify` on the same host class). The result is a null/negative
one and stays in its research directory.
