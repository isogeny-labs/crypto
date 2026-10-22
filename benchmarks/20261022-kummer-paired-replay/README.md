# Paired Kummer collector validation replay

This directory copies the visual record for the [matched study](../../research/kummer_differential_collector_20261022/MATCHED_REPORT.md) and the [fresh isolated replay](../../research/kummer_differential_collector_20261022/reports/run-20261022T1354Z/RUN.md). The study remains the source of truth for code, inputs, raw JSONL, the 36-cell GAE price model, and its independent validator.

The model reports a full-cold uncached collector/rho ratio of 8.959687–55.935424 across 36 pooled price cells; the collector loses every cell. At `S/M=0.5`, `I/M=10`, and 0.1 GAE per native logical operation, its ratio is 14.356662. The isolated process reproduced all 14 raw candidate/rho files and five analysis files byte for byte. Its 24:50.16 wall duration was observed on an L0 shared host and is not a comparative wall benchmark.

The full-cold candidate loses all 45,360 target/price rows. The online-only
column has 94 individual target-price wins, although its pooled cost loses
all 36 cells. The [run report](../../research/kummer_differential_collector_20261022/reports/run-20261022T1354Z/RUN.md) and its CSV check retain that distinction.

- [Results PDF](results-report.pdf) and [run PDF](run-report.pdf)
- [SVG graph](collector-cost-paired.svg) and [150 dpi PNG](collector-cost-paired.png)
- [Validation gate](VALIDATION.json), [copy manifest](COPY_MANIFEST.json), and [raw data link](results)

The canonical IC leaderboard and progress graph already contain the matched-study result. The replay produced no changed price values, so those rendered graphs are unchanged.
