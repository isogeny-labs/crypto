# Block recurrence audit and residue-section evaluator

T-1904 implements two bounded algebraic constructions: a denominator-cleared block recurrence checker and a coefficient-scan-free residue-section evaluator. See DERIVATION.md for exact assumptions, costs and proof scope. All research arithmetic is Rust; Python is used only for metadata/report generation.

`main.rs` supplies quotient arithmetic and triangular block construction; `checks.rs` is the dense recurrence oracle. `power.rs` computes (X-1)^h modulo X^m-v and checks all returned modes. It imports the existing polynomial engine and meter; it does not modify or copy the recovery adapter. This is a separate evaluator awaiting integration.

Replay from repository root, with a fresh output path and stop-on-error:

```sh
set -e
timeout 90 rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info research/primefield-companion-followup-20261102/feature-recurrence-compression-audit/main.rs -o /tmp/section-audit-fresh
timeout 60 /tmp/section-audit-fresh
timeout 30 lean research/primefield-companion-followup-20261102/feature-recurrence-compression-audit/Algebra.lean
```

Validation01 is the successful block-only audit; its source snapshots are retained. Validation02 failed compilation and then accidentally ran the old binary; that output is explicitly rejected for the new checks. Validation03 uses a fresh binary and stop-on-error and validates both constructions. No cloud, timing comparison or frozen-cohort change. Historical full-Cargo missing-fixture limitation remains; affected standalone checks are used.
