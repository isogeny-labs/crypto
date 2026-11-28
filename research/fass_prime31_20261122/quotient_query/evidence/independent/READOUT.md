# Independent replay and persisted-certificate readout

Date: 2026-11-23 UTC. This is a separate-role, same-host check using the
producer's immutable binary. The arithmetic checker was written separately
from the producer, but the replay and reader use the same compiled program
and toolchain. These are unsigned finite development observations, not signed
PCC verification, a second-host replication, a formal proof, or a timing
comparison. All result statements retain unverified PCC standing.
[pcc:unverified]

## Completed calls

Both authorized calls completed on the first invocation, with exit status
zero, empty stderr, and unchanged frozen source and executable hashes. Each
had a 180-second deadline, a 2-GiB virtual-memory limit and one configured
worker. No build, source modification, corrected-source retry or resource
increase was made by this reviewer. The separate full-root library gate was
released to the coordinator only after both calls ended.
[pcc:unverified]

| Call | Actual result | Retained evidence |
| --- | --- | --- |
| Same-binary qualifier replay | Completed all 95 declared cases; stdout and saved objects match the producer byte-for-byte | [Replay output](replay1/qualifier.stdout.jsonl), [exit status](replay1/qualifier.exit-status), [comparison status](replay1/comparison-status.txt) |
| Persisted-object reader, `--check producer/proofs producer/qualifier.stdout.jsonl` | `passed=true`; 27 certificates, 20 S3 inputs, 111 polynomial identities, 275 grid cells; producer solver not invoked | [Reader output](checker1/checker.stdout.json), [exit status](checker1/checker.exit-status), [reader source](../../native/checker.rs) |

The replay output is exactly 35,156 bytes, with SHA256
`f19a4806299fa117bb24fda0a2da4bf599fe22337ea5732e4dba7763f4d23ef3`.
The reader output has SHA256
`e38f3d7247201022534ac119875b29b16793956a435427b1574229865f3b268c`.
The executable SHA256 before and after both calls is
`b78202639890aedf2b1afb971ec6d6d0186d5f0182be3e6eedfb9b514b94c75c`.
These bindings are in the distinct [replay pre-call](replay1/pre-call-manifest.txt),
[replay post-call](replay1/post-call-manifest.txt),
[reader pre-call](checker1/pre-call-manifest.txt) and
[reader post-call](checker1/post-call-manifest.txt) manifests.

## What the replay reproduces

The qualifier's retained summary reports the following fixed-panel counts;
the reviewer replay is byte-identical to that entire output, not just its
summary. These counts describe executed finite checks, not independent
random samples or a general complexity bound. [pcc:unverified]

| Recorded category | Count |
| --- | ---: |
| All cases | 95 |
| Curve / synthetic cases | 88 / 7 |
| Direct tensor-norm constructions checked | 83 |
| Certificate polynomial identities | 618 |
| Complete-grid cells checked | 24,454 |
| Rejected certificate mutations | 318 |
| Refused invalid inputs | 13 |
| Empty algebraic queries | 41 |
| Sum of affine support over the declared cases | 853 |
| Complete certificates retained | 27 |

Each certificate is checked inline by the separately written
[independent arithmetic](../../native/independent.rs). That code checks
prime-field admission; canonical, monic, split and squarefree selectors;
complete pairwise-coprime right partitions; both divisibility identities;
the Bezout identity; and the resulting support count and empty verdict. Its
direct grid comparison checks the root inventories and agreement between
the certificate and H at every grid point. All 88 curve cases additionally
invoke its original-S3 input binding. The grid is a correctness reference;
the candidate query solver does not consume the enumerated root lists.

The 318 mutations are certificate controls on the predetermined persisted
subset: 12 classes on each of 26 cases with nonempty right partition, and
six unconditional classes on the G=1 case. The separate 13 input refusal
controls are not 13 certificate mutation classes. The source-level class
review remains in [MATH_REVIEW.md](MATH_REVIEW.md).
[pcc:unverified]

Only 83 of the 88 curve cases attempted the new direct tensor construction.
The five remaining degree-63 curve cases use the prior dense H construction
and emit `norm=null`. Both kinds still run the quotient-query identity and
grid checks. This was fixed in the [protocol](../../PROTOCOL.md) before calls;
the replay did not change that panel.

## What the saved-proof reader establishes

The reader separately opens the 27 producer certificates and their 27 input
sidecars. It checks certificate identities and complete grids without
invoking the producer's dynamic Euclidean solver or norm construction. Its
20 S3 sidecars additionally pass curve, selector, right-message, liftability
policy and direct H input checks, including the supplied public target
ordinate. The remaining seven are the declared synthetic algebra inputs.
[pcc:unverified]

With the producer JSONL supplied, the reader checks the exact persisted case
set, certificate SHA256 and byte size, and the recorded dimensions, support,
empty verdict and identity/grid counts. Its output reports
`aggregate_certificate_binding=true` and `raw_case_records=95`. The latter
is a count of parsed case records; it does not mean the reader checked all
95 complete certificates. It checked the 27 available objects. The other 68
objects were checked in memory during each qualifier call and have hashes
and sizes in the raw output, but their full bytes were not retained.

