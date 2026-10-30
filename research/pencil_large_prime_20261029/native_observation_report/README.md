# Native observation report

This presentation tool reads an existing hash-bound `pencil-price-cohort-output/v1` file and explicit caller metadata. It emits `report.json`, `cost-grid-exact.csv`, editable `coverage.svg`, editable `report.tex`, and a native LaTeX `report.pdf`. It executes no solver, analyzer, trial, or cloud action and does not establish input authenticity or scientific admission.

The supported design has three arms, six rounds (one warmup), and five measured pairs per workload. Workload count is dynamic; 1, 60 and 1,260 workloads therefore use their actual denominators. All 36 price cells must exist exactly once. Incomplete cells retain null ratios; complete cells require exact integer-string fractions. Timing-qualified ratios are outside this tool's scope. Missing observations and original outcome labels remain separate. Diagnostic partial admitted sums remain in the full retained JSON and are never promoted to complete ratios.

Run:

```sh
cargo +1.99.1 run --release --locked --manifest-path research/pencil_large_prime_20261029/native_observation_report/Cargo.toml -- request.json NEW_OUTPUT_DIRECTORY
```

The request schema is `pencil-native-observation-report-request/v1`. Required text fields are `title`, `run_id`, `scope`, and `source_commit` (explicitly labeled caller-supplied, not verified provenance). `analysis` is `{path,sha256}`. `sidecars` is an array of JSON file bindings, `notes` is an array of strings, and `resources` is null or a separate JSON file binding. The renderer checks every input digest. Resources are retained separately without conversion to algorithm cost units. Output directories must not exist. Native `pdflatex` with TikZ, longtable, array, geometry and hyperref is required; shell escape is disabled.

The graph presents admitted/expected pair counts, never statistical uncertainty or performance estimates. When fewer than two non-null ratio cells are available it presents a planned-arm and progress diagram instead. Exact ratios remain unrounded in CSV, JSON and PDF. The PDF is a presentation of retained values, not independent arithmetic replay.

Software validation uses `cargo +1.99.1 test --release --locked`, `cargo +1.99.1 fmt --check`, and `cargo +1.99.1 clippy --release --locked --all-targets -- -D warnings` with this manifest. The real-PDF fixture test requires LaTeX; optional `REPORT_FIXTURE_DIR` retains fixture packets in a new directory. Fixtures are software examples, not experimental outcomes. The initial fixture packet and initial passing log are preserved separately; the mixed fixture originally used failure outcome labels before those labels were corrected to verified for the admitted example. An initial development compile failed with E0283 at the outcome-count accumulator and was corrected with an explicit owned-string error type; that tool output was not saved as a raw log.

When at least two cells have non-null algorithm ratios, an additional `finite-cost-ratios.svg` shows both algorithm-only and calibration-inclusive ratios across all 36 explicitly labeled price cells. The common linear axis includes a ratio-one guide. Nulls are text labels without points. This graph uses floating point only for approximate visual coordinates and `~` labels; exact integer-string fractions in JSON/CSV remain authoritative and appear in SVG point titles. Cells share underlying observations and are not independent samples. If fewer than two cells have ratios, the arms/progress diagram and the complete null-preserving table remain the outputs, as required by the review-visualization rule. Fractions outside finite floating-point projection range reject graph rendering rather than silently plotting invalid coordinates.

The additive `validation/ratio-svg-v2/` packet records the separate graph extension. Earlier validation and fixture directories are unchanged. The SVG extension is exercised using explicitly synthetic software fixtures; it does not record a new empirical result.
