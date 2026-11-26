# Blocked x-only BSGS versus batched rho

Status: protocol frozen; implementation and measurements pending.

This study will compare the existing paired x-coordinate BSGS kernel with the
existing single-thread batched negation-map Pollard rho implementation on fixed,
public, prime-order toy curves. The primary workload is one cold target with all
solver setup charged. A separate secondary panel may reuse one BSGS table across
64 targets; it is not evidence about the cold one-target problem.

The frozen design is in [`PROTOCOL.md`](PROTOCOL.md), with pre-timing
corrections to the admitted CPU envelope in
[`PROTOCOL-ADDENDUM-1.md`](PROTOCOL-ADDENDUM-1.md) and the lossless seed-custody
encoding in [`PROTOCOL-ADDENDUM-2.md`](PROTOCOL-ADDENDUM-2.md). The randomness
contract is in [`seed-manifest.json`](seed-manifest.json). No timing result has
been collected or interpreted yet.

## Sources checked

- `src/cryptanalysis/bsgs_fast.rs`, especially `BsgsFastPlan`, the paired
  kernel, candidate verification, and table accounting.
- `src/cryptanalysis/prime_fast.rs`, especially `RhoConfig::tuned`,
  `rho_parallel`, collision verification, and `RhoOpCounts`.
- `research/lit2026_w11_generic_engineering_20261011/README.md`, which measured
  the paired BSGS kernel against its chain predecessor but did not time it
  against rho.
- `research/ic_e222_m221_20261106/interleaved_rho_bsgs/{PROTOCOL,README}.md`,
  which supplies the balanced scheduling and A/A pattern for different
  order-6 multi-target algorithms; its timing rows are not reused here.
- `docs/experiment-seeds.md` for domain-separated public seed custody.
- The repository knowledge-base wrapper was invoked, but no `research-kb`
  executable or MCP endpoint is available in this runtime. No novelty or
  absence claim is made from that failed search.

Daybreak independently checked the repository history and found no existing
same-host benchmark of these exact two implementations. That review is a design
input, not signed validation or an execution result.
