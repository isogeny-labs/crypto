# PKM factor bases from smooth maps (`pkm-smooth-tower`), 2026-11-14

Report: [REPORT.pdf](REPORT.pdf) (source [REPORT.typ](REPORT.typ)). Diagram:
[figures/pipeline.svg](figures/pipeline.svg); graphs
[figures/cm_orders.svg](figures/cm_orders.svg) and
[figures/method1_trials.svg](figures/method1_trials.svg), drawn from the
recorded CLI outputs in [data/](data/) by
[figures/make_figures.py](figures/make_figures.py).

Code: `src/cryptanalysis/pkm_smooth_tower.rs`, `src/bin/pkm_smooth_tower.rs`
(PR #4091, commit `a894bbdaa9`). Construction and checking tooling only; no
benchmark, nothing priced against index calculus or rho [pcc:unverified].

Rebuild: `python3 figures/make_figures.py` (matplotlib), then
`python -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf')"`.
