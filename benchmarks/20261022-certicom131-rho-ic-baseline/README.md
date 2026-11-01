# Certicom-131 rho / index-calculus baseline: diagnostic copies

Browseable copies of the [study report](../../research/certicom131_rho_ic_bench_20261022/report.pdf)
(`report.pdf`, byte-identical to the study's) and its three graphs as SVG and
150 dpi PNG, rendered from the same plotting source
(`research/certicom131_rho_ic_bench_20261022/analyze.py`). The study directory
is the source of truth.

Headline measurements (one pinned core, Xeon 2.1 GHz KVM guest): ECC2K-130
client 13.2 M iterations/s; ECC2-131 pclmul batched walk 8.50 M steps/s;
ECCp-131 32-bit Montgomery batched walk 1.23 M steps/s; best index-calculus
variant 3.4x to 75x the matched rho's S on instances up to 2^39.

**This is not an admitted `benchmarks/README.md` row.** The validation gate
requires a replay certificate, an independent validator report and a matched rho
reference with the section 5a band; none exists for this baseline, and it
changes no code under test. See the study report for scope and limits.
