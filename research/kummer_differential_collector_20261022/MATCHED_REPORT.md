# Same-process Kummer collector and rho cohort

Study `KUMMER-DIFFERENTIAL-COLLECTOR-20261022`, completed 2026-10-22 UTC.
The matched native `paired_uncached` executable ran the frozen uncached
candidate over all 1,260 public targets, then ran negation rho over the same
1,260 workload IDs in that process. The candidate path checked each recovered
scalar against the planted value and the transferred Jacobian equation. The
paired rho path source-checked each workload's planted scalar. The independent
Rust analyzer directly verified every recovered candidate scalar as `kP=Q`
on the original elliptic curve over `Fp²`, compared all 1,260 candidate rows exactly to
the previously frozen uncached rows, and recomputed all prices from fresh
operation counts. The 1,260 fresh rho M/S/I arithmetic records also matched
the archived same-source M/S/I counts.

The candidate processed 230,430,039 quotient states and inserted 109,480,302
signed split relations. It recorded 739 failed intermediate Recover calls and
1,080 walk restarts. All 1,260 final solves were verified. These counts and the complete
cold-cost price grid equal the separate-process archived comparison; the
newly calculated 288 price rows were compared exactly to its uncached rows.
The fresh paired measurement
establishes the same-process reference needed for the repository validation
gate.

Across all 36 whole-cohort price cells, uncached full-cold candidate/rho
ratios range **8.959687–55.935424**. The candidate loses in all 36 cells and
all 45,360 target/cell comparisons. Online-only ratios range
**8.492013–53.451983**. At `S/M=0.5`, `I/M=10`, and
`native=0.1 GAE/logical`, candidate cold cost is 15,631,567,864.99 GAE,
rho cost is 1,088,802,370.33 GAE, and their ratio is **14.356662**.
The exact 288 slice-and-price rows and 45,360 target-and-price rows are in
`results/paired/analysis/{price-grid,target-price}.csv`, including every
loss. Native charge 0, 0.1 and 1 GAE retains the same ranking; the exact
band is in `VALIDATION.json`.

The candidate cold boundary charges each target a fresh model root search,
theta search and five two-torsion matrices, then its source transfer, signed
anchor, quotient collector and verification. The full model setup is charged
60 times per model, once per target, without amortization. The rho control
uses its measured native source addition unit and fresh source verification.
`DATA_DICTIONARY.md` specifies operation categories and `VALIDATION.md`
gives reproduction commands. `results/paired/{uncached,rho}/*.jsonl` holds
the raw solves; `results/paired/analysis/` holds the native replay certificate,
validator report, summary and prices. `VALIDATION.json` pins the two audit
artifacts and records the same-process rho reference. `KUMMER_PAIRED.pdf`
contains the graph and all 36 full-cohort price cells, with standalone
`collector-cost-paired.svg` and `.png` for review.
