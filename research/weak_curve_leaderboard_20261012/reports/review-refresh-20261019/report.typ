#set page(paper: "a4", margin: 18mm)
#set text(font: "Libertinus Serif", size: 10pt)
#set par(justify: true)
#set heading(numbering: none)
#let current = json("current-audit.json")
#let previous = json("previous-audit.json")
#let provenance = json("provenance.json")
= Conditional GHS cost-model refresh
*2026-10-19 · crypto PR #2154 · conditional formula model*

The genus-128 audit now reads the transfer registry's measured Jacobian-operation table and its stated extrapolation exponent. The previous audit read a retired quadratic-price field. This report lays out recorded model outputs; it adds no timed experiment.

#table(
  columns: (1.2fr, 1fr, 1fr, 1fr), inset: 6pt,
  table.header([Source curve], [Previous total], [Current total], [Formula rho]),
  [c2pnb176w1], [71.336033], [74.834885], [94.973],
  [c2pnb272w1], [71.773568], [74.879405], [143.970],
)
All prices are *log2 modeled bit-operation costs*. Both exact source curves select bound 15 from 84 bounds, with 48,921,052 nominal half-split places. The smooth-polynomial count, nominal base count, and modeled query count are unchanged.

#figure(image("cost-sweep.svg", width: 100%), caption: [Recorded previous/current sweeps and formula rho references. One deterministic model sweep per source curve; no empirical confidence interval.])

== Assumptions and witness gates
The operation table measures Cantor operations only through genus 32. Genus 128 is extrapolated with exponent 2.347. The audit also assumes half the irreducibles give usable places and polynomial smoothness predicts divisor smoothness. The smooth cover, selected-subgroup map, actual usable places, measured relation yield, useful rank, final linear algebra and target descent remain separate witness gates. The formula gap to rho does not establish a measured complete attack cost.

== Validation and preservation
Six standalone release tests passed. The audit and snapshot replay, attack-map and decomposition validators passed; 98 weak-curve and 991 P-192 manifest entries matched. Repository-wide CI remains a separate merge gate. Dated original reports and graphs remain historical evidence. Current study links and website JSON mirrors identify the refreshed prices.

#pagebreak()
= Exact identities and provenance
The receipt identities below retain the representation and field-polynomial fingerprints, coefficients, generator encoding and subgroup fingerprint; unspecified fields remain unspecified. The current receipt binds its registry inputs by SHA-256.

#for row in current.curves [
  == #row.name
  #text(size: 8pt, raw(row.curve_id.replace(":", ":\n"), block: true))
  *Subgroup order:* #text(size: 8pt, raw(row.subgroup_order))

  *Relative degree:* #row.relative_degree. *Model genus:* #current.genus.

]
== Frozen comparison
*Reviewed PR head:* #text(size: 8pt, raw(provenance.reviewed_head))

*Comparison base:* #text(size: 8pt, raw(provenance.comparison_base))

*Integrated review commit:* #text(size: 8pt, raw(provenance.integrated_commit))

#for (name, digest) in provenance.data_sha256 [
  *#name:* #text(size: 8pt, raw(digest))

]
== Reproduction and sources
The two adjacent audit JSON files are the graph's sole numerical inputs. #raw("build_report.py") verifies their frozen hashes and lays out recorded values. #raw("provenance.json") records the comparison and validation scope. Run from the study directory:

```sh
cargo test --release --locked
cargo run --release --locked -- check snapshot.json
cargo run --release --locked --bin ghs_genus128_cost_audit -- --check
python3 reports/review-refresh-20261019/build_report.py
typst compile reports/review-refresh-20261019/report.typ \
  reports/review-refresh-20261019/report.pdf
```
The study's #raw("README.md") and #raw("REPORT.md") link this refresh. Historical reports under #raw("reports/run-20261015T0854Z") and #raw("reports/run-20261015T0929Z") retain their original prices. The website's #raw("ghs_genus128_cost_audit.json"), #raw("decomposition_properties.json") and #raw("snapshot.json") use regenerated current data. No shared performance graph is repriced: this change updates conditional formulas, with no measured runtime claim.
