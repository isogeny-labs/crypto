# Selected-root development qualification

The next runtime lead avoids constructing vertex consequences that the compact
constructor discards. `solve` now selects the highest eligible vertex lazily;
`solve_all_vertices` retains the previous eager path as an explicit ablation.
Both use the same resultant, Euclidean GCD, coordinate and change-of-order
kernels. The lazy route also reuses its recognized-pair list for coordinates.
The fixed 15x runtime target remains open [pcc:unverified].

## Scope and correctness

Canonical edge resultants are cached by recognized-pair index, including zero
results. Candidate vertices are examined in descending index order; incident
projections and GCD reductions retain their old order. Each used orientation
and selected root has an exact original-input identity check. All compact
local-identity, containment, reducedness, S-pair and native boundary checks
remain; caps and unsupported-input admission are unchanged.

Native APIs expose `try_compact_quotient_basis` and the explicit
`try_compact_quotient_basis_all_vertices` ablation. The same worker executable
supports `compact` and `compact-all-vertices` alongside unchanged `f4` and
expanded `certified` controls. The all-vertices arm is an optional diagnostic
ablation; it does not replace either frozen protocol control or add an
unregistered route for declaring the 15x target met.

## Fixed development results

Source e060f3ee6 implements the lead. fc85acc26 only corrects a test's use of
a sibling-private helper; production arithmetic and native tests are unchanged.
The initial failed core build is retained in selected-root-development/.

The immutable core release qualifier at fc85acc26 completed 32 tests; its
counter-feature counterpart completed 33. On seed-7 n=3/4/6 systems over both
requested primes, lazy and eager paths have identical full compact definition
and final-basis cofactor rows, and their complete basis equals the expanded
constructor's basis. Disconnected/unused vertices are still rejected. A
disconnected inconsistent upper triangle forces three ineligible candidates;
each of its three edges is constructed only once before the lower cycle
supplies its root. Producer records PCC-20261116-025524 and
PCC-20261116-41e626 are unsigned and unverified [pcc:unverified].

On each fixed six-cycle the asserted structural diagnostics are:

| Constructor | Edge resultant constructions | Vertex GCD folds |
| --- | ---: | ---: |
| Previous all-vertices compact | 6 | 6 |
| Lazy selected-root compact | 2 | 1 |

One fold may contain multiple Euclidean reductions. These counts exclude
coordinate construction and certificate/basis checking; they are neither total
field-operation counts nor time or memory measurements. Certificate storage
and degree diagnostics retain their prior meaning. Neither an F4 solving
degree nor degree of regularity is inferred [pcc:unverified].

Actual-root optimized qualifiers built at e060f3ee6 completed six API checks
and three CLI contract checks. The API case checks both-field six-variable
lazy/eager full-certificate equality and rejects altered public data. The
complete CLI case compares F4, expanded, lazy compact and eager compact bases;
the other cases retain capped-F4 incompleteness and unsupported wide-F4 status.
All three copied native executable hashes match before/after calls. Records
PCC-20261116-786af5 and PCC-20261116-31210e remain unsigned and unverified;
source-relationship-01.json records the test-only difference from fc85acc26
[pcc:unverified]. The full root unit suite is not claimed to pass.

Strict release clippy completed for the shared normal/counter targets and the
actual-root qualifier package's targets (PCC-20261116-1e80f2, unsigned and
unverified). Both qualifier archives retain separate source/hash envelopes;
the migrated core archive's content-addressed pointer matches its envelope
[pcc:unverified].

## Runtime admission and next step

No scientific timing was run. COMPACT_SPEED_PROTOCOL.md remains frozen with
matched complete-basis controls, paid checking, isolation/A/A admission,
conditional fresh seeds and adjusted lower endpoints above 15. Checkpoint
access is still unavailable: managed status has no bound secret or outbound
identity. No fresh holdout has been generated. After access and execution
admission, qualify the backend adapter and run the registered campaign; report
the actual outcome, including a loss or an incomplete control.

Sources checked (KB): unavailable MCP and wrapper executable. Fallback sources
and the primary Isabelle source are recorded in SELECTED_ROOT_HYPOTHESIS.md,
frozen under PCC-20261116-e3e64b. No novelty or new algebraic theorem is claimed.
