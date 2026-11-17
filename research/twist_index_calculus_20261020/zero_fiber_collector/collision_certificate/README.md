# Repeated-factor certificates for twist relation search

The [prospective protocol](PROTOCOL.md) tests whether exact low-degree
ideal certificates can remove zero-fiber systems before full root finding.
The factor base, canonical pairs and relation definition stay fixed. Phi
and doubling receive the same algorithm. This is a candidate improvement
to relation-search cost; no production benefit is established yet.

## Checked construction

For the peeled abscissa `a`, the cubic
`q_a = a^3-a^2 e1+a e2-e3` evaluates to
`(a-x1)(a-x2)(a-x3)`. A checked polynomial identity
`q_a^k = sum(h_i f_i)` forces every common solution to repeat that peeled
factor. The existing decoder requires distinct factor abscissae. The
certificate is a sufficient rejection condition, not an assumption that
every system has this property.

The Rust implementation searches total-degree Macaulay spaces D=4..8 and
powers k=1..3. Sparse polynomial expansion checks the identity separately
from the dense elimination. A separate input pack excludes prior roots,
relation hits, original solver costs, logs and target scalars. Post-run
auditing binds the original public indexing and polynomial inputs, checks
every certified identity and retained field root, and rejects any attempt
to discard a system with an accepted physical relation.

## Verified and pending work

| Requirement | Evidence/status |
|---|---|
| Native positive, negative, mutation and schema controls | Gate v3: 5 tests pass, 0 fail; build exit 0 |
| Conditional algebraic rejection proof | Three Lean lemmas check; [proof map](proof/PROOF_MAP.md) names the missing native/field/group bridges |
| Universal cubic identity certificate | [IDC1h10810f17e0f6c8f6](proof/identity.json), 64 derived plus 64 extra points; false identity refused |
| All 42 exact input packs | Preparation v2 exit 0; [registered workload](workload-v1.json), source hashes in `workload-v1.sha256` |
| Full matched production | 42 case-arms, first plus repeat; launcher controls pass; [freeze](FREEZE.json) precedes launch |
| Actual integrated target recovery/rank/cold price | Pending; the existing 85-configuration panel has its original separate owner |
| Admitted timing, strongest IC/rho/external controls, calibrated GAE | Unresolved; no promotion inferred |

Native executable SHA-256:
`4c49d77faa9b5ef3fd54459bd0ac0cf69ee077eef43ec84361753f7dbc3555f5`.
Workload SHA-256:
`17ecb4c302526bc58e592466071e1c2704eaaf43a0a0ca1efe563e1bd69b3c10`.

The [implementation report](BUILD_AND_PROOF.typ) and
[inspected two-page PDF](BUILD_AND_PROOF.pdf) record this checkpoint.
The corrected diagram, its SVG/150 dpi PNG and an unlisted benchmark copy
are preserved with the same evidence. The original overflowing layout,
failed proof gate and schema-preparation failure remain in the raw history.

## Execution and interpretation

The [execution supplement](EXECUTION_SUPPLEMENT.md) fixes diagnostic
criteria before production. Preserve [execution history](EXECUTION.md),
all failed attempts, the first/A/A data, independent audits and exact
source/input/workload seals. Each case uses the common reservation, one
worker, 8 GiB, a 1,200-second outer watchdog and a 1,180-second producer
budget. Actual launch receipts are written after the queue wait. An
interrupted attempt is never given a new deadline automatically.

No old F4 solve or public-point census is repeated. This round executes a
new certificate algorithm on retained polynomial inputs. Its partial
multiplication model retains failed-search and sparse-check work and old
F4 row counts for uncertified systems. No fallback F4 call occurs here;
polynomial construction, other solver primitives, memory and I/O remain
unpriced. Any integrated pipeline gain must be measured and checked in its
own complete comparison. The original broader goal remains open.

Raw new directories are `../runs/collision-certificate-*`. A separate
versioned collision-certificate archive and native replay/publication are
pending after this experiment terminates. Existing Phase 1 and parent
archives remain immutable. The earlier Phase 2 archive selection can exclude
this separate round, so it need not wait for this experiment's publication.
