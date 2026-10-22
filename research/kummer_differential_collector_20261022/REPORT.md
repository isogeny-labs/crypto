# Fixed-difference Kummer collector: uncached cohort

Frozen output for `KUMMER-DIFFERENTIAL-COLLECTOR-20261022`, 2026-10-22 UTC.
The protocol and cached-inverse amendment were fixed before their respective
runs. This report records the completed **uncached** arm. The cached arm is
an explicit, separate pending experiment; no cached cost result is inferred
from its one-target pilot.

The signed quotient collector solved and source-verified all 1,260 public
frozen targets: 180 each at 16, 20, 24, 28, 32, 36 and 40 bits. It processed
230,430,039 quotient states, inserted 109,480,302 signed split relations,
recorded 739 Recover failures and made 1,080 walk restarts. The native
collector checked each transferred solution; the separate Rust analyzer
`src/bin/analyze.rs` independently recomputed every `kP=Q` on the original
`E(Fp²)` and joined the same-source rho row. It also matched all 21 cold
model setups, 1,260 cold target setups and all collector rows.

Full-cold candidate/rho cost spans **8.959687–55.935424** across all 36
whole-cohort price cells. The uncached candidate loses in every whole-cohort
cell and every individual target/cell comparison (0/45,360 wins). At
`S/M=0.5`, `I/M=10`, and `native=0.1 GAE/logical`, the summed candidate
cost is 15,631,567,864.99 GAE and summed same-source rho cost is
1,088,802,370.33 GAE, a ratio of **14.356662**. Online-only candidate/rho
ranges **8.492013–53.451983**. All seven size slices and all 36 price cells,
including every loss, are in `results/price-grid.csv`; individual paired
prices are in `results/target-price.csv`.

Cold candidate cost charges each target the full fresh model root search,
theta search and five two-torsion matrices, followed by that target's source
transfer, signed anchor, collector and verification. The same model setup is
charged to each of its 60 targets, without amortization. Rho costs are the
frozen same-source `rho_neg` counters from a separate archived process in
`research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst`,
priced with the same 36-cell grid. A same-process rho reference for the
validation gate is a separate pending measurement. `DATA_DICTIONARY.md`
defines fields and
counting boundaries; `VALIDATION.md` gives reproduction commands. The PDF
report `KUMMER_COLLECTOR.pdf` includes the graph and all 36 whole-cohort
cells. The native `replay-certificate.json` and `validator-report.json`
pin the source replay and headline recompute. Raw rows and figure files
are included here for outside reanalysis.
