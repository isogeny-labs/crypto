# Fixed-difference Kummer collector: uncached and cached cohorts

Frozen output for `KUMMER-DIFFERENTIAL-COLLECTOR-20261022`, 2026-10-22 UTC.
The protocol and cached-inverse amendment were fixed before their respective
runs. Both arms completed the same 1,260-target cohort. The cached arm reused
four fixed theta inverses in each of the two independent xADD callers, with
eight initial inversions charged per target.

The signed quotient collector solved and source-verified all 1,260 public
frozen targets: 180 each at 16, 20, 24, 28, 32, 36 and 40 bits. It processed
230,430,039 quotient states, inserted 109,480,302 signed split relations,
recorded 739 Recover failures and made 1,080 walk restarts **in each arm**.
The target-by-target quotient trajectory and graph operation counts matched
between arms; their inversion counts differ. The native
collector checked each transferred solution; the separate Rust analyzer
`src/bin/analyze.rs` independently recomputed every `kP=Q` on the original
`E(Fp²)` and joined the same-source rho row. It also matched all 21 cold
model setups, 1,260 cold target setups and all collector rows.

Full-cold candidate/rho cost spans **8.959687–55.935424** for uncached and
**8.617640–54.093644** for cached across the same 36 whole-cohort price
cells. Both arms lose in every whole-cohort cell and every target/cell
comparison (0/45,360 wins per arm). At
`S/M=0.5`, `I/M=10`, and `native=0.1 GAE/logical`, the summed candidate
cost is 15,631,567,864.99 GAE uncached and 14,095,269,577.37 GAE cached,
against summed same-source rho cost of 1,088,802,370.33 GAE. The respective
ratios are **14.356662** and **12.945664**. Online-only candidate/rho
ranges are **8.492013–53.451983** uncached and **8.149964–51.610204** cached.
All seven size slices and all 36 price cells for both arms,
including every loss, are in `results/price-grid.csv`; individual paired
prices are in `results/target-price.csv`.

Cold candidate cost charges each target the full fresh model root search,
theta search and five two-torsion matrices, followed by that target's source
transfer, signed anchor, collector and verification. The same model setup is
charged to each of its 60 targets, without amortization. Rho costs are the
frozen same-source `rho_neg` counters from a separate archived process in
`research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst`,
priced with the same 36-cell grid. A same-process rho reference is separately
recorded under `results/paired/`; it is not substituted into the archived
comparison here. `DATA_DICTIONARY.md`
defines fields and
counting boundaries; `VALIDATION.md` gives reproduction commands. The PDF
report `KUMMER_COLLECTOR.pdf` includes the graph and all 36 whole-cohort
cells. The native `replay-certificate.json` and `validator-report.json`
pin the source replay and headline recompute. Raw rows and figure files
are included here for outside reanalysis.
