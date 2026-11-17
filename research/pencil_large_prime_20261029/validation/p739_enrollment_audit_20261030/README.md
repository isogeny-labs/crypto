# Exact retained-plan enrollment audit

This standalone Rust checker reads existing plans only. It never generates a
plan, target or solver result. Inputs are the retained p739 gate, cohort-16 and
fresh-session plan. All algorithm seeds are exact u64 values; the tests include
u64::MAX and adjacent integers above2^53 to detect lossy conversion.

Observed mapping:60 complete workload objects,3 complete resolved arm objects,
and1080 execution identities map bijectively into cohort16. Missing, duplicate,
extra-subset and seed-mismatch counts are zero.900 sequence positions differ;
both original seq values and IDs are retained in machine mapping rows. The fresh
plan is fully equal to the gate plan. This establishes retained membership only,
not general admission or authority to launch remaining1200 targets.

Sources checked: actual ecbench.plan/v1 retained files and existing study native
analysis enrollment contracts; research-visuals skill for neutral evidence-linked
presentation. No new mathematical or novelty claim is made.

Reproduce with cargo +1.99.1 run --release --locked --offline -- GATE COHORT FRESH
NEW_OUTPUT_DIRECTORY from this crate. Tests use cargo +1.99.1 test --release
--locked --offline. PDF rendering requires native pdflatex with TikZ. Source,
Cargo.lock, raw test/clippy logs and emitted report.tex/SVG/JSON are retained.
The PDF was rendered and visually inspected: one page, no clipped labels/hashes.
Source-checking and report generation do not confer experiment qualification.
