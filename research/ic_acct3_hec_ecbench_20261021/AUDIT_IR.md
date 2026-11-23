# Instruction audit (`ecbench audit-ir`) of the hyperelliptic ecbench arms

2026-11-07. Machine-readable summary: [`AUDIT_IR.json`](AUDIT_IR.json)
(`ic-acct.audit-ir/v1`). Rows: [`audit/audit-rows.json`](audit/audit-rows.json).
Raw `ecbench.audit_ir/v1` lines: [`audit/audit-ir.jsonl`](audit/audit-ir.jsonl).
Figure: [`audit/figures/ir_per_gate.svg`](audit/figures/ir_per_gate.svg) (PNG beside it,
drawn by `audit/figures/plots.py` from the rows only).

This is the audit `docs/units/BITOPS.md` ("What the unit leaves out, and how
that is caught") requires before a count is admitted as a `bitop/v1` bound.
README.md's "Limits" recorded that it had not been run. It is a diagnostic
for missing counters. It is not proof that all work is counted, and it does
not validate the gate formulas as circuit costs. It is **not** a
`VALIDATION.json`, and it admits no leaderboard or `benchmarks/` row.

## What was run

- **Code.** Commit `8c37d9a6e7` on `claude/ic-acct-3` (PR #2654), the commit
  that produced `sessions/g2-p41`. The branch head `67c7e7e060` does not
  compile: `src/cryptanalysis/ecbench/workload.rs` has an unclosed delimiter
  left by a union merge (`error: this file contains an unclosed delimiter`,
  `impl Instance` at line 1005). The audit therefore ran on the study's own
  commit. `hec.rs` is not on `main`.
- **Inputs.**
  - `sessions/g2-p41/spec.json`, sha256 `a1aef2b3…717c`.
  - `records.jsonl`, sha256 `9c2a0fdc…2b21`, as the artifact manifest pins it,
    restored from the same commit.
- **Command.** `ecbench audit-ir --spec sessions/g2-p41/spec.json --seq 0 … --seq 44`
  under valgrind-3.22.0, with `--tool=callgrind --cache-sim=no --branch-sim=no`
  and `RAYON_NUM_THREADS=1`.
- **Scope.** Every one of the 45 records, each re-executed once from its
  committed seed and target.
- **Instrumentation run, not a measurement.** Ir is Callgrind's simulated
  instruction count inside the `ecbench_before_solve` / `ecbench_solve`
  bracket.

## Result: pass

| arm | runs | reproduce `bitop/v1` total | reproduce every phase's primitive counts | recovered scalar | complete | Σ bitops | Ir / kbitop pooled [min, max] |
|:--|--:|:--|:--|:--|:--|--:|:--|
| `hec.ic_fb_walk` | 5 | 5/5 | 5/5 | 5/5 | 5/5 | 117,422,628 | **1,259** [1,233, 1,285] |
| `hec.rho_full_width` | 40 | 40/40 | 40/40 | 40/40 | 40/40 | 4,535,211,568 | **1,313** [1,309, 1,317] |

- **Reproduction.** Each re-execution reproduces the committed record's
  `work` block exactly: the total, and every primitive count of all six
  phases. The sums equal README.md's (117,422,628 and 4,535,211,568).
- **Instructions per gate.** IC/rho is **0.959**. The IC arm spends no more
  instructions per counted gate than its reference walk on the same curve, so
  there is no sign of uncounted work. The `bitop_unit_20261016` panel
  admitted arms within 2× of their rho.
- **Absolute level.** The level, about 1.3 Ir per gate, is near the top of the
  range of the repository's other arms (19–1,686 Ir per kilo-bitop in
  `research/bitop_unit_20261016/audit/final-panel.json`). The reason is the
  6-bit field: `p = 41` makes a `field_mul` 450 gates, while the machine word
  operation does not shrink. Only the ratio between arms on one curve is the
  diagnostic.

## What this admits, and what it does not

- Both arms now meet BITOPS.md's two conditions for ranking in `bitop/v1`:
  complete records, and a passed instruction audit. The study's IC/rho ratio,
  0.207 at this row, is therefore a ratio of two audited counts.
- The validation gate's three artifacts are still missing, so no leaderboard
  or `benchmarks/` row follows:
  - an independent replay certificate;
  - a validator report;
  - a `VALIDATION.json`.
- The study's own counting conventions still apply (README.md, "Limits"):
  - one word operation per RNG draw;
  - memory movement uncounted;
  - operand validation inside the total, which the self-check policy reports as
    a separate subset.
- The audit is per arm. Callgrind saw one interval per solve; the
  hyperelliptic phases change inside inner loops, so they were not split.
