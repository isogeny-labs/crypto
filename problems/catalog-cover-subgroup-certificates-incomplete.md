# Catalog cover-subgroup certificates are incomplete

- **Status:** `open`
- **Kind:** proof gap, implementation gap, and independent-verification gap
- **Affected work:** all 664 EC1/map bindings in the [content-addressed cover graph](../docs/curves/cover-links.yaml), the [ICV1/EC1 registry](../docs/curves/registry.json), and the [catalog-wide transfer assessment](../docs/curves/cover-transfer-assessments.json)
- **Next action owner:** curve identity and transfer certification workstream (unassigned)
- **Last updated:** 2026-10-14

## Problem

We cannot promote catalog-wide subgroup preservation, image-order, or recovery
claims because only the exact order-29 binary fixture currently has the full
narrow certificate. The remaining 663 EC1/map bindings have exact numeric
catalog tuples but lack proof-grade, map-bound evidence for the generator's
exact source order, kernel intersection, image order, and recovery.

All 664 declared subgroup orders are numerically coprime to their cover
degrees: 530 degree-two bindings have odd declared order and 134 degree-three
bindings have order not divisible by three. That calculation supplies a
necessary conditional input, not the missing exact-order or induced-map
certificate. The catalog-wide typed checks now present for all 134 binary
generators exercise the bounded generator path, but they do not turn
the other 133 binary numeric order declarations into proof-grade exact-order
certificates and do not provide an arbitrary-class recovery path.

## Evidence and uncertainty

- **Observed:** [`docs/curves/registry.json`](../docs/curves/registry.json)
  contains 664 EC1 representations on 659 curve models; five models have two
  distinct representations. [`cover-links.yaml`](../docs/curves/cover-links.yaml)
  binds each representation to its exact CV1 map, but every generated
  `subgroup_transport` value is `not_tested`. The catalog-wide integer check
  finds `gcd(2,r)=1` for all 530 degree-two bindings and `gcd(3,r)=1` for all
  134 degree-three bindings. The existing detailed evidence supports only the
  order-29 record on its typed point-class domain.
- **Derived or proved:** if a subgroup really has exact order `r`, checked
  homomorphisms satisfy `phi_* phi^* = [d]` on it, and `gcd(d,r)=1`, then
  pullback is injective on that subgroup and multiplication by `d^-1 mod r`
  recovers the source class. The antecedents, especially exact source order
  and an executable composition map on the relevant domain, are not supplied
  merely by the catalog tuple or gcd calculation. The native typed-path checks
  have no `identity.certificate/v1` / `IDC1h…` record, so they are not promoted
  to a proof-grade universal algebraic identity.
- **Suspected explanation:** none. Evidence coverage, not an unexplained
  arithmetic failure, is the known gap.
- **Missing capability:** proof-grade primality/factor and exact generator
  order evidence, plus map-bound pullback image, kernel, image-order, and
  recovery certificates for every EC1 representation. The 530 prime-family
  bindings additionally depend on the balanced-Jacobian problem; arbitrary
  binary-result recovery additionally depends on the general-norm problem.
- **What this evidence does not establish:** missing certificates do not show
  that a selected subgroup fails to transfer. Conversely, catalog labels,
  coprimality, a conditional group-theoretic lemma, or generated point-class
  checks alone do not establish operational transfer from arbitrary Jacobian
  output or any speedup.

## What would unblock it

- [ ] For each of the remaining 663 exact `(map_uid, EC1, curve_uid)` tuples,
  issue a replayable source certificate: declared-field validation, generator
  membership and nonidentity, proof-grade factor/primality evidence for `r`,
  and exact-order checks sufficient for the actual order structure.
- [ ] Construct the exact pullback image, validate its divisor class, prove its
  image order and kernel intersection, and bind every result to the full map,
  curve, field, generator, and subgroup identities rather than to a bit size or
  model alias.
- [ ] Check the composition and `d^-1 mod r` recovery identity on the public
  supported domain, including infinity and ramified fibers; keep recovery from
  arbitrary Jacobian output unknown until the relevant general norm exists.
