# Ordinary-prime IC baseline frontier at main 40c65545

Date: 2026-11-23. Tracking: PR #4956. Scope: bounded source and historical-note
review; no arithmetic, build, test, benchmark or campaign was run for this review.
Status: source-review observations and historical producer reports,
**[pcc:unverified]**.

The reviewed main revision contains applicable word-arithmetic pair-table and
batched-inversion improvements for ordinary-prime IC. They are an
**optimization/control candidate pending a separately qualified exact source
closure**, not a proven faster available control. The inspected module also
contains conflicting top-level table definitions. Keep the pinned `prime_fast`
and cryptanalysis457 specialized-table controls; this review supplies no matched
fixed-target result that would replace either one. [pcc:unverified]

## Sources and immutable identities

All paths in the following table are in `isogeny-labs/crypto` at
`40c65545b44a41fb20945deb7a955dcd6989e87b`. The relevant implementation changes
are in that revision's ancestry; they are not an immediate first-parent change
introduced by commit `40c65545` itself.

| Primary repository path | Git blob |
| --- | --- |
| `src/cryptanalysis/ec_index_calculus.rs` | `6f49bd02dd4a918e0a931fdbccc433aec9553ed3` |
| `research/prime_32bit_20261122/README.md` | `5879d1bedc5d0773139dd8a19316410ca2e5c374` |
| `research/prime_36bit_20261122/batchinv/BATCH_INVERSION.md` | `4ff297624c482982543af36ee55c5fc6e0d57235` |

The exact `src/cryptanalysis/ec_index_calculus.rs` SHA-256 is
`e5be394572758675e0d7849027266f4f1aa0b74ddd8947810c9b572224866dbf`.
Additional primary notes read at that same revision were
`research/prime_36bit_20261122/README.md` and
`research/prime_36bit_20261122/PAIR_TABLE_U64.md`.

| Relevant implementation history | Exact commit |
| --- | --- |
| Build pair sums directly in word arithmetic | `9e8db9c943d94d6bfe57ea1967326077888a8687` |
| Batch-invert MITM relation-probe denominators | `00262fb7907f95cbfef76106900b690505dede3d` |
| Subsequent merge of main into the MITM branch | `737a14680b99cfddb0d81e3be0bc853d9bc61ae8` |

Existing study identities are recorded in
`research/prime31_aux_matched_20261122/isogeny-choice/upstream/manifest.json`
(`prime_fast`) and this directory's `upstream/manifest.json` (specialized source
from `isogeny-labs/cryptanalysis` at
`457ae1818aaf8134bd833ad9c1b9d57015264510`). This review did not re-audit those
implementations or infer a ranking from their source age.

Sources checked (KB): the parent task already recorded KB unavailability. This
bounded follow-up used the exact repository sources above and the study's
`isogeny-choice/PROTOCOL.md`, `SCOPE.md`, and one prospective v3 cell to identify
the fixed target. It made no novelty or exhaustive prior-art claim. Conductor's
scope-check command was attempted before this document edit but returned exit
127 (`conductor: command not found`); work remained in the parent-assigned file.

## Current-main source integration conflict

Line numbers below refer only to the pinned Rust blob above.

| Symbol | First definition | Second definition |
| --- | --- | --- |
| `pub struct PairSumTable` | Line 2383: MITM three-point table with `by_x`, `by_x_u64`, word factor-base points and field parameters | Line 3912: compact two-point fingerprint table with `fps`, packed `vals`, points and `a_fe` |
| `pub fn build_pair_sum_table` | Line 2427: constructs the MITM table | Line 4008: constructs the compact fingerprint table |

These are in the same module's top-level scope. Neither pair is enclosed in a
separate module or selected by a `cfg` attribute. The later test module begins
at line 4926 with `#[cfg(test)]`, followed by `mod tests` at line 4927, after
both definition pairs. The two representations have incompatible fields and
their respective consumers use those different fields. The source therefore
has a duplicate-definition conflict, not an intentional pair of conditional
implementations. This is a source observation; no compiler was run in this
review. [pcc:unverified]

A separately qualified closure must distinguish the table families and their
call sites, or otherwise select the intended complete implementation without
silently changing its semantics. The independent standalone study qualification
does not by itself qualify this whole current-main module or remove its duplicate
definitions. This note does not edit the shared module.

## Applicability and differences from the fixed study

The study's recorded original-curve input is:

| Parameter | Recorded value |
| --- | --- |
| `p` | `2147483647` |
| `a`, `b` | `847514846`, `2087444724` |
| Prime subgroup order | `2147446831` |
| `G` | `(625037715, 2100627251)` |
| Public `Q` | `(984837385, 1142774582)` |
| Relation arity | `3` |
| First natural-query and rho seed | `4188` |

