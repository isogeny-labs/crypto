FULLY SYNTHETIC VOLCANO / ECDLP EXPERIMENT
Invented data. No elliptic curves constructed. No discrete logarithms solved.

Open run-final/reports/explorer.html for the self-contained interactive report.
Read run-final/reports/experiment_report.pdf for the printable report.

Requested design
  - Nominal field sizes: 2^30, 2^40, 2^50.
  - Family labels: prime field, ordinary binary, Koblitz origin.
  - Five strata, depth four, 161 abstract vertices per component.
  - Three horizontal degree overlays on all levels (11, 23, 29).
  - Ten symbolic targets at EVERY vertex; min, mean, max retained.
  - Rho, two-summand IC scan, three-summand IC pair-table models.
  - Null, planted floor advantage, planted floor penalty.
  - Cold accounting and setup shared across exactly ten targets.
  - Full-graph and balanced profiling use separate pilot targets and charge
    graph construction, candidate preprocessing, and all pilot work.
  - Accounting weights 0, 0.1 and 1 for invented native operation counts.

Scope
  This is a complete comparison of the specified abstract component and cost
  models. It is not the all-degree graph of all curves over a field, a real
  isogeny census, a real IC/rho benchmark, or evidence of an ECDLP speedup.
  Family and field sizes are model labels. The nominal subgroup prime is a
  parameter; it is not certified to lie on any elliptic curve.

  Degree-3 ordinary volcanoes have no horizontal degree-3 edges below the
  surface. Separate degree labels provide the requested horizontal overlays.
  Their cyclic shifts are illustrative, not actual ideal/isogeny actions.

  The Koblitz-origin model applies the assumed Frobenius discount only at
  level zero. Descendants use ordinary binary costs. A deep Koblitz-origin
  volcano at exactly degrees 30, 40 and 50 has not been constructed.

Reproduce (C++17 compiler; Python used only for plots/documents)
  bash run.sh run-new

  Python rendering dependencies: numpy, matplotlib, reportlab.
  No network access, cryptographic library, Sage or PARI is required.
  The simulator refuses to overwrite any existing output directory.
  Default seed: 20261018. Exactly ten targets and three pilot targets.

Native simulator and independent CSV audit only
  mkdir -p build
  g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic src/simulate.cpp -o build/simulate
  g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic src/audit.cpp -o build/audit
  build/simulate --self-test
  build/simulate --seed 20261018 --out run-new
  build/audit run-new

Change the model seed or depth
  build/simulate --seed 99 --depth 4 --out run-seed99
  Depths 1 through 5 are accepted. The delivered run uses depth 4.
  Rendered fixed-depth report prose describes the delivered depth-4 design;
  adapt that prose before distributing a report for another depth.

Data files (all cost values synthetic)
  trials.csv.gz          Full stage counters: 130,410 rows.
  curve_summary.csv.gz   All 78,246 per-vertex ten-target summaries.
  strategy_trials.csv.gz Charged strategy comparisons for all ten targets.
  strategy_summary.csv   Min/mean/max/median/SD and rho ratios by strategy.
  level_summary.csv      Per-target level means, summarized across ten targets.
  patterns.csv           Descriptive floor/surface model comparisons.
  pilot_selection.csv    Candidate choices, pilot charges, no holdout use.
  diagnostics.csv        Native-computed stage means and memory assumptions.
  vertices.csv           Abstract positions; coordinates explicitly absent.
  edges.csv              All 643 labeled multiedges, including loops.
  topology_ablation.csv  Path distances with 0/1/2/3 horizontal overlays.
  config.json            Frozen assumptions and accounting equations.
  validation.json        Simulator structural/accounting checks.
  audit.txt              Independent audit output.
  replay.txt             Byte-identical core replay check.
  SHA256SUMS              Delivered file integrity checks.

The archive compresses the three large CSV files. To audit the delivered run:
  gzip -dk run-final/trials.csv.gz
  gzip -dk run-final/curve_summary.csv.gz
  gzip -dk run-final/strategy_trials.csv.gz
  build/audit run-final

Statistical reading
  Per-curve statistics summarize ten targets. Per-level statistics first
  average vertices within each target, then summarize ten target-level means.
  Minima across 108 floor candidates have a selection advantage over one
  surface candidate. Pilot selection is separate from those ten test labels.
  No confidence intervals, significance tests or empirical exponent claims.
  The three size labels reveal the assumed formulas, not real-curve patterns.

Accounting
  Unit = invented group count + weight * invented native count, uncalibrated.
  No seconds, actual throughput or hardware speed claims are reported.
  Full rank, target decomposition and recovery are assumptions, never flags
  claiming cryptographic correctness. Memory is modeled, not allocated.
  Batch results share factor-base-log preprocessing; online target work and
  costed verification are retained. Each actual repository solver would need
  its own measured accounting and reusable-preprocessing interface.

Repository context
  Both aburan28/crypto and aburan28/cryptanalysis were inspected read-only.
  The native simulation is new standalone code informed by their interfaces
  and reporting conventions. Core repository cryptographic algorithms were
  not executed. No remote files, benchmark ledgers, or PRs were changed.
  See reference/sources.json and reference/INTEGRATION.txt.

  tools/volcano_diagram.py is retained from aburan28/crypto, with a small
  adaptation for synthetic terminology. Original copyright remains with
  its authors. Source blob and local changes are documented in provenance.

Validation
  986 structural/accounting assertions passed.
  Independent audit checked all raw-to-summary statistics and target counts.
  All core output files replayed byte-for-byte on the recorded toolchain.
  Cryptographic correctness is not applicable to invented observations.
