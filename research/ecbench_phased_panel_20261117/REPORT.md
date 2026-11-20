---
title: "A phased curve panel for ecbench"
subtitle: "20-, 30-, 60-, and 80-bit subgroup gates"
author: "Isogeny Labs research planning"
date: "2026-11-17; lane revision 2026-11-20"
geometry: margin=0.72in
colorlinks: true
linkcolor: blue
urlcolor: blue
header-includes: |
  \usepackage{seqsplit}
---

**Evidence state:** planned and unmeasured. This report selects curves and gates;
it does not report an experiment, an attack, or an ECDLP speedup.
[pcc:unverified]

## Decision

Use subgroup size `b = ceil(log2(r))`, not field size, to define four phases:
18–23, 27–33, 56–62, and 76–84 bits around nominal 20, 30, 60, and 80 bits.
Phase 1 is deliberately wider than a three-curve panel: it contains seven cells
so the study can identify Koblitz sign, cofactor, binary field structure, and
prime-curve automorphism effects before those effects are confounded with
scaling. Phases 2–4 retain one Koblitz, one general-binary, and one prime-field
cell.

The exact identities, parameters, and current execution states are frozen in
[`panel.json`](panel.json). Promotion is a scientific-readiness decision. The
separate end-to-end claim gate in [`PROTOCOL.md`](PROTOCOL.md) is required for
any speedup statement. [pcc:unverified]

The curve ladder is crossed with four independently promoted lanes. Algebraic
decomposition and generic table/root-finding decomposition remain separate
tracks. Direct hyperelliptic/Jacobian work is a third lane, and a typed
cover/transfer plus destination-solver and source-recovery pipeline is a fourth.
Phase 1 now pins one exact direct Jacobian cell and one exact typed cover route.
The direct entry is an 18-bit prime subgroup of a genus-2 Jacobian with
same-Jacobian negation rho and hyperelliptic index calculus. The transfer entry
is a degree-three same-field pullback from `P1-K0-23` to a genus-3 Jacobian
image with source rho, generic destination rho/BSGS controls, a
pushforward-projected characteristic-two hyperelliptic relation candidate,
source replay, and nine-phase cold wall accounting. Phases 2–4 remain blocked. These are executable
preregistrations, not completed measurements or speedup results [12–15].

| Lane | P1 | P2 | P3 | P4 |
|---|---|---|---|---|
| elliptic algebraic | ready | ready | partially blocked | blocked |
| elliptic generic decomposition | ready | ready | partially blocked | blocked |
| hyperelliptic/Jacobian | ready | blocked | blocked | blocked |
| cover/transfer | ready | blocked | blocked | blocked |

![Four phased panels separated by admission, correctness, rank, replay, and resource gates. The speedup-claim gate is a separate branch. [pcc:unverified]](phased-panel.svg)

## What “representative” means here

The panel is representative by *structural trait*, not by security strength or
identical standardized model. SEC 2 distinguishes prime-field random curves,
binary Koblitz curves, and binary verifiably-random curves; its binary Koblitz
families exercise `a=0` and `a=1`, while its standardized binary examples use
small cofactors [1]. NIST SP 800-186 likewise records Koblitz and pseudorandom
binary families and cofactors 1, 2, 4, and 8, while deprecating binary curves for
new applications [2]. RFC 7748’s Curve25519 is a widely deployed prime-field
Montgomery curve with cofactor 8 [3]. The ecbench index-calculus paths in scope
operate on Weierstrass/Koblitz representations, so this panel exercises
cofactor-8 handling but does **not** claim to reproduce Montgomery-ladder or
Edwards-form performance.

The selected traits are:

- prime fields and binary extension fields;
- Koblitz `a=0` and `a=1`, including the Frobenius action;
- general binary curves over prime- and composite-degree ambient fields;
- cofactors 1, 2, 4, and 8, plus deliberately labeled high-cofactor stress
  cells;
- generic prime curves, an `a=-3` prime curve, and `j=0` / `j=1728`
  automorphism sentinels; and
- registered repository curves at small and intermediate sizes, followed by
  exact Certicom ECC2-79 and ECCp-79 challenge anchors at the final size [4].

No small curve is a secure deployment recommendation. The 79-bit challenge
curves are historical public exercises, not present-day security targets and
not evidence that this study’s methods solve them faster. [pcc:unverified]

## Selected cells

