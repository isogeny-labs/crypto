# ecdl-tensor-ic Rust port (2026-10-29)

Native Rust port of the Go `ecdltensor` prototype (toy index calculus for
ECDLP with relations found by Boolean tensor contraction) as
[`crates/ecdl-tensor-ic`](../../crates/ecdl-tensor-ic/README.md). Go source:
carrier branch `research/ecdl-tensor-go-source-20261029`, commit 1c29f66.

Port verification report: [`reports/run-20261030T0017Z/report.pdf`](reports/run-20261030T0017Z/report.pdf)
(source `report.typ`, graph `gadget-shapes.svg`, raw outputs beside it).

Result: all 19 Go tests ported and passing; `ecdlscale` output byte-identical
to Go; both demos recover ξ with identical network shapes. This is a research
prototype, not a performance result: no leaderboard, `benchmarks/` or
validation-gate entry. No speedup is claimed; the counters are exponential.
