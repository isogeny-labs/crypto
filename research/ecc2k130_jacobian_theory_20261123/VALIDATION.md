# Validation receipt (producer, unsigned)

- `g++ -std=c++17 -Wall -Wextra -Werror -O2 check.cpp`: exit0; output equals check.txt.
- UndefinedBehaviorSanitizer (`-fsanitize=undefined -fno-sanitize-recover=all -O1`): exit0; output equals check.txt. The final added ord131(2) assertion was replayed in the optimized build.
- Arithmetic witness includes a full-factorization Lucas primality proof, not a probable-prime assertion. This is a custom native replay, not an IDC1 certificate, signed execution proof or Lean theorem.
- `pcc standing`: f0df24, 6e3ede and 7b7fc5 are all unverified.
- `pcc ledger verify --base origin/main`: 1830 entries, chain ok, after materializing tracked schema-rejections and claim files omitted by sparse checkout. Historical missing-type claims remain rejected/ineligible under the existing manifest; no historical record was edited.
- `pcc lint-prose --stdin` on PR body: zero findings.
- `git diff --check`: exit0 before commit.
- Typst0.15.1 builds report.pdf; pdfinfo reports four pages; pdftotext extracts all sections. All four page renders were visually inspected.
- `lean --version` and `cargo test --release --lib`: exit127, commands absent; attempts retained. Arithmetic.lean is not compiled. Full geometric Lean definitions/theorems are also missing.
- No production Rust or benchmark source changes. No experiment timing or security-cost data added.

The tracker-only PR head had two unrelated native workflow failures:

- https://github.com/isogeny-labs/crypto/actions/runs/45259183082 — round123_quad_ab.py line23, `assert frozen.count(old)==1`.
- https://github.com/isogeny-labs/crypto/actions/runs/45259182411 — round124_native_trace_ab.py line12, source-string `reciprocal_64`/`let quotient` assertion.

That head changed only docs/ACTIVE.md. The cited research source and Rust
production modules are unchanged by this task. These are not successful
checks; their unrelated failures are recorded, not repaired in this lane.

The PR remains draft because the requested curve and evaluable map are still
missing, and the mathematical deductions lack independent review/formalization.

Report prose lint returns four line-local findings on “trial-proved”, the
primality argument, an imperative transfer requirement (“prove P_C=P_A”),
and the explicit phrase “not Lean verified”. The report's opening marks
all deductions unverified and individual conclusions repeat that status.
The pinned report is retained unchanged; PR-body lint has zero findings.
These report-linter findings are disclosed, not counted as a passing lint.
