# Native launcher argument check for the K13 L2 admission spec

The committed v3 admission spec is accepted by the native
`isolab_k8s_benchmark` argument builder in a source-compiled Rust test. This
is a local parser and command-construction check; it did not submit an image
or Job and does not measure solver performance. The losslessly compressed raw [stdout](test.stdout.gz),
[stderr](test.stderr), [machine receipt](RESULT.json), and [verifier](verify.py)
bind the result. [pcc:unverified]

## Source and control

The added [test](../../taskq/deploy/eks-karpenter/mcp/src/main.rs) includes
the exact [v3 spec](../rootfind_k13_l2_admission_probe_20261116/SPEC-K13-ADMISSION-v3.json)
at compile time. It calls `validate("isolab_k8s_benchmark", ...)` and
`benchmark_args(...)` without calling `call(...)`, `kube(...)`, or a submit
command. The test checks that the command selects `c8a.2xlarge`, requests the
`l2` isolation profile, embeds the identical spec, and retains five measured
rounds, one warmup, and `L2` in that embedded document. The crate's existing
negative controls reject insufficient rounds and a missing A/A control.
The prior [source-plan audit](../rootfind_k13_source_plan_audit_20261116/REPORT.md)
binds all 288 planned executions to the frozen ten-arm panel. [pcc:unverified]

The Rust 1.99.1 release test command in `RESULT.json` exited zero: eight
tests passed, zero failed. `cargo fmt --check` passed after formatting the
new test. The first passing pre-format run is retained as
[`test-preformat.stdout.gz`](test-preformat.stdout.gz) and
[`test-preformat.stderr`](test-preformat.stderr); its initial format check
requested only line wrapping. The final source, lockfile, v3 spec and raw
outputs are SHA-256 bound in `RESULT.json`. [pcc:unverified]

## Scope and next gate

The local argument builder accepted the request shape; a deployed launcher,
container image, EKS admission, L2 executions, answer checks and independent
replay remain untested by this check. The current AWS profile returned
`UnrecognizedClientException` when reading both daily and monthly budgets
([daily stderr](budget-daily.stderr), [monthly stderr](budget-monthly.stderr)),
so available headroom is unknown. The last recorded monthly spend was over
its limit, and Conductor T-6814 still reserves the checkpoint path. Those
conditions prohibit a new build or Job under
[`COST_CONTROLS.md`](../../taskq/deploy/eks-karpenter/COST_CONTROLS.md).
The next step is to restore the existing account identity, read back both
budgets and reserved spend, and pass the branch-specific checkpoint gate;
only then can v3 be submitted. The complete-cost requirements remain in the
[existing problem](../../problems/rootfinding-complete-cost-calibration.md).
[pcc:unverified]

The original [rootfinding manuscript](../../papers/rootfinding-decomposition/rootfinding_decomposition.tex)
and [program review](../../papers/ecdlp-index-calculus-program/sections/rootfinding.tex)
describe the pair-table method; this change is a launch validation, not a
new decomposition method. Shared KB source IDs
`source:17a1b67993e4b781bbf1198fd2598c2f` and
`source:e9db8cc2be204429e8fb1daa12a3324c` were checked. KB revision
`9141e386254340a9bdba3036f2bb706a` was last refreshed on 2026-11-14,
before this study; current local files supply the source identity.
[pcc:unverified]

![Local acceptance and remaining execution gates](flow.svg)
