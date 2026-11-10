# Native export/plot metadata adapter

The frozen mathematical analyzer is ../analyze.cpp, pinned before data. This
Rust utility was written after collection to format its accepted observations,
per-run provenance, plot counts and the canonical two-row export. It does not
invoke or modify the census/analyzer, fit a model, or create new physical rows.

`export ROOT UTC OUTPUT` requires complete successful outcomes and exact A/A,
then writes receipts, count/SVG graphic and export. `check ROOT EXPORT` validates
protocol freeze/arm identities, frozen analysis pin, physical row semantics and
all referenced artifact hashes. Validate the canonical schemas separately with
the existing native schema tool. All producer checks remain unsigned.

Build with cargo --release --offline; Cargo.lock is study-local. Earlier frozen
common-scalar contract adapter is not reused for this study's export semantics.
No canonical Python checker was copied or used for research execution.
