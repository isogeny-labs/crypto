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
`ffd0bd0dd33f53e53bd8f5526ffa1c2ad20cce30` and finished with exit 101:
6,098 passed, 11 failed, 133 ignored. Native statement
`PCC-20261106-b56be1`, standing **unverified**, pins
[the final broad outcome](validation/broad-library-final.json). Six failures
lacked tracked inputs, three were introduced APS registry/optimization
inventory omissions at that compiled source, and two required unavailable
Linux diagnostic-clock procfs. The inventory omissions were repaired and
rerun above without changing assertions, skips or exemptions. Exact input
restoration made six emulator cases and one historical bound pass in
separate targeted reruns. Those later passes do not replace this failed
broad outcome. No final-source or later-head full-library pass is claimed
[pcc:unverified].

The earlier pending receipts retain their original boundaries. Required
hosted jobs passed on head `f33f6dda9e4fdae869b2401572b1717454d99e84`;
that hosted observation is separate from the failed FFD broad run, and its
passes do not transfer to a later outcome-documentation head
[pcc:unverified].

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

Subsequent checkpoint observation: the hosted source1bc custody/standard gate
passed, under unverified native statement `PCC-20261106-2d094d`. The artifact
body download returned HTTP403 and independent stored-object readback remains
open. [CHECKPOINT-OUTCOME.md](CHECKPOINT-OUTCOME.md) records this newer boundary;
the earlier preflight failure and repair below are preserved. No speed,
scaling or performance-admission result follows [pcc:unverified].

The first hosted source-checkpoint job stopped in its exact-payload preflight;
the standard gate was skipped and no upload/authentication outcome followed.
The repository ignores copied nested lockfiles. The new regression reproduces
that refusal, then all 13 custody cases pass after the snapshot receives
exact path-specific ignore exceptions. Native statement
`PCC-20261106-21c262` pins
[this administrative repair](validation/source-custody-repair.json), standing
**unverified**. The earlier 12-case receipt is preserved at its own source
boundary; a new upload needs a fresh fenced admission [pcc:unverified].

Native statement `PCC-20261106-55ef66` pins the source-linked
[report checks](validation/report-checks.json) and
[development-policy receipt](validation/development-policy.json). It has
**unverified** standing. The report's three pages were inspected individually;
Markdown/TeX semantic text agrees after removing only table-layout commands
from a private reader copy. The actual rendered TeX/PDF was not changed by
that comparison. Scoped formatting, actionlint, twelve administrative custody
tests and local size/artifact policies passed. Whole-repository curve-name
lint stopped on a missing tracked sparse file; the unchanged native
added-line checker separately found no task findings. This is not a complete
whole-repository validation or a hosted-check result [pcc:unverified].

The source1bc checkpoint acknowledgement above is its own boundary. No
profiler session, performance pilot or EKS experiment is asserted here.
Later run source/protocol changes need a fresh bounded checkpoint admission.
Complete pricing/accounting, independent
metric/full-set replay, exact registered four-size curves, frozen disjoint
pilot/held-out protocols, native analysis/power/budgets and run admission
remain open. A merged component PR does not complete T9 or the research panel.