Input sidecar bytes were not separately hashed in the producer raw JSONL.
The reader explicitly reports
`input_sidecar_bytes_previously_hashed_in_raw=false` and emits a fresh hash
for each observed sidecar. Input checking follows the sidecar's declared
kind and parameters; the same-binary replay independently reproduces those
exact sidecar bytes. This is not a claim that a signed prior commitment to
all sidecar bytes existed. [pcc:unverified]

The direct directory comparison found equality for every one of the 27
certificate files and 27 sidecars. Both sorted 54-file hash inventories have
digest `30b8233a10ebbeb43c3ee3c4d4e854bd0020b5d87792d0ea1965e3b14456d5ef`:
[producer objects](replay1/producer-proof-files-sha256.txt) and
[replayed objects](replay1/replay-proof-files-sha256.txt).

## Degree-63, X=1 prediction check

The frozen [prediction](../../CANCELLATION_PREDICTION.md) has SHA256
`0250383b885c4aaa6a6e3a40f3d4dbe34f49d40adb84f113217b11d27d034d6e`.
It predicts aggregate degree masses from the earlier observed support and
the written selector-target lemma. This is a check of that particular
previously studied fixture, not a new random-target validation.

The retained replay rows report the following for the fixed Prime31 curve,
selector degree 63 and target abscissa X=1. [pcc:unverified]

| Left selector | B | e | Emitted blocks | Grid cells | Affine support | Serialized certificate bytes |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| All selector roots | 63 | 67 | 2 | 4,221 | 133 | 218,952 |
| Rationally liftable selector roots | 34 | 67 | 2 | 2,278 | 133 | 117,233 |

Both rows contain `degree_mass=[[1,1],[2,66]]` and
`degree1_t=[1934056707]`. Thus the sum of right-modulus degrees on gcd-degree
one blocks is 1, and the corresponding sum on gcd-degree two blocks is 66.
The degree-product count is exactly 1+2*66=133. The actual algorithm happened
to emit two blocks; the prediction required the aggregate degrees, not that
particular block count.

Both masks contain `zeros=133`, `row=67`, `pair=67`, `union=133`, `other=0`.
Here `row` means x=X, and `pair` means S3(x,X,t)=0. Their intersection has one
coordinate pair, and their union covers the complete observed support. No
support disappears when the left selector is restricted to its liftable
roots in this fixture. [pcc:unverified]

The unique one-point fibre is the returned t=1934056707. Its identification
with x(2Q) follows from the reviewed lemma: under X in the full selector and
Y nonzero, the diagonal S3 polynomial has that unique root, and every other
right fibre must have at least two distinct x roots. The identification is
a deduction from that statement and the complete fibre count; no separately
instrumented point-doubling assertion was executed by this reviewer.

Both degree-63/X1 rows have `persist=false` and `norm=null`. Their complete
certificate bytes are therefore outside the persisted reader's 27-object
scope. They were reconstructed and checked inline by the frozen replay.
Their degree and mask diagnostics are reproduced source-bound outputs.

The mask says nothing by itself about useful signed relation yield. Distinct
eliminated-coordinate witnesses and sign choices may project to the same
pair. The example Q=3P in the prediction demonstrates that a pair admitting
a cancellation explanation can also admit P+P+P=Q. No pair is discarded and
no useful-row count is inferred from `other=0` here.

## Source custody and its exact limitation

The producer's original `source-sha256.txt` lists 13 files and omits
Cargo.lock because its inventory respected ignore rules. That original
record remains unchanged. The copied read-only lockfile was present in the
producer snapshot, and before reviewer replay it compared byte-for-byte
equal to the live template. Its SHA256 is
`5ee78c7054c3e0fbc8a436494525189a3c9e7fa7732bf2eed5e83bbca15961ea`.
The omission and current check are retained in the
[post-producer custody addendum](replay1/post-producer-custody-addendum.txt).

Using `rg --files --hidden --no-ignore`, the reviewer made a complete
14-file inventory before replay and before the persisted-reader call.
Both inventory files have SHA256
`ee30e8b00e41709eb6e0a6d270e2ba2f6be75da57e23310e29ef105bf7559213`.
Every listed file passed the [replay postcheck](replay1/source-postcheck.txt)
and [reader postcheck](checker1/source-postcheck.txt). These checks add a
pre-reviewer binding for the lock; they do not retroactively establish its
producer pre-call hash. The executable itself retains its original producer
pre-call hash, which matched both reviewer calls. [pcc:unverified]

## Integration obligations

The certificate proves an affine polynomial-grid statement at the declared
input. Positive components give monic fibre gcds; empty components exclude
their complete right blocks. The written CRT and reduced lex-basis arguments
are reviewed separately in [MATH_REVIEW.md](MATH_REVIEW.md), without Lean or
kernel verification.

The consuming relation solver still needs complete witness recovery,
rational lifts, compatible signs, subgroup checks, duplicate handling and
group replay. Identity-prefix and identity-target charts remain separate
from this affine query. This readout does not convert coordinate support,
an empty affine grid, operation counters or serialized proof sizes into a
complete signed-relation, scalar-recovery or performance conclusion.
