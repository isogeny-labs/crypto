# Development checks and source boundaries

All observations below are producer assertions with **unverified** standing.
Native statement `PCC-20261106-5e324a` pins
[`validation/final-components.json`](validation/final-components.json);
`PCC-20261106-34db73` pins the preceding
[`validation/completed-components.json`](validation/completed-components.json).
Neither is a signed execution or performance verdict [pcc:unverified].

## Final algorithm and integration boundary

Algorithm/reader source commit: `413a93d6f9940c855200468175174126136bb49c`,
tree `7de87030db563d43eaaf746275b876c395534925`.

* Public lead tests: 8 passed. These exercise actual producers, the separate
  ordinary-residue reader, literal-cell tampering, retained full/frontier
  agreement, all guessed coordinates, certified orbit pruning and overflow
  refusal. The eighth runs a fresh native L0 scratch session through planning,
  child execution, record sealing, static audit and the separate reader.
  The rho control alone receives deterministic count replay; APS does not.
* Existing APS public integration: 4 passed; producer/reader bridges: 3 passed.
* Existing APS example: 10 passed; the scalar-check example builds with no
  unit cases. No test duration is a runtime measurement.
* Strict Rust 1.98 and stable lints passed for the final public test target.
  The immediately preceding core/registry/optimization boundary also passed
  both strict root library/tests/example lints [pcc:unverified].

The core closeout started at `01f4059d4`; its tested unit inputs are unchanged
by the later public-test-only commit `413a93`. Its registry matrix has 7
passing cases and optimization inventory 4. Focused APS, F4, adapter,
Callgrind-reader and GLV selections passed 40/76/14/5/15 respectively. The
`f4_fp` filter includes the existing tower cases. These unit reruns use the
same release optimization/assertions and maintained 256 root codegen units,
with line-table debug info to produce a distinct Cargo artifact while the
older broad binary remains live. This is correctness evidence; do not use it
for performance comparison [pcc:unverified].

## Separate reader and formal boundary

At `aac11676180d17c6b013bbcad67c82f61b546057`, the standalone reader passed
55 library and 5 CLI tests on both stable and Rust 1.98, plus format and
strict lint on both. Those reader sources are unchanged at the final source
pin. Synthetic profile packets test the contract, not actual Callgrind
execution or costs. `PCC-20261106-34db73` has unverified standing
[pcc:unverified].

`PCC-20261106-40dd8e` binds the three core-only Lean frontier declarations.
The local native kernel check passes with zero research-trusted axioms;
ordinary Lean foundations remain visible in the proof map. Its **standing
is unverified** without a signed native verdict. This does not prove the
Rust implementation, finite-field chart lifting, F4 or complete-cost bounds
[pcc:unverified].

## Broad-suite and environmental limits

The actual broad release library command compiled source
`ffd0bd0dd33f53e53bd8f5526ffa1c2ad20cce30` and is still running at this
receipt boundary. Its failures include missing tracked inputs, unavailable
Linux time-namespace procfs and three introduced global-inventory omissions.
The latter were repaired and rerun above without changing assertions,
skips or exemption lists. Exact tracked-input restoration made six emulator
cases and one historical bound pass on targeted reruns. None of these
targeted outcomes replaces the broad command's eventual failed outcome;
no final-source or new-head full-library pass is claimed [pcc:unverified].

The earlier compile failures, resource interruption and invalid positive
orbit fixture remain in additive continuation receipts. The orbit fixture
was corrected to the existing certified Random-label example while the
Guaranteed-label refusal was retained; no production guard was weakened.

Global format checking found eight unrelated paths and no task overlap;
the scoped files were checked separately. The whole workflow-wiring check
ended at its metadata-lookup budget without a verdict. The whole benchmark
record check sees no records in this minimal checkout and stale global
progress; this task adds no benchmark result and does not regenerate that
unrelated progress. Raw build/dependency logs, host details and the full
native append-only ledger stay private [pcc:unverified].

## Remaining admission

No source checkpoint, profiler session, performance pilot or EKS experiment
is asserted here. The source-only gate needs its actual bounded native
admission and hosted outcome. Complete pricing/accounting, independent
metric/full-set replay, exact registered four-size curves, frozen disjoint
pilot/held-out protocols, native analysis/power/budgets and run admission
remain open. A merged component PR does not complete T9 or the research panel.
