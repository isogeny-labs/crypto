# Validation of the 2026-10-26 visual handoff

Source parent before this report commit: `d70274b1452ae08c5dee92f23a74b9e6661a99d6`. No research algorithm, frozen input, audited receipt or worker code changed in this report patch. The following checks were run on the shared Apple M4 Pro checkout before pushing.

| Check | Observed result |
|:--|:--|
| `CARGO_TARGET_DIR=/private/tmp/n83-evidence-cargo-target cargo test --release --lib -- --test-threads=2`, sandboxed | 4,159 passed, 9 failed, 117 ignored; all nine failed tests were in local network/socket modules under restricted loopback access. This result is retained as a failed environment run. |
| Same exact release library command with local socket access | 4,168 passed, 0 failed, 117 ignored; 301.15 s test execution. |
| `CARGO_TARGET_DIR=/private/tmp/n83-evidence-cargo-target cargo test --release --example koblitz_n83_factor_base_export -- --test-threads=2` | 19 passed, 0 failed, 0 ignored. |
| `python3 -m pytest -q research/koblitz_n83_factor_base_sweep_20261008/test_*.py` | 63 passed in 10.54 s; compatibility validation of historical tests, no Python research implementation added. |
| `python3 -m pytest -q research/sat_factor_base_review_20260908/autolab/test_boundary_autolab.py` | 52 passed in 2.89 s; historical boundary compatibility validation. |
| `pandoc` with Typst, `pdfinfo`, `pdftoppm`, visual page inspection | All three current PDFs compile, open and render legibly; one page each. |
| `xmllint --noout` on three SVGs; `sips` on three PNGs | SVGs parse; PNGs are 1,688 pixels wide and tagged at 150 dpi. |
| Relative Markdown link existence, byte comparison of study and benchmark PDFs, `git diff --cached --check` | Passed after adding explicit PDF binary attributes. |

The sandboxed Rust test failure is not a research result and is not silently replaced by the later passing run. This report patch contains no new timing comparison; the test execution time is validation cost, not IC cost.
