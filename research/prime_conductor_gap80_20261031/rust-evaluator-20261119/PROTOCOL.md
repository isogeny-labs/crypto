# User-requested Rust evaluator: local execution

User requested Rust on2026-11-19. Isogeny scheduling, jets, prepared log norms,
coverage gates, coefficient reconstruction and twist transport are implemented
in `src/bin/ecbench/isogeny_eval.rs`. Field/Jacobian formulas reuse the unchanged
repository Montgomery source. `isogeny_eval/flint.rs` owns contexts/polynomials
through an Rc-based checked dynamic-library interface; a small C file exposes
existing FLINT primitives and allocation/string ABI only. No C++ constructor is
called by the Rust evaluator. This is Rust with explicit FLINT/GMP FFI, not a
claim that the numerical libraries are implemented in Rust.

Frozen build-d actual CLI and helper-library hashes must match before execution.
The18 Rust complete small-map controls match retained independent PARI references
on470 point inputs across two fields, square/nonsquare twists and blocked/full
rectangles. Eight rejection controls and six arithmetic/coverage/parser tests
pass. Scope clippy with warnings denied applies to port code; the unchanged
included Montgomery dependency keeps two indexed-loop lint exclusions only.
Original diagnostics and failed bootstrap attempts are preserved. [pcc:unverified]

Retained complete case: prime degree33555419, exact original field/subgroup and
33-point input from the native baseline. Run Rust main and its normalized dual
under the CLI's existing shared benchmark lock;300second caps and12GiB address
limit, max_window4096,block4096,batch64,scale2. The independent auxiliary probe
uses scale1. Require exact quotient and input/image agreement, original subgroup
checks and33 normalized dual compositions through independent PARI. A repeated
full constructor may be run on the same input, keeping each elapsed value. These
are whole-process L0 diagnostics including library load/hash, parsing/primality,
construction/evaluation and output; no separated transfer price or speedup. [pcc:unverified]

After that qualification, actually attempt the unchanged original80-bit prime
target in Rust, using the same33-point header/input, twist1, max_window500000000000,
block64,batch64,12GiB address limit and60second watchdog. Preserve timeout/OOM/
errors and missing output; incomplete traversal emits no quotient or point images.
This cap is a budget, not a mathematical lower bound or nonexistence argument.

No build/audit/other benchmark overlaps a measured call. All helper/library/binary
and input identities are frozen before timing; user authorization selects local
execution. Host metadata and actual L0 status remain recorded. Solver/scalar
results are separate Rust ecbench methods; their clocks are not constructor
clocks. Full32-target ECDLP and the original80-bit crossing remain open until
actually completed. [pcc:unverified]

Sources checked: exact repository Montgomery/Jacobian code, native complete
evaluator/prepared-norm identities and immutable33555419 baseline/PARI references;
FLINT headers/APIs; previously reviewed CM/ordinary-order sources linked in the
baseline CM_ARGUMENT.md. KB unavailable; no novelty assertion.
