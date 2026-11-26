# Dickson selectors with quadratic S3 coupling — work plan

Date: 2026-11-22 (America/Los_Angeles). Implementation: [PR #4946](https://github.com/isogeny-labs/crypto/pull/4946), following [tracking PR #4940](https://github.com/isogeny-labs/crypto/pull/4940), which was independently merged while the implementation was being prepared. Tracking commit: `e95645aede904b21421683d917b29b622140da1e`; reviewed base: `cac32640c0897b5e69701cce57665407567fd61c`.

## Objective and scope

Implement and qualify three Rust representations of the same affine point-decomposition problem over `p = 2147483647`: ordinary quartic S3 coupling, a quadratic symmetric-invariant lift (Q), and a quadratic chord-line lift (L). Reuse production Dickson, finite-field, curve, and F4 code. Investigate whether preserving the selector ladders and changing the coupling reduces complete relation-extraction work.

The broader objective remains a nonenumerative four-summand extractor producing signed, independent public relations for the complete prime-field index-calculus solver. A constructor, local identity check, or bounded solver result is reported at its own scope. Complete-cost comparisons with the strongest applicable index calculus and optimized negation-map rho remain later obligations.

This document proposes a scientific comparison. It is **not an execution receipt, spending authorization, or an admitted experiment protocol**. Local source-bound unit tests remain development work under AGENTS §9a. Comparative sweeps wait for the published source, canonical protocol, checkpoint gate, and live EKS admission.

## Sources and prior work

- [`pkm_semaev.rs`](../../src/cryptanalysis/ic_framework/pkm_semaev.rs): reuse `dickson_abscissae`, `dickson_factor_base`, and the existing `Membership::Dickson` meaning. Do not create a second selector convention.
- [`symmetrized_semaev.rs`](../../src/cryptanalysis/symmetrized_semaev.rs): existing symmetric-polynomial construction and `eval_s3_u64` provide the original S3 reference.
- [`f4_fp.rs`](../../src/cryptanalysis/f4_fp.rs): reuse the native solver and its visitation/stop mechanisms; unrestricted auxiliary variables belong to the full base field.
- [`pkm_dickson_torus_20261015`](../pkm_dickson_torus_20261015/README.md): historical m=2/3/4 tower experiments already exist. That report records equal torus/split solving degrees in its measured cells. It motivates a coupling comparison, not a claim that Dickson towers were untried. Its historical execution and validation standing are not promoted here. [pcc:unverified]
- [`prime31_wz_rho_frontier.rs`](../../examples/prime31_wz_rho_frontier.rs): exact inherited curve, recorded subgroup parameter, and generator below.
- Merged [PR #4928](https://github.com/isogeny-labs/crypto/pull/4928), read at published head `fc184b03d47e13ec922c9b489f3b3f8a976a8bd9` (merge `2051d2f5b2c0a717deb5d47b8bdd7599264689e0`): the [iteration-six mathematical note](https://github.com/isogeny-labs/crypto/blob/fc184b03d47e13ec922c9b489f3b3f8a976a8bd9/research/prime31_summation_compression_20261122/iteration6/MATHEMATICAL_NOTE.md) already gives rational S3-tree purity, an all-node quadratic oriented lift, and free-order affine-chain completeness on this same prime and curve. The [supplied-chain verifier](https://github.com/isogeny-labs/crypto/blob/fc184b03d47e13ec922c9b489f3b3f8a976a8bd9/examples/prime31_rational_chain_guard.rs) implements top-down signed recovery and fixed correctness controls; it does not discover target decompositions. These results have the published unverified PCC standing. [pcc:unverified]
- The same published tree contains the older [native pair solver](https://github.com/isogeny-labs/crypto/blob/fc184b03d47e13ec922c9b489f3b3f8a976a8bd9/research/prime31_ic_20261115/native/lib.rs): `decompose` uses tower F4, finite quadratic-tower root branching, signed point replay, and explicit resource-limit outcomes; `solve` carries the relation/rank/scalar-recovery path. Its m=2 extraction already exists. The [oriented m8 example](https://github.com/isogeny-labs/crypto/blob/fc184b03d47e13ec922c9b489f3b3f8a976a8bd9/research/prime31_ic_20261115/native/examples/prime31_m8_oriented_f4.rs) is a mixed S3/S4 input/resource gate, with a basis-only outcome rather than a complete root extractor. Neither is a new mechanism introduced here. [pcc:unverified]
- The [iteration-seven fixed-curve density theorem](https://github.com/isogeny-labs/crypto/blob/fc184b03d47e13ec922c9b489f3b3f8a976a8bd9/research/prime31_summation_compression_20261122/iteration7/FIXED_CURVE_FIBRE_DENSITY.md) concerns a selectable regular Dickson fibre, with explicit size and setup restrictions. It supplies no density guarantee for this plan's fixed zero fibre; see the scope below. [pcc:unverified]
- [Semaev, 2004](https://eprint.iacr.org/2004/031): original summation-polynomial construction; formula also inspected in [Galbraith–Menezes](https://cacr.uwaterloo.ca/techreports/2005/cacr2005-02.pdf), §3.2. [Kosters–Yeo](https://arxiv.org/abs/1503.08001): first fall degree does not justify identifying it with the subsequent solving degree.
- Sources checked (KB): no exposed KB tools; the repository wrapper was unavailable in this runtime. Fallback is the named source files, historical study, primary literature, and direct reads of the published #4928 files at the immutable head above. This does not establish exhaustive novelty coverage. [pcc:unverified]

The sibling shared-S3 lane, PR #4922, remains separate; its source and frozen evidence are untouched.

## Fixed instance and domain

| Field | Frozen value / treatment |
|---|---|
| Curve model | `Y^2 = X^3 + A X + B` over `Fp` |
| p | `2147483647` |
| A, B | `847514846`, `2087444724` |
| Recorded subgroup parameter n | `2147446831`, inherited from the source fixture |
| Generator G | `(625037715, 2100627251)` |
| Identity linkage | Exact fixture identified by source and coefficients; ICV1/EC1 linkage unresolved. An exact coefficient/hex search found no matching model in the base registry. No neighboring model's identifier is substituted. [pcc:unverified] |
| Selector | `D_(2^ell)(x,1)=0`, with `lo=0`, `c_top=0`; use its preimage, not an unrestricted Dickson image |
| Actual point base | Same enumerated selector roots, curve-lift filter, subgroup check, canonical ordinate policy, and digest on all arms |
| Targets | Public synthetic coordinates; any construction witness or known scalar remains checker-only |

Do not infer a proved group order, primality certificate, cofactor, or ordinary status from the copied subgroup parameter. Retain source provenance and actual checks separately. Before an admitted comparison, resolve required registered curve identity or explicitly record the unresolved identity as a gate.

The #4928 density theorem assumes a supplied nonsingular odd-order curve, `p>=4096`, a power of two `4<=q<=sqrt(p)/16`, and `4q | (p+1)`. It gives a selectable regular fibre with at least `q/4` liftable coordinates and accounts for failed sampling trials and setup. It explicitly does not guarantee density for `c=0`. At M31 the largest eligible power of two under this cap is 2048, so the 4096-root zero-selector correctness check is outside that theorem's size range as well as its selectable-fibre contract. The existing native `Base::build` scans seeds and selects a terminal value; its recorded depth-six `N=64, B=31` fibre is not silently substituted for this plan's zero fibre. [pcc:unverified]

## Exact arms and equality criterion

For each triple `(x,y,z)`, define `e1=x+y+z`, `e2=xy+z(x+y)`, `e3=xyz`.

| Arm | Coupling per triple |
|---|---|
| A: quartic | Existing `S3(x,y,z)=0` |
| Q: invariant lift | `r=xy`; `e2=r+z(x+y)`; `e3=rz`; `(e2-A)^2=4(x+y+z)(e3+B)` |
| L: chord lift | `r=xy`; `lambda^2=x+y+z`; `r+z(x+y)=A-2 lambda nu`; `rz+B=nu^2` |

The relation chain uses m−1 blocks and m−2 affine intermediate coordinates; attach the same Dickson ladders to the m factor-base coordinates. Every unknown, including every auxiliary, must be extracted over `Fp`. Restricting an intermediate to the factor base changes the problem and is inadmissible.

Q and quartic encode the same Semaev surface; L carries base-field line witnesses. **Raw solution counts need not match.** Compare canonical signed point relations only after the common actual-base test, curve lifting, internal sign reconciliation, multiplicity deduplication, and replay of the whole relation against the target. Record the raw and filtered counts separately. The horizontal chart `e1=0`, repeated coordinates, zero ordinates, and twist-only S3 roots require explicit controls.

The twist-only distinction is local to an unconstrained S3 triple. Under #4928's rational-tree hypotheses—an odd-order source curve, a finite nonzero rational target, and all chain abscissae in `Fp`—the connected quartic chain already forces rational leaf lifts. Q has the same projected chain solutions. **No additional global M31 twist-filtering gain for L is inferred from the isolated twist-only control.** L also permits independent sign reversals `(lambda,nu)->(-lambda,-nu)` at its nodes; the signed-root oriented formulation below removes that ambiguity. The source curve's hypotheses must be checked before applying the theorem. [pcc:unverified]

An affine chain omits an internal sum equal to O. The first qualifier must detect that condition, report its scope, and test an explicit infinity/cancellation control. It must not silently claim complete decomposition coverage. A later complete extractor needs an infinity branch, a justified alternate ordering, or separately handled cancellation relations.

#4928 supplies the relevant alternate-ordering theorem: in a prime-order group of order n, every nonzero-total signed multiset of `m<=n` nonidentity points has an ordering with no zero prefix. This is completeness for unordered signed multisets with free leaf slots, not every ordered tuple or a fixed sorted order. It does not extend to every balanced tree: `P,P,P,-P` has a cancelling child in each two-pair partition. Reuse that theorem and its controls rather than presenting a new completeness argument. [pcc:unverified]

## Existing smaller oriented formulation — required comparison arm

The published #4928 construction orients each internal node v by its parent point `(x_v,y_v)`, with the signed target fixed at the root. It introduces a slope `lambda_v` and imposes

```text
lambda_v^2 = x_left + x_right + x_v
2 y_v lambda_v = (x_left-x_v)(x_right-x_v) - (3 x_v^2 + A)
y_w = lambda_v (x_w-x_v) - y_v        for each nonroot internal child w.
```

All these equations are quadratic. Leaf ordinates are recovered afterward. On the stated rational-tree domain the augmentation is bijective, and the existing verifier's `recover_node`/`recover` implement the root-oriented recovery followed by group replay. Its use of the actual signed root avoids independent per-node sign choices. The present harness's `lift_abscissae` replay is a bounded correctness check; it does not supersede that prior linear-node recovery method. [pcc:unverified]

For the same m leaves and depth ell, before any optional algebraic-closure field guards:

| Representation | Variables | Equations | Checkpoint |
|---|---:|---:|---|
| Quartic A | `m ell + m - 2` | `m ell + m - 1` | Current constructor |
| Invariant Q / chord L | `m ell + 4m - 5` | `m ell + 4m - 4` | Current constructors |
| #4928 parent-oriented lift | `m ell + 3m - 5` | `m ell + 3m - 4` | Required next comparison arm; not implemented in this checkpoint |

Thus the prior oriented lift uses m fewer variables and m fewer equations than the current Q/L encodings. Lower counts alone are not a solver-cost result. Implement and qualify this fourth arm before the eventual admitted comparison. Use the same zero fibre, target, full-field extraction contract, ordering policy, and cost accounting; directly constructing the older verifier's seed-selected base would change the comparison input. [pcc:unverified]

The implementation contribution at this checkpoint is the explicit matched quartic/Q/L polynomial builder and its production `solve_visit` extraction harness. Its tiny all-field correctness case invokes the solver on equations, rather than accepting supplied chain coordinates as #4928's iteration-six verifier does. This distinction does not make root extraction itself new: the older m=2 native pair solver above already extracts finite-field witnesses. No new all-arity solving bound or priority claim follows.

## Hypotheses and falsification

1. **Representation correctness:** projected accepted signed relations agree on fixtures where the affine chain applies. Any unexplained mismatch falsifies that implementation; stop interpreting its solver figures and preserve the counterexample.
2. **Coupling benefit:** compare Q and L with both A and the prior smaller parent-oriented formulation after extra variables, extraction, rejection, and replay are charged. Lower input degree alone predicts neither lower solving degree nor lower total cost. Omitting the smaller existing formulation would leave the admitted comparison incomplete.
3. **Failure mode:** the additional auxiliaries or base-field extraction may outweigh any smaller Macaulay degree. Preserve capped or regressing arms; a shorter capped trace is not a win.

The isolated square-minus-two ladder has pairwise-coprime leading monomials under a degree-compatible order; coupling can change that structure. Its exponentially large eliminant does not by itself imply a large Gröbner solving degree. This study tests the coupled system rather than either heuristic shortcut. [pcc:unverified]

## Development qualification before scientific execution

- Compile the actual constructor source, with hashes of every imported production module and the lockfile/toolchain. Do not substitute a rewritten arithmetic implementation.
- Test the Dickson ladder against its defining recurrence and existing root builder; test original S3 against Q/L witnesses and independent curve addition.
- Exercise m=2,3,4 constructor degree/variable accounting, round-trip witnesses, rational-chart exceptions, twist rejection, and signed replay.
- Enumerative tiny-field or tiny-base truth tables are **correctness controls**, explicitly labeled. If a candidate solver enumerates a base, uses pair tables, or falls back to coordinate scanning, report that execution path as generic decomposition; it does not satisfy the nonenumerative extractor objective.
- Capture distinct build, write-once pre-call, and post-call manifests. Execute a separately copied immutable qualifier with its hash captured before the call. Keep stdout separate from any native `create_new` raw output.
- Generate/check native `identity.certificate/v1` records for identities used by the report; record actual IDC IDs and keep their evaluated/proved tier explicit. A coefficient-field evaluation is not a Lean proof.

## Proposed first admitted comparison — pending

The small initial grid is a bounded diagnostic, not a replacement for the full requested experiment or an asymptotic test.

| Summands m | Initial selector depths ell | Purpose |
|---|---|---|
| 2 | 1, 2, 3, 4 | Single coupling; grow selector depth |
| 3 | 1, 2 | First coupled intermediate |
| 4 | 1 | Initial four-summand integration and caps |

After the prior parent-oriented formulation is implemented and qualified, schedule all four arms per grid cell on two deterministic public target points and one constructed positive-control target when the actual point base permits one: at most 84 proposed arm slots. Only the original three arms are implemented in this checkpoint; this expanded grid is still pending and is not a claim of an executed fourth arm. An empty base, undefined positive control, failed target construction, rejected subgroup point, timeout, OOM, or solver cap remains an explicit planned-slot outcome. Failures do not suppress later planned arms. Freeze exact seeds, target coordinates, base digests, orders, source SHA, and commands in the canonical protocol before submission.

Use the same fixed variable ordering policy initially, same source build, one solver thread, same resource caps, and the same accepted-relation definition. Include A/A on identical copied binaries if any clock comparison is planned. Keep a planted-positive control separate from the unseen public-target panel. Export identical systems to the strongest available external Gröbner implementation for the later solver-performance comparison; name missing external comparisons explicitly.

Proposed envelope: one EKS job, 2 vCPUs, 8 GiB RAM, a 60-minute total deadline including build and idle time, **US$5 total ceiling**, and **zero automatic retries**. Initial per-arm stops: 10 seconds, degree bound 6, 2,048 matrix rows and 4,096 columns, with a separately bounded extraction/candidate budget. These are proposed caps; the adapter must expose and test every cap, and admission must bind an absolute UTC deadline and exact command. If a cap cannot be enforced, do not submit a job that merely claims it. The live budget, permitted image/build route, canonical protocol, S3 checkpoint, and available capacity must all be established first. No spend is authorized by this paragraph.

Record input/solving degree, matrix shape and fill, counted field operations, reductions, candidate extraction work, raw/accepted/duplicate/dependent relations, replay cost, memory, stop reason, and all unpriced counters. Timings are absent unless the required isolation and A/A controls were admitted. Incomplete arms cannot support a gain. A component improvement requires lower fully charged work per accepted independent relation on matched completed workloads; complete-DLP cost remains unset until measured.

## Delivery and continuation

Publish constructor code, correctness evidence, source manifests, native identity certificates, this plan, the source-linked report, SVG diagram, and visually checked PDF in implementation PR #4946. No existing frozen study is rewritten. Mark each requirement against evidence; local qualification, independent review, PCC signed standing, full-library validation, and scientific admission are separate statuses.

After qualification, implement the smaller published parent-oriented arm and complete the full-domain production visitor integration for all arms, returning signed public relations with explicit fallback classification. Reuse the existing oriented recovery and pair-solver controls at their proper scopes. Then execute the admitted four-arm matched comparison and feed accepted independent relations into complete scalar recovery. Extend depth only through a newly frozen resource/attempt record. A new measurement gets its own immutable evidence directory and required benchmark/dashboard records.