- [ ] Issue and replay the required `identity.certificate/v1` record, or attach
  a stronger independently checked formal proof, for every universal
  `phi_* phi^* = [d]` statement used by a subgroup certificate.
- [ ] Add independent replays and negative controls for altered generators,
  orders, fields, map UIDs, coordinate conversions, and the five models with
  multiple EC1 representations. Extension-field coordinates must remain their
  exact coefficient arrays rather than being coerced to scalar strings.

These conditions are sufficient for proof-grade catalog subgroup certificates
only where the underlying induced-map implementation is also supported. They
do not provide the separate Jacobian relation solver or any end-to-end cost
comparison.

## Next discriminating step

- **Action or proof obligation:** complete proof-grade source-order evidence
  for the 133 binary EC1 records beyond the existing order-29 control, then
  combine it with the generated typed pullback/composition checks without
  upgrading the arbitrary-norm claim.
- **Inputs and controls:** use every exact binary EC1 tuple from the graph;
  retain the order-29 fixture as a positive control and include a changed
  generator, changed `r`, divisible-by-three synthetic order, duplicate/missing
  representation, and a two-representation curve as negative controls.
- **Execution limits and stop condition:** verify public catalog data only; do
  not search for discrete logarithms. Stop a batch on the first invalid tuple
  and retain the exact failing identity and reason rather than silently
  downgrading or skipping it.
- **Outcomes and next actions:** complete binary certificates narrow the open
  population to the 530 prime-family bindings; a failed exact-order proof
  leaves that tuple unknown or invalid with preserved evidence; a typed-map
  failure blocks its transfer claim; an infrastructure failure remains a
  tooling blocker. Prime-family work proceeds only after a compatible induced
  map exists.
- **Recheck trigger:** new exact-order proofs, completed catalog binary
  certificates, or a balanced prime-cover transfer implementation.

## 2026-10-27: live geometry and dated assessment scopes

The live registry now has 20,592 map rows and 994 EC1 representations. Native
geometry regeneration preserves every old map and adds 4,801 rows; the additions
include six binary and nine extension-field EC1 representations. Geometry
verification supplies no missing subgroup-transfer certificate.

The separately dated transfer assessment explicitly binds catalog commit
`728304718cb0ab6df70da620a908a5f76f175293`: 15,791 maps, 15,789 covers, 557 fields
and 979 EC1 representations. Its original profiles, memberships, supported and
unknown statuses, and independent semantic digests remain unchanged. The
4,801 new maps and 15 new EC1 representations fall outside that assessment;
they inherit no old transfer-support result.

The next coverage task is to bind a new assessment to the enlarged registry,
enumerate those exact new map and subgroup identities, and obtain or explicitly
leave unknown each required source-order, induced-map, image/kernel and recovery
receipt. Existing certificates must retain their exact identity scope. Merely
copying the earlier profile counts or relabelling geometry as transfer evidence
does not satisfy this obligation. The historical counts below remain their
original dated observations.

## Resolution and history

Unresolved. Classifying every tuple, including an explicit `unknown`, fulfills
the assessment inventory but does not supply the certificates required to
resolve this problem.

- 2026-10-26 (additive): [`transfer-audit`](../research/transfer_audit_20261011/RESULTS.md) executes the
  generator path of the binary genus-3 cover for four registered curves (sect113r1, ECC2K-130, K-163,
  B-163) and four toy Koblitz curves: `phi^*(G)` has exact order `r` in `Jac(H)(F_q)` by Cantor
  arithmetic (`[r]phi^*(G) = O`, `phi^*(G) != O`, `r` prime), with additivity on 4 sample pairs,
  scalar equivariance on 3 scalars and the `[3]` push-pull witness
  (`certificates/*/cover_binary_g3_deg3.json`, status `verified`). This is image-order and kernel
  evidence for those four EC1 generators only; the order of `<G>` is the registry's claimed order
  validated by `[r]G = O`, primality of `r` and two annihilated random points, not a point-count
  proof; the arbitrary-class norm and recovery remain open, and the 129 other binary and all 530
  prime bindings are untouched. Still unresolved.