These values are transcribed from the study input, not recomputed by this review.
The prospective v3 cell
`panels/recovery-candidates/p31-prepared-recovery-candidates-s0-00.json`
retains them; a prospective cell is not a selected/admitted experiment.

The new table builder selects its word path when `curve.p.bits() <= 63`, so the
fixed `p` is inside its field-size range. Its point addition accepts the actual
curve coefficient `a`; it is not restricted to the archived `a = p - 3`
fixtures. It builds unordered pair sums directly in word arithmetic, avoiding
BigUint point additions merely to obtain word-sized lookup keys.
[pcc:unverified]

`find_one_relation_s4_via_mitm_counted` samples `R = alpha*G + beta*Q` and probes
the signed three-point relation family. For each sample, the denominators
`x(F_k) - x(R)` are batch-inverted and reused for both `R + F_k` and `R - F_k`.
Zero denominators use the original addition path. Candidate hits still enter
`try_finalise_s4`, which re-adds the signed points in the BigUint group before
accepting a relation. The optimization does not introduce a different relation
family. [pcc:unverified]

The complete entrypoints are
`ec_index_calculus_dlp_s4_mitm_staged` and
`ec_index_calculus_dlp_s4_mitm_staged_with_la`. Their shared body charges the
factor base, table preparation, relation collection, linear algebra modulo
`curve.n`, and final `[x]G = Q` verification within the solve. Dense or sparse
linear algebra can be selected. This makes the implementation family relevant
to a complete control, subject to the source conflict and these differences:

- **Factor base:** the S4 shared body calls `build_factor_base`, enumerating
  small x coordinates. It does not consume the study's auxiliary fiber or its
  exact matched plain base. A projected factor-base builder exists elsewhere
  in the module but is not exposed by these S4 entrypoints. Subgroup membership
  and any projection requirement must be established for the exact input;
  this review did not revalidate curve order/cofactor facts.
- **Sampling:** these MITM entrypoints expose no seed argument. Their finder
  calls `random_scalar` directly. Archived target seeds therefore cannot be
  assumed to specify the same natural-query stream as the study's explicit
  seeded collector. A matched control needs its source-defined sampling rule
  and randomness recorded.
- **Bounds and accounting:** relation trial/attempt bounds are present, but
  the inspected API is not the study's selected-cell runner. The 120-second,
  4096-MiB, one computational thread/affinity CPU, no-retry and original-deadline
  envelope still needs exact integration. Stage milliseconds are not complete
  calibrated operation counts.
- **Comparative standing:** no reviewed artifact solves this fixed `G,Q`
  under a matched admitted protocol against `prime_fast`, the specialized
  table and optimized negation rho. Source applicability alone supplies no
  performance ranking. [pcc:unverified]

## Historical timing evidence and its limits

The 32-bit README uses `p = 4294967291`, `a = p - 3`, `b = 267`, and
`n = 4295072597`. The 36-bit notes use `p = 68719476731`, `a = p - 3`,
`b = 33`, and `n = 68719022539`. These are different known-answer fixtures,
not additional measurements of the fixed Prime31 problem.

- The word-table note reports about a 13-fold table-stage reduction and a
  roughly 2.2-fold complete CPU reduction on its 36-bit fixture. The latter
  comparison also changes the factor-base size from 2048 to 3072, so it is
  not an isolated implementation-only comparison at a fixed base.
- The batch-inversion note reports a 1.73-fold internal complete CPU
  improvement at fixed `B = 3072` over four archived target seeds. It reports
  successful scalar checks, but this review did not independently replay the
  raw runs or audit their source/binary/admission chain.
- Both note families mark their results `[pcc:unverified]`. Their shared host
  was heavily loaded; CPU totals and wall-clock stage timers must not be
  conflated or imported as calibrated complete-operation evidence.
- The 32-bit README expressly withdraws the earlier IC-versus-rho language.
  The initial rho was an unoptimized single walk. The correction reports a
  much faster tuned `cryptanalysis` rho using r-adding, negation,
  distinguished points and batched affine addition. Its reported IC/rho
  ratios, as well as the 36-bit ratios, remain statements about those archived
  fixtures and cost definitions. They are not fresh matched Prime31 results.

These observations justify retaining the improvements in the candidate control
frontier and retaining the strongest applicable rho reference. They establish
no admitted advantage or ranking for this study. [pcc:unverified]

One source comment must not be reused as a complexity justification: the MITM
comment states a cost `B^2 + p/B` and then gives a balancing exponent inconsistent
with that expression. This review makes no asymptotic claim from that comment.

## Next qualification boundary

Preserve the existing control pins. If this implementation is added, first
produce an exact source closure resolving the conflicting table identities,
record its base and sampling semantics, and qualify complete recovery and
resource/accounting integration. Only a separately admitted fixed-target
comparison can establish whether it strengthens the measured control frontier.
No root-module repair, selection, publication receipt, image, job, budget or
execution admission is created by this document.
