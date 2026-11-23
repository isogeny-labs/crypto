# Signed FASS extraction: fixed development qualification

Prepared on 22 November 2026 Pacific / 23 November UTC, before native
qualification. Tracking: [PR #4976](https://github.com/isogeny-labs/crypto/pull/4976).
Published predecessor: `02d296ba3c80034a4a8a806d627db10d596b96cb`;
continuation tracking commit: `04df4460cdf97c874a16beff7c6fd5d32ab77295`.
The source-freeze manifest fixes the final bytes of this protocol before calls.

## Task and acceptance boundary

Turn the predecessor's positive triangular quotient components into the
complete set of ordered triples `(P1,P2,P3)` satisfying `P1+P2+P3=Q`, for the
declared finite, sign-closed factor base. Repetitions are allowed; the identity
is excluded from the base but is an admitted target. Preserve every eliminated
coordinate and sign witness. Retain separate permutation classes and integer
coefficient rows with all ordered-triple backpointers. Never use the coordinate
cancellation mask as a rejection criterion.

This is a native implementation qualification, not a timing study or a
comparative performance experiment. It extends the existing exact quotient
solver without changing its frozen arithmetic or evidence. Acceptance requires
agreement with an independently coded exhaustive group oracle for every fixed
case, exact input/output binding, direct group replay, and all predeclared
rejection controls. A cap, malformed input, failed proof or missing case is an
error or an incomplete outcome, never an empty result.

Complete scalar recovery, independent relation rank, matched cold costs and
the separate repository-wide library gate remain distinct deliverables. The
component emits a phase ledger with defined counters and explicit unpriced
work; no deterministic diagnostic counter is described as a full cost.

## Sources and ownership

Sources checked (KB): no `kb_status` or `search_knowledge` MCP capability is
exposed. `scripts/research-kb status` reports no executable (exit127). Scoped
fallback: `../quotient_query/{THEORY,PROOF_MAP,REPORT}.md`, its published Rust
kernel/norm/independent checker, `../target_quotient/THEORY.md` and its oracle,
and `src/cryptanalysis/semaev_prime_poly.rs`.

Primary mathematical sources:

- Petit, Kosters and Messeng, [Algebraic approaches for the Elliptic Curve
  Discrete Logarithm Problem over prime fields](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
  sections2.1,2.4,3.1: selector restrictions and the need to turn polynomial
  roots into actual group relations.
- Kosters and Yeo, [Notes on summation polynomials](https://arxiv.org/pdf/1503.08001),
  section2, Proposition2.1: the algebraic-closure characterization does not
  license treating every ground-field abscissa as a rational signed point.
  Semaev's original https://eprint.iacr.org/2004/031.pdf was identified but its
  direct fetch returned403; this accessible primary paper supplies the statement.
- [RFC6090](https://www.rfc-editor.org/rfc/rfc6090.html), section3 and appendixF.1:
  affine group addition, negation and the identity/vertical-tangent branches.
- The classical D5 and regular-chain sources already pinned in the predecessor
  theory describe the reused triangular-query engine. This continuation adds
  extraction and coverage checks, not a new dynamic-evaluation principle.

Conductor is absent; `docs/CONDUCTOR.md` documents the missing integration as
nonblocking. Parent owns this protocol, tracking, publication and call receipts.
The producer owns the new extractor and main; a separate role owns the new
arithmetic oracle and persisted reader; a third role owns the written proof,
proof map and report. No predecessor source or frozen evidence may be edited.

## Exact relation contract

Work on nonsingular `y^2=x^3+A*x+B`, with admitted prime `p=97` or
`p=2147483647`. Coordinates are canonical residues. The selector is
`L(z)=z^d-1`, where positive `d<=128` divides `p-1`. Selectors do not supply
an identity point. The base consists of all rational signed points with
selector abscissae satisfying the configured group policy.

Two group policies are admitted here: the full rational group, or an explicitly
generated finite cyclic subgroup. The latter validates a nonidentity on-curve
generator, its declared first return to the identity, and the complete distinct
inventory before membership lookup. Merely checking `[r]P=O` is not accepted
as membership in an arbitrary specified cyclic subgroup. The Prime31 cases use
the full-group policy; this qualification does not certify their group order.

Use both left-domain policies: full `F=L`, and admissible `F`, the monic product
over the abscissae of the actual rational, group-filtered base. The second
coincides with the predecessor's liftable policy when the group policy is full.
An empty admissible base gives `F=1` and a well-defined empty answer.

For finite `Q=(X,Y)`, use the predecessor's squarefree rational right message
`G`, retaining all selector witness buckets. For each positive quotient cell
`(a,t)`, recover every root of `gcd_z(L(z),S3(a,z,t))` and every right selector
witness of `gcd_w(L(w),S3(t,w,X))`. For every admissible signed lift of `a,z`,
form `R=P1+P2`. Require finite `R` with `x(R)=t`; form the unique `P3=Q-R`;
check exact base/group membership, the right bucket, and the original group sum.
These checks retain the original ordinate Y, including the negative-target cases.

The missing-prefix chart independently emits `(P,-P,Q)` for every base point P
when `Q` is in the base. For identity target Q, take `G` to be the actual base
abscissa polynomial and use the finite-prefix route with `P3=-R`; there is no
identity-prefix output because the identity is excluded from the base.
Two-torsion has one distinct lift, including when a point is its own inverse.

Deduplicate only complete ordered point triples. Then form permutation classes
and integer coefficient rows as separate quotients with explicit backpointers.
The coefficient representative for x uses the smaller rational ordinate; y=0
has a single positive representative. Integer rows are not reduced modulo an
unstated point order, and row equality is not a rank or usefulness test.

## Fixed 132-case registry

All category duplicates are retained. No fixture depends on an observed output.
Every item below uses both left-domain policies.

1. Ordinary small-field fixture `p=97,A=2,B=3`, degrees
   `{1,2,3,4,6,8,12,16}`: the first three non-two-torsion rational points in
   increasing x with the smaller ordinate, plus `(30,0)`. 64cases.
2. Existing fixture `p=2147483647,A=847514846,B=2087444724`, degrees
   `{7,21,63}`, targets at x in `{0,1,2}` with the smaller rational ordinate.
   18cases. Its curve parameters are preserved, without an order assertion.
3. Existing rank-one control `p=97,A=96,B=0,d=2`, targets x in `{0,1,2}`
   with the smaller rational ordinate. Six cases.
4. One identity target for each degree/model in items1–3. 24cases.
5. Negative of the smaller-ordinate x=1 target on item1 at d2 and item2 at d7.
   Four cases. These targets use the guaranteed selector root1 to exercise a nonempty signed
   cancellation branch; the choice precedes all new native calls.
6. Exact cyclic-subgroup control `p=97,A=2,B=3,d=96`, generator `(3,6)` with
   declared first return5; targets `(80,10)`, `(80,87)`, identity, `(0,10)`.
   Eight cases. The last target is permitted outside the configured subgroup;
   the exact relation set must then be empty.
7. Deliberately exceptional order-four control `p=97,A=1,B=0,d=1`, full-group
   policy, targets `(1,14)`, `(1,83)`, identity, `(0,0)`. Eight cases. If
   `P=(1,14)`, the same finite-prefix cell `(a,t)=(1,0)` for `Q=-P` must retain
   both `(P,P,P)` and `(-P,-P,P)`. The separate triple `(-P,P,-P)` belongs to
   the identity-prefix chart. This control checks a proposed mask rejection
   directly; it is not a proxy for the ordinary Prime31 fixture.

The independent reader defines the registry separately and binds both target
coordinates, curve, degree, domain and group policies, and subgroup generator
and order. It never dispatches verification from a producer-supplied type label.

## Construction and checks

Reuse the unchanged tensor norm for d<=21; for d63 and d96 use the unchanged
prior direct selector norm, followed by quotient reduction. Record the chosen
construction explicitly. This is not a fresh comparison of the two norm methods.
Consume and check the quotient certificate rather than trusting a producer
support-count field. The independent signed oracle uses its own finite-field
and affine-group arithmetic, with exhaustive ordered first pairs from its own base
inventory and the uniquely determined third point. It does not invoke the producer's S3, norm or query solver to decide
the expected relation set.

Compare every ordered triple and every group of backpointers, rather than only
counts or a producer digest. Every accepted signed witness is replayed against
the original target. The independently specified reader mutation panel is frozen
in `READER_CONTROLS.md` before the first build:43 semantic mutations in both
raw-bound and bare reading modes, five further raw-only mutations (91 required
rejections), and three positive controls. It must exercise omitted identity
and finite witnesses, missing collision backpointers, altered sign/target,
two-torsion duplication, subgroup metadata, malformed point/base inventories,
missing/extra/duplicate cases, false empty claims, changed bytes, and wrong
fixture labels. Each control operates on a copy, leaving retained outputs intact.

The producer also exercises16 fixed input refusals: unsupported/invalid prime,
noncanonical A/B, singular curve, zero/nondivisor/over-limit degree,
noncanonical target x/y, off-curve target, nonminimal/wrong/unit/over-limit
subgroup order, and off-curve subgroup generator. Their exact named inputs
are fixed in source. Admitted selector degree is at most128; explicit subgroup
order is between2 and512 inclusive. No refusal invokes an empty verdict.

## Retained objects and byte limits

Serialize the complete case result in memory with a fixed32MiB limit. Every
case record binds that complete serialization and its input sidecar by SHA256
and byte count. Save complete results and sidecars only for this fixed subset:

- Ordinary97 d2, including identity and negative target:12.
- Rank-one97 d2, including identity:8.
- Every order-four control:8.
- The four subgroup controls with admissible F:4.
- Prime31 d7, including identity and negative target:10.

Total42 result/input pairs. The other90 cases are compared in memory and retain
count/digest records, not a claim that their complete objects were published.
Per persisted result and per sidecar, the predeclared limit is64KiB. Compact
JSON uses a documented schema. Do not change the persistence selection, split
large records or compress data to evade policy after observing sizes. An
oversized artifact follows the repository store policy or is a named publication
blocker. All native calls refuse to overwrite output directories or files.

## Evaluated diagonal-identity supplement

A separate binary includes the unchanged repository identity-certificate
module. It issues one `identity.certificate/v1` object for the unconditional
polynomial identity `S3(a,z,a)=(3*a*a+A)^2-4*(a^3+A*a+B)*(z+2*a)`, with
variables a,z,A,B and syntactic total-degree bound4. Its certificate uses32
fixed derived evaluation points over the module's fixed field2^61-1, a public
seed and `proof:null`. A separate-role call checks the saved statement at32
additional fixed derived points and rejects altered-statement and altered-digest
controls. Exact seeds and API calls are source-pinned before execution. These
are finite evaluations, not a symbolic or Lean proof, and no adversarial
false-acceptance probability is asserted from public deterministic seeds.
The nonsingularity implication and extraction bound remain written arguments.
Its output is separate from the42 signed-result/input pairs.

## Custody, resources and attempt allowance

Before building, snapshot every transitive compiled source, Cargo manifest and
lockfile plus the frozen protocol/control registry. Include ignored lockfiles
explicitly, not through a default `rg --files` inventory. Record SHA256 and byte
counts. Publish the exact source/protocol commit before qualifier execution.
Build only that snapshot, then copy the executable to a separate immutable path.
Use the thin `run_bounded.sh` wrapper with distinct write-once build, pre-call
and post-call manifests. Before the first build, exercise its transport with
the native `/usr/bin/false` followed by `/usr/bin/true`, each in a separate
receipt under10seconds/2GiB, requiring exit1 then0 and unchanged inputs.
This tests error propagation and continuation without running arithmetic. Retain exact
commands, host/toolchain, stdout/stderr and exit status, and check all source and
binary hashes after every call. Do not retrofit a missing pre-call hash.

- Build:240seconds,2GiB virtual memory, one Cargo job, offline locked inputs.
- Qualifier:300seconds,2GiB virtual memory, one process/thread.
- Separate-role replay, saved-output reader and controls: each300seconds,
  2GiB virtual memory, one process/thread, sequentially after the producer.
- Diagonal-identity issue and separate-role readback: each60seconds,2GiB,
  one process/thread, separately recorded from the signed extraction calls.
- At most one corrected-source attempt after a diagnosed implementation defect,
  with a correction note, fresh snapshot, and the same panel and limits. Preserve
  the first attempt. A second failure ends this qualification attempt.
- One additional source-bound root-library development gate may follow this
  new component:600seconds,6GiB virtual memory, one job/thread, exact locked
  source inputs and the preserved previous build cache. It is separate from the
  old timed-out call and cannot be called passed unless tests actually complete.
  Materialize tracked dependencies before it; no resource increase or retry.

There is no paid cloud or performance experiment in this protocol. Local builds
and fixed component correctness checks are development work under AGENTS§9a.
Future competitive measurements retain the EKS, checkpoint, resource/admission,
matched-reference and complete-accounting requirements; they are not silently
replaced by this local qualification.

## Publication and evidence status

Publish the Rust implementation, frozen inputs and calls, failed attempts if
any, separate-role readout, written completeness argument and proof map,
phase-accounting contract, source-linked report, diagram and inspected PDF.
Register all result claims through PCC with unverified standing unless the
existing independent signed verifier changes it. No Lean executable is currently
available; a written proof or a finite native check is not a checked Lean proof.
Performance dashboards remain unchanged because this task publishes no admitted
comparative speed or scalar-recovery result. Mark the existing PR ready once the
component is finished and checked, preserving the distinct unresolved gates.
