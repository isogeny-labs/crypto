# Independent signed-extraction qualification readout

Date: 22 November 2026 Pacific / 23 November UTC.
Tracking: [crypto PR #4976](https://github.com/isogeny-labs/crypto/pull/4976).
Source: [immutable pre-call publication
4d1e768c7d0a6fdeef75d9a7f2597e262b87231b](https://github.com/isogeny-labs/crypto/commit/4d1e768c7d0a6fdeef75d9a7f2597e262b87231b).

**Outcome:** the separate-role replay matched the complete producer stream and
all 84 retained proof/input files. The saved reader accepted the fixed 42-pair
set, all 91 prescribed rejection controls and three positive checks behaved as
specified, and the separate diagonal-identity reader accepted its fixed finite
evaluation certificate and rejected both mutations. All four native calls had
exit status zero, custody status zero and empty stderr. These are unsigned
same-host development observations, with unverified PCC standing.
[pcc:unverified]

## 1. Frozen inputs and independence

The parent published the source, protocol and exact fixture selection before
the build. The [source freeze](../producer/attempt1/source-freeze.json) records
20 project files, including the actual lockfile, unchanged predecessor query
and norm modules, the production field/polynomial and SHA-256 routines, and
the separate identity-certificate module. The offline locked build pins
registry dependencies through the lockfile checksums.

| Object | SHA-256 |
| --- | --- |
| Source manifest | `276419918e6f0465349f6ee7091a0189a55efa10a2b95d3cfa39ded4d41261f8` |
| Frozen protocol | `0bae4bb72d5b62683cd14db608d4257f0f88527b5897a3e49301d8dec917c2ac` |
| Reader control specification | `180189398ff8d477d134c399f51663f148d77cb26972fd40f09f205fc23bcb53` |
| Cargo lockfile | `68a20a39035ef6e7d93c98056edc927f8d9e978830a6427b1196669adcc11fdf` |
| Immutable qualifier binary | `c1e54f2de1ae2803a009fcde7d0f1a9fb231062f768b11d0f6243dae8a5ead43` |
| Immutable diagonal binary | `fe290cfdbfc86debdb2e6df5e9224561e60a8cadc3b44b1f1c09f81acf855476` |

Every reviewer call used the frozen `run_bounded.sh`, a newly named input
manifest and receipt directory, and separate pre-call/post-call inventories.
The manifests include the 20 source files and both immutable binaries; the
replay and identity calls additionally retained custody of all original proof,
input, raw and identity-certificate bytes. This custody inventory is broader
than the files read by each executable, and is not a claim that every pinned
file was a runtime dependency of every call. All recorded hashes remained
unchanged. [pcc:unverified]

The new independent affine oracle does not call producer field, curve,
polynomial, S3, norm, query or extraction functions. It constructs the exact
selector and signed base, enumerates every ordered first pair, and determines
the only possible third point by its own group arithmetic. It independently
reconstructs all permutation and integer-row groups and their backpointers.
It uses a symmetric S3 expression and complete-grid products for original-input
binding. The unchanged predecessor independent polynomial checker checks the
quotient partition, cofactor and Bezout identities. Certificate data types and
SHA-256 are shared. The replay uses the same already-built executable; no
separate compiler or second host was used.

## 2. Calls and retained outputs

All calls used one thread and a 2-GiB virtual-memory limit. Replay, saved
reader and reader controls each had a 300-second deadline; the identity read
had a separate 60-second deadline. The shell wrapper captured source and
artifact checks, command, host, bounds, stdout, stderr, exit and custody
statuses. It did not overwrite an earlier receipt.

| Call | Receipt | Output bytes | Outcome |
| --- | --- | ---: | --- |
| Same-binary qualifier replay | [replay](replay/) | 42,695 | Complete stream and all 84 files identical |
| Saved-output reader | [reader](reader/) | 8,410 | 42 retained pairs; complete fixed registry and input/byte binding |
| Reader mutation panel | [controls](controls/) | 12,170 | 91 rejections and three positives |
| Saved diagonal-identity reader | [identity-read](identity-read/) | 521 | 32 original plus 32 extra evaluations; two rejected mutations |

Each outcome in this table is a finite development check, not a signed
execution verdict. No corrected-source attempt, retry or budget increase was
used. [pcc:unverified]

| Output | SHA-256 |
| --- | --- |
| Producer and replay raw stdout | `aac7736e0977f829eb2b638fc3a0ec45a134ebf9dd034c2103c4682c2b1dd7f8` |
| Saved reader stdout | `1cac4c477f2b1d54d82bc06f07a734ed74416dc1c01049fe5cb79b1f354242c6` |
| Reader controls stdout | `46b50f164716ab98314c8d8932c943b3d5d8a032f0686674a44e20b98c5114d4` |
| Identity reader stdout | `ff31a0bbc7ebae5b3943200d4e8a74d236f34fc6aa833372a157f8018dec4ad6` |

The replay's [comparison receipt](replay/comparison.txt) compares the raw
stream, complete filename inventory, and every artifact with the original.
The producer and replay files were left unchanged. The chronological role
record is [ROLE_LOG.md](ROLE_LOG.md); the earlier static review is
[SOURCE_REVIEW.md](SOURCE_REVIEW.md).

## 3. Full-panel observations

The identical [raw streams](replay/stdout.jsonl) report the unchanged 132-case
panel and all 16 invalid-input refusals. Every case was compared in memory
against the independently implemented group oracle and complete original-input
binding before its compact record was emitted. Complete objects were retained
only for the 42 cases named in the pre-call protocol. The other 90 objects have
their exact byte lengths and hashes in the raw stream and can be reconstructed
from the frozen executable; this readout does not imply their full bytes were
published. [pcc:unverified]

| Panel quantity | Recorded count |
| --- | ---: |
| Cases | 132 |
| Retained complete input/output pairs | 42 |
| Algebraic support coordinates | 9,391 |
| Ordered signed triples | 1,496 |
| Affine-prefix triples | 1,166 |
| Identity-prefix triples | 330 |
| Permutation groups | 320 |
| Integer-row groups | 198 |
| Empty signed outputs | 66 |
| Tuples containing an inverse point pair | 884 |
| Masked affine tuples without an inverse point pair | 66 |
| Algebraic support cells with no signed output | 8,483 |

Counts are summed across the fixed cases, including both selector policies
and other declared controls. They are not independent relation ranks,
distinct target solves, or a useful-row yield estimate. [pcc:unverified]

### The existing degree-63 fixture

For the existing p=2147483647, A=847514846, B=2087444724 curve, both
left-selector policies return the same signed sets at degree 63. The actual
signed base has 68 points. Full F has degree 63 and admissible F degree 34.
The following values are copied from the respective compact records, which
were checked inline and reproduced byte-for-byte. [pcc:unverified]

| Target | Degree G | Affine coordinate support | Ordered triples | Affine / identity prefix | Permutation / integer-row groups | Inverse-pair triples |
| --- | ---: | ---: | ---: | --- | --- | ---: |
| Canonical point at x=0 | 68 | 0 | 0 | 0 / 0 | 0 / 0 | 0 |
| Canonical point at x=1 | 67 | 133 | 201 | 133 / 68 | 34 / 1 | 201 |
| Canonical point at x=2 | 68 | 0 | 0 | 0 / 0 | 0 / 0 | 0 |
| Identity O | 34 | 0 | 0 | 0 / 0 | 0 / 0 | 0 |

At x=1, all 201 returned triples contain an inverse point pair. The 68
identity-prefix tuples are absent from the affine coordinate chart and are
supplied by the separate chart. This classifies this finite fixture; it does
not justify discarding the coordinate mask in general. [pcc:unverified]

### Controls that distinguish the obligations

For y²=x³+x over F97, with P=(1,14) and target -P=(1,83), each policy
returns four ordered triples: two affine and two with identity prefix. Exactly
one masked affine triple has no inverse pair. In particular, the returned
(P,P,P) and (-P,-P,P) share the finite coordinate pair (1,0), while
(-P,P,-P) belongs to the identity-prefix chart. [pcc:unverified]

The exact order-five subgroup controls return 13 triples for each of the two
nonzero targets, 12 for O, and zero for the outside target, under both policies.
For the rank-one two-torsion control at x=1, all four triples contain an inverse
pair, but the integer coefficient rows retain coefficients two. Group
cancellation and zero integer rows are distinct notions. [pcc:unverified]

## 4. Saved-reader checks and mutation outcomes

The saved reader checks original producer bytes without invoking the extractor,
norm constructor or quotient solver. It accepted exactly the 42-pair registry,
bound all 132 compact case identities, and independently checked 156 polynomial
identities, 652 grid cells and 596 ordered first pairs. The retained objects
contain 148 ordered triples, including 56 identity-prefix and 12 masked
noncancelling tuples. [pcc:unverified]

The reader uses the independent registry rather than a sidecar-supplied type
label. Exact curve, target ordinate, selector policy, subgroup generator and
order are bound. It reconstructs actual subgroup membership, all base lifts,
the quotient input, all eliminated witness buckets, ordered triples, chart
routes and every grouping backpointer. Typed parsing and value roundtrip also
reject unknown nested fields or omitted explicit fields. With raw records,
both certificate and sidecar hashes and byte lengths must match.

The frozen [control list](../../READER_CONTROLS.md) produced all 91 required
rejections and three positive checks. In bare mode, the omitted (P,P,P)
witness, omitted identity branch, forbidden coordinate-mask filter, and
coordinate-bucket collision collapse each failed the complete ordered-tuple
comparison. These failures therefore did not depend on a stale byte hash.
The opposite subgroup generator was rejected even though its generated group
is the same, because the input binding names a specific generator.
[pcc:unverified]

Every mutation used a fresh in-memory copy. Harmless whitespace in a sidecar
passed in bare mode and failed under exact byte binding, as prescribed.
The reader does not promote arithmetic-call counters into independently priced
field-operation totals. Its compact-counter consistency checks only establish
agreement with the bound retained object.

## 5. Diagonal identity and scope limits

The retained 2,580-byte identity certificate has SHA-256
`78f84d871060866bdfae1deb8509bd3b246516e69b99c99342fed78813dffcc3`
and statement identifier `IDC1hb354e24cbbf6ca6a`. Its reader binds the exact
expression and metadata, checks 32 original and 32 extra fixed public
evaluations over modulus 2305843009213693951, and rejects the coefficient and
digest mutations. The public fixed evaluation points do not supply the
random-secret-challenge assumptions of an adversarial probability bound, and
the program expressly makes no such claim. This is not a formal proof.
[pcc:unverified]

The implementation, written completeness argument and finite panel are separate
evidence. This readout supplies no checked Lean theorem, signed PCC execution
verdict, second-host replay, complete end-to-end ECDLP cost, speed ranking,
relation-matrix rank or recovered scalar. The full repository library test is
a separate parent-owned gate; these successful component calls do not mark it
passed. Complete cost accounting and any performance comparison retain their
own frozen protocol, resource admission and reference requirements.

The precise new interface is an exhaustive signed three-summand set, with an
empty result meaningful only after every admitted affine and identity chart
has completed. Missing bytes, invalid input, failed checks or a resource cap
remain errors or incomplete calls, never negative mathematical certificates.