| Cell | `b` | Field | `h` | Role and distinguishing trait | Current ecbench state |
|---|---:|---:|---:|---|---|
| P1-K0-23 | 21 | `GF(2^23)` | 4 | core Koblitz `a=0`, Frobenius | runnable narrow |
| P1-K1-23 | 23 | `GF(2^23)` | 2 | other Koblitz sign | runnable narrow |
| P1-B21 | 20 | `GF(2^21)` | 2 | general binary, prime extension degree | runnable narrow |
| P1-B24 | 22 | `GF(2^24)` | 8 | composite-degree ambient field | runnable narrow |
| P1-P20 | 20 | 20-bit prime | 1 | generic short Weierstrass | runnable narrow |
| P1-P25-J0 | 21 | 25-bit prime | 12 | `j=0`, order-6 automorphism sentinel | runnable narrow |
| P1-P28-J1728 | 23 | 28-bit prime | 50 | `j=1728`, order-4 automorphism sentinel | runnable narrow |
| P1-JG2-18 | 18 | `F_769`, genus-2 Jacobian | 3 | direct higher-genus destination | runnable narrow |
| P2-KOBLITZ-37 | 28 | `GF(2^37)` | 596 | Koblitz high-cofactor stress | runnable narrow |
| P2-B27 | 27 | `GF(2^27)` | 2 | general binary | runnable narrow |
| P2-P32a | 32 | 32-bit prime | 1 | standard-panel `a=-3` prime | runnable narrow |
| P3-KOBLITZ-73 | 57 | `GF(2^73)` | 109796 | wide Koblitz stress | rho/claw reference only |
| P3-B59 | 59 | `GF(2^59)` | 2 | general binary at u64 boundary | runnable narrow |
| P3-P61-J0 | 61 | 61-bit prime | 1 | `j=0` at u64 boundary | runnable narrow |
| P4-KOBLITZ-83 | 82 | `GF(2^83)` | 4 | registered confidence gate | rho/claw reference only |
| P4-ECC2-79 | 79 | `GF(2^79)` | 2 | Certicom general-binary anchor | import/backend blocked |
| P4-ECCp-79 | 79 | 79-bit prime | 1 | Certicom prime anchor | import/backend blocked |

Here `b` is subgroup bits and `h` is cofactor. Registry identities and proved
statuses come from the committed curve registry and traits catalogue [5, 6].
The two external anchors use the exact Certicom parameters mirrored at pinned
`isogeny-labs/ecc-challenges` commit
`efd49aa0a802cda679748533ec5bc5e44ab9ea71`, blob
`a82dc0ee00a6d25bbfc09f260a4e5430a2a52f9e` [7]. They deliberately have no
local ICV1 or EC1 yet. [pcc:unverified]

The Phase 1 transfer route is not another source cell. It maps the exact
`P1-K0-23` order-`2095853` subgroup through
`phi(u,v)=(u^3,u*v+1)` to the pullback image in
`Jac(v^2+u^2*v=u^7+u)` over the same `GF(2^23)`. Because `gcd(3,r)=1`, the
implemented degree-three composition certificate makes the restriction
injective conditional on the registered exact source order. Destination
Jacobian operations remain unpriced in source GAE; wall-time comparisons and
that lower-bound operation ledger must be reported separately [13–15].

### Trait cautions

The B24 curve is defined over the full degree-24 field according to the traits
catalogue. The *ambient field* has proper intermediate fields because 24 is
composite; this report does not claim that the curve descends to one. Likewise,
`j=0`, `j=1728`, or Koblitz structure identifies an available automorphism, not
its scalar action on the chosen prime subgroup or its measured benefit. ecbench
must verify that action before the corresponding cubic, quartic, or Frobenius
rho arm is used.

## Why these phases

### Phase 1: isolate traits before scaling

The K0/K1 pair changes the Koblitz sign at the same field degree. B21/B24
separates prime- versus composite-degree ambient fields and cofactor 2 versus 8.
P20 provides a generic prime baseline, while the two CM sentinels catch invalid
assumptions about automorphism folding and high cofactors. All subgroup sizes
remain inside the 18–23-bit gate even though the sentinel field sizes are 25 and
28 bits.

### Phase 2: first three-family scaling step

