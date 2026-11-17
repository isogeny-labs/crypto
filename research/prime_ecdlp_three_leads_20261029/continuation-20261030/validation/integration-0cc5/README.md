# Fresh validation of the integrated source

Source merge: `a030c1630518564bdb3633815391858b424377ce`, parents
`45b53fbc8386d6be13b2f96cc9f805bffe69dd94` and
`0cc5e0d23a37a502dd363e41d866ec4991d3afb3`.

The changed Lean gate passes **18 declarations, zero trusted exceptions**.
Official module kernel replay passes against imports, not `--fresh`.
[Proof receipt](proof-receipt.json) binds the exact merged source/dependency/gate
objects and raw/stored log hashes. Earlier proof receipts remain unchanged.

Fresh native validation **stopped at compilation with Cargo101**. No new tests
ran; all eight later phases were not reached. [Failure receipt](native-failure/receipt.json)
binds the source, exact small logs and terminals. The unclosed delimiter in
`binary_semaev_chain_sat.rs` is byte-identical at a030, PR head0cc5 and its main
parentd4b3, matching earlier comment6526695631. It was not patched or waived.
Current main's file preview contains a repair, but that build has not been
validated here and the PR now reports conflicts. Copilot-first integration is
required before adopting a new source state.

The following is the historical launch description. Detached supervisor46238
was launched at18:37:34UTC with parent1 and SID/PGID46238. Directory:
`/workspace/pkm-scalar-certificates-20261030/remote-0cc5/`. No interactive exec
session exists. At the launch check Cargo46654 and rustc children were live,
compiling the new dependency state. All17 committed source/fixture objects and
tracked cleanliness are checked before every phase. Recipe/bindings are
committed here; the older31539 recipe is terminal and must not be restarted.

The sequence remains three example suites, strict lint/format, CLI build and
actual checkpoint-refusal message/exit1, backlog build/validate, library build,
then a2400-second serial library execution cap. Compilation is outside that
execution cap. No bound source edit or newer-base merge while the job is active.

Collect actual processes/children, phase started/finished/exit/log records and
supervisor.exit/finished. Missing terminal/dead process is incomplete. If the
suite caps, reconcile its actual declared/pass/fail/ignored/active/not-reached
counts; the new declared count is not assumed to equal the old5646. Wrapper124
is not Cargo exit or a full-suite pass. Preserve failed/capped evidence and
diagnose change-caused failures without skipping tests or bypassing gates.

Raw logs larger than64KiB stay out of Git: retain final size/hash/local path,
bounded excerpt and NOT_UPLOADED if approved artifact transport is absent.
No compression or splitting to evade policy. Older native-final-faade receipts
validate only their own source. The failed run is terminal. A repaired integration requires its own fresh
checks and binary bindings before ordinary publication. Auto-merge remains paused.
