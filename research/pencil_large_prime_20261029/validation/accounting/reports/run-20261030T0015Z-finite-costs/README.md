# Finite counted-cost gate observation packet

This write-once report displays the covered p739 gate's 36 exact price cells, all 60 workloads and 300 measured pairs. `observations.json` and `cost-grid-exact.csv` are byte-identical copies of the covered-analysis receipt and exact CSV. The Rust renderer reads their rational values and converts to f64 only for rounded display/plot coordinates; it does not recompute experimental counts or execute workloads. Both curves are retained and nearly overlap in parts of the plot.

Reproduce from this directory:

```
CARGO_TARGET_DIR=/tmp/pencil-finite-observation-render-target cargo run --offline --release --locked
pdflatex -interaction=nonstopmode -halt-on-error report.tex
```

The renderer source, editable SVG, TeX narrative and generated report source are included. Rust/LaTeX only; no Python analysis or renderer. The first rendering invocation used the wrong cwd; its failure is recorded in render-attempts.txt. The first PDF build required unavailable pgfplots; build-missing-pgfplots.txt retains the failure. The completed renderer uses installed standard LaTeX picture primitives and color instead. Three rendered PDF pages were visually inspected for clipping, table completeness and observation labels; no new scientific observations were generated.

Resource records preserve controller/job failures independently of the complete record set. Qualified wall ratios remain null. Previous reports and initial null-coverage results are unchanged. Exact operation ratios are specific to the finite accounting model and this 60-target gate, not the full 1260-workload campaign.
