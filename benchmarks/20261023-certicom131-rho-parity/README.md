# Certicom-131 rho step-rate parity: diagnostic copies

Browseable copies of the [study report](../../research/certicom131_rho_parity_20261023/report.pdf)
(`report.pdf`, byte-identical to the study's) and its two graphs as SVG and 150 dpi PNG, rendered from the
same plotting source (`research/certicom131_rho_parity_20261023/analyze_parity.py`). The study directory is
the source of truth.

Headline (one pinned core, 2.8 GHz Xeon, same session, A/A drift 8%): ECC2K-130 client 10.3 M iterations/s;
ECCp-131 AVX-512 walk 15.06 M steps/s; ECC2-131 bitsliced walk 11.46 M steps/s.

**This is not an admitted `benchmarks/README.md` row.** The validation gate requires a replay certificate
on an independent host, a validator report and a matched rho reference with the section 5a band; none exists,
and the numbers are step rates, not solves. See the study report for scope and limits.