P32a is already in the repository standard panel [8]. B27 is a proved,
low-cofactor general-binary cell. No registered low-cofactor Koblitz curve falls
inside the 27–33-bit subgroup window; the degree-37 Koblitz curve
`icv1-f2m37-tm534059-32aad96b` is therefore retained as a clearly
labeled high-cofactor stress cell rather than misrepresented as a standard-curve
analogue. [pcc:unverified]

### Phase 3: current implementation boundary

B59 and P61 fit the current explicit `u64` general-binary and prime
constructors. The degree-73 Koblitz curve
`icv1-f2m73-tm184271214331-9d25cc67` has a 57-bit subgroup but a 73-bit field, so it routes to
the wide Koblitz implementation. The present ecbench contract gives that path
strong rho/claw methods, not a matching index-calculus pipeline [9]. Phase 3 is
therefore partially blocked until method applicability is symmetric enough for
the planned comparison.

### Phase 4: exact challenge anchors, no interpolation

The degree-83 Koblitz curve `icv1-f2m83-tm6151469093347-debefd74` is already a registered repository confidence-gate curve. ECC2-79 and
ECCp-79 provide exact public binary and prime challenge anchors. Current generic
binary methods stop at degree 62 and the explicit prime constructor is `u64`,
so neither external anchor can be run in the intended index-calculus comparison
today [9]. The phase remains blocked until the curves are locally registered,
generic wide arithmetic is implemented, and candidate/reference parity tests
pass. A projected 80-bit row is not accepted as a substitute.

## Promotion gates

The executable policy is detailed in [`PROTOCOL.md`](PROTOCOL.md). The decision
sequence is:

1. **Admission:** identities, subgroup sizes, proof status, exact curve specs,
   method applicability, strongest rho choice, and all seeds/budgets are frozen.
2. **Relation/rank:** direct relation verification, nonzero lower bounds for
   relation and independent-rank yield, at least 16 terminal episodes, and an
   exact one-sided Clopper–Pearson lower bound of at least 0.80 for full-rank
   success. Statistical pruning uses the existing familywise v1 protocol [10].
3. **Correctness/custody:** zero wrong answers or invalid relations; complete
   cold phase costs and preserved timeouts/OOMs; FB1/FBM1, IC1, plan, seed, host,
   and artifact identities retained.
4. **Controls/replay:** A/A interval includes 1, positive and null controls
   behave, every answer replays locally, and candidate plus rho replay on an
   independent host.
5. **Resources:** the next preregistered envelope fits the approved budget with
   4× headroom or receives an explicit revised cap before running.

At least one candidate in each mandatory core family must complete a ready
elliptic lane. Phase 1’s four extra sentinels are also mandatory. Higher-genus
and transfer lanes have their own mandatory destination objects and route
obligations. Promotion is per lane: a failed or blocked lane stops at that
boundary without blocking or promoting another lane. A redesigned algorithm
restarts at Phase 1 under a new ID.

The default 0.80 full-rank probability, 16 episodes, 2× maximum yield shortfall,
and 4× resource headroom are operational program thresholds, not mathematical
constants. They must be frozen before probes. Changing one after observing data
creates a new candidate/evidence stream.

## Separate speedup gate

Promotion only authorizes the next experiment. A speedup claim additionally
requires verified cold end-to-end solves against the cheapest applicable rho,
complete setup/collection/filtering/linear-algebra/individual-log/verification
costs, at least 32 public targets below 64 bits and 8 above, paired uncertainty
whose upper bound is below 1 at weights `0`, `0.1`, and `1`, controls,
independent replay, and current repository `gate`, `size`, and `artifacts`
checks [11]. A relation-yield improvement alone is not sufficient.
[pcc:unverified]

## Alternatives rejected

- **Use field bits as phase bits.** Rejected because field degree and prime
  subgroup size diverge sharply on the degree-37 and degree-73 Koblitz stress cells; the repository
  contract defines the DLP size by `r` [11].
- **Use exactly three curves in Phase 1.** Rejected because a later failure
  could not be attributed to sign, cofactor, intermediate-field availability,
  or automorphism handling.
- **Require only standardized curves.** No standardized curve exists at these
  toy subgroup sizes. Registered analogues plus exact final challenge anchors
  make the structural mapping and limitations auditable.
- **Drop blocked Phase 4 cells.** Rejected because that would quietly replace
  the requested 80-bit binary/prime comparison with a Koblitz-only result.
- **Project Phase 3 to Phase 4.** Rejected because an extrapolation is not a
  solve, timing, memory observation, or backend correctness test.

