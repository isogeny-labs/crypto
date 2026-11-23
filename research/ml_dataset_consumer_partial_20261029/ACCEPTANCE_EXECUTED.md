# Acceptance ledger — final local software gates

This ledger distinguishes authored contract targets, development execution, and
final immutable execution. Runtime/test source is
`9e1d8e3817bb69285462957de72c7fac2f6983c6`; the delivery documentation does not
change that native tree. No blanket completeness claim follows from test counts.

## Positive coverage

| Rows | Concrete case |
|---|---|
| P01/P14 | Original producer policy-only anchor and relocated CAS mirror; seven context artifacts/6564 bytes; raw metadata retained |
| P02 | Legacy actual five-file binding with opaque optional role/diagnostic bytes |
| P03/P10 | Three snapshots; two committed segments; ordinary event plus empty and non-UTF8 payloads |
| P04 | Policy→start→end→next start→observation-only interrupted prefix |
| P05 | Recorded operation error and terminal exit, no subsequent phase |
| P06 | Four recorded phases, both retained mocked acquisition and synthetic unavailable-counter variant |
| P07 | Full acquisition from retained source; permitted fallback with closed descriptor reuse and zero kernel ID |
| P08 | Explicit unsupported big-endian diagnostic, absent native calls/TIDs |
| P09/P11 | Absent versus explicit empty capture; first stop, unchanged stop, absent later delta |
| P12 | Omitted nullable fields and independently reordered metadata with new raw pins |
| P13 |1000-digit integer protocol; separate raw-plan/canonical-plan domains |
| P15 | Full mocked HEAD/conditioned GET chain and payload; actual-remote flag remains false |
| P16 | Saved report claims ignored by fresh audit |

Successful test wrappers independently calculate metadata/artifact/context,
logical-object, consumed-reader and pre-report-write totals from original raw
fixtures and the required read passes. They assert a positive artifact-open
counter control. This is fixture arithmetic, not a validator-derived oracle.

## Negative coverage and scope

The 30 named N01–N30 functions each contain one or more contract-authored cases.
Each case records its new anchor pin, intended semantic error, actual error,
fixed/rebuilt/stale-event/tree-mutation hash mode, and artifact-open count.
Promised preflight cases assert zero source artifact opens before the error.
Semantic mutations refresh outer hashes through the caller-pinned anchor.

Representative coverage includes duplicate decoded keys, strict wire types,
profiles, missing/gapped ancestry, cross-binding, reference aliases, ordered
membership, namespace conflicts, raw and canonical context, protocol failures,
observations, all envelope claim fields, capture state, raw LF-inclusive event
chains, same-delta payloads, policy/phase/native mismatches, early-prefix faults,
uncommitted tails and forged saved reports. The full unsupported/native lifecycle
matrix also remains covered by the existing mandatory phase unit suite.

Explicit limits: the finite writer does not construct a cryptographic receipt
hash cycle; N28 rejects invalid early bytes but does not instrument the exact
phase-core invocation ordinal; each matrix row's representative cases are not
an exhaustive enumeration of every possible field/counter mutation. No source
review or test count is relabeled as scientific or actual-cloud evidence.

## Resource, filesystem and transport coverage

R01 exercises exact and+1 anchor/metadata/line limits with real local fixtures;
artifact/reference caps are checked before materialization; all19 finite context
identities and ordinal255/256 are checked. Full8GiB/256-snapshot/64MiB-artifact
transfers are not performed, and cumulative admission may refuse earlier.
R02 uses infinite-reader sentinels plus bounded subprocess output/time tests in
the existing transport suite. R03 counts repeated CAS references without discount
through logical byte/count limits. R04 rejects node/key input before owned
deserialization. R05/R08 cover precharged growth, canonical scratch, arithmetic
and bounded report construction. R06 shares child budget across prefixes and
final audit. For R07 the chosen implementation uses fresh charged event values;
it does not claim an arena reset/reuse optimization.

R09/R10 include nonblocking FIFO child with a fixed timeout, symlink/ancestor
replacement, hardlinks, same-size content replacement, and one pinned-root swap
counterexample. R11 mock failures cover length,412, command timeout, corrupt and
overlong GET, with no retry/fallback; conflicting inherited AWS settings are
overridden in a subprocess. R12 checks exact namespace, missing member and extra
empty-directory/tail/helper files. R13/R14 inject file/intermediate-directory
sync, pre/post rename and pre-marker-removal failures: nonzero result, incomplete
marker, and refusal of subsequent adoption. They are deterministic software
failure tests, not power-loss or filesystem crash certification.

## Development evidence

All raw failures remain in consumer-partial-validation. check-01 failed on macro
recursion; partial-dev-01 failed because a removed helper was still referenced;
partial-dev-02 had one incorrect replacement-test assertion. Corrections were
made to their causes. dev03 passed37/0/1; positives04 passed12/0/0; integration05
passed72/0/1 with original negative gates; limits06 passed7/0/0. Clippy-dev01
reported an overwide report function and temporarily unused counter API;
clippy-dev02 passed after the function-interface correction and test integration.
These are uncommitted development executions, not immutable release acceptance.

## Final source-bound execution

The final release suite passed **239 tests, 0 failures, 9 opt-in tests ignored**.
Strict release all-target Clippy with `-D warnings` passed. Twelve executables
were copied and hashed before execution; both original and copied binaries were
rehashed after the suite. The final CLI SHA256 is
`04ea30dacf6f2755239874b7e1b66107348d67c349aae6845694c1a7895ba591`.

The separately invoked actual CLI test passed eight local restore/fresh-audit
pairs on NFS: P01, P03/P10, P04, P05, P06, P07, P08 and P13. Each checked report
retains the restricted partial claims. All other positive/negative/resource
representatives above execute in the mandatory release suite, including the
positive source-open counter control.

The prior freeze `e430ad68789191ec81b58cd4cc9b8bc70c3db540` passed its 236-test
suite, strict Clippy, 42 exact sealed legacy/checked audit and export comparisons
against the retained baseline, and the mocked legacy fetch CLI byte comparison.
Its actual partial CLI failed with EINVAL on the migration NFS mount. That failed
run, source identity, binaries and all fixtures remain unchanged in the evidence.
The reviewed correction adds exclusive link installation for unsupported rename
flags, preserving occupied destinations and returning failure without automatic
adoption on errors. Tested pre-removal faults retain the marker; a late sync
error after marker removal may leave it absent, without returning success. Three
additional tests exercise positive, occupied-name, source-replacement/topology
and intermediate-failure cases. See `NFS_INSTALL_AMENDMENT.md`.

The only production path changed after e430 is `partial/io.rs`; other changes
are test retention/cases and documentation. The 42 sealed comparisons and mock
legacy fetch remain labeled **e430 evidence for unchanged code paths**, not
new-binary executions. The final source underwent the full release and lint
gates plus the new NFS CLI replay. Actual remote invocation remains unperformed,
with no generation, solver-resumption or worker-publication claim. The separate
durability record identifies controller S3 archival and fresh restored replay.