## Evidence and access limitations

The repository knowledge-base service and ecdb credential route were unavailable
in this environment. Their absence is not evidence. Selection therefore used
the committed registry/traits catalogues, repository protocols, and the primary
standards/challenge sources below. No curve facts were filled from an unpinned
search result.

The original curve-ladder selection used crypto source snapshot
`46ad9d87de45e372f8a741a3a38dee5b385425eb`; the Phase 1 adapter revision is
implemented in this report's PR source. One-target L0 smoke runs checked both
direct-Jacobian arms and both transfer adapters, but they are correctness
diagnostics only. No 32-target canonical Phase 1 session has run, and this
report makes no performance observation.

## Sources

1. Standards for Efficient Cryptography Group, [SEC 2 v2, Recommended Elliptic Curve Domain Parameters](https://www.secg.org/sec2-v2.pdf), 2010. Retrieved copy SHA-256 `87b8f3703364ed5b21ba8582e411cc0cbf477bcaa3f4f45e0d6580d1c00d9952`.
2. NIST, [SP 800-186, Recommendations for Discrete Logarithm-Based Cryptography: Elliptic Curve Domain Parameters](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-186.pdf), 2023. Retrieved copy SHA-256 `53c7cf3340528aaa0a7bbeaef947f2099568192367d4341a7d4abceb55f78432`.
3. Langley, Hamburg, and Turner, [RFC 7748: Elliptic Curves for Security](https://www.rfc-editor.org/rfc/rfc7748.txt), 2016. Retrieved copy SHA-256 `279ca0ecc5e92e2962e27b846986aeb74729d9dd34bd4a04a362f80dcb596ad3`.
4. Certicom, [The Certicom ECC Challenge](https://www.certicom.com/content/dam/certicom/images/pdfs/challenge-2009.pdf), 2009. Retrieved copy SHA-256 `3a0cff3f84e933e9f9b22a0972fee5673a2ed16a406421f2daa945ffa33d5c13`.
5. Isogeny Labs crypto, [`docs/curves/registry.json`](../../docs/curves/registry.json), exact ICV1/EC1 representations and generators.
6. Isogeny Labs crypto, [`docs/curves/traits.json`](../../docs/curves/traits.json), point-count, subgroup, subfield, and endomorphism statuses.
7. Isogeny Labs ecc-challenges, [pinned `instances.json`](https://github.com/isogeny-labs/ecc-challenges/blob/efd49aa0a802cda679748533ec5bc5e44ab9ea71/src/ecc_challenges/multicurve/instances.json), Git blob `a82dc0ee00a6d25bbfc09f260a4e5430a2a52f9e`.
8. Isogeny Labs crypto, [`docs/ic/STANDARD_PANEL.md`](../../docs/ic/STANDARD_PANEL.md), canonical standard-panel policy.
9. Isogeny Labs crypto, [`docs/ecbench/README.md`](../../docs/ecbench/README.md) and [`src/cryptanalysis/ecbench/workload.rs`](../../src/cryptanalysis/ecbench/workload.rs), current constructor and method boundaries.
10. Isogeny Labs crypto, [`research/relation_yield_pruning_20261031/PROTOCOL.md`](../relation_yield_pruning_20261031/PROTOCOL.md), familywise pruning and full-rank success protocol.
11. Isogeny Labs crypto, [`docs/ic/SUCCESS_CRITERIA.md`](../../docs/ic/SUCCESS_CRITERIA.md), end-to-end evidence and target-count gates.
12. Isogeny Labs crypto, [`research/ic_acct3_hec_ecbench_20261021/AUDIT_IR.md`](../ic_acct3_hec_ecbench_20261021/AUDIT_IR.md), direct hyperelliptic/Jacobian ecbench arm audit and its validation boundary.
13. Isogeny Labs crypto, [`docs/curves/COVERS.md`](../../docs/curves/COVERS.md), executable cover catalogue and its explicit non-solver boundary.
14. Isogeny Labs crypto, [`research/binary_genus2_cover_20261014/REPORT.md`](../binary_genus2_cover_20261014/REPORT.md), checked binary genus-2 cover maps without a destination-solver speed claim.
15. Isogeny Labs crypto, [`docs/transfer-assessment/MEASUREMENT.md`](../../docs/transfer-assessment/MEASUREMENT.md), three-arm and nine-phase transfer measurement contract.
