---
title: "Projected queries: checked native support and remaining IC work"
author: "ECDLP research programme"
date: "2026-10-26"
geometry: margin=22mm
fontsize: 10pt
colorlinks: true
header-includes:
  - '\usepackage{graphicx}'
---

This report covers native query and supplied-relation verification. It contains
no new projected-curve panel, timing comparison, complete IC price or measured
crossover. The original subgroup and cofactor receipts remain unchanged.

## Conditional query law

Assume a correct finite abelian curve group of certified order $hr$, a prime
$r$, $\gcd(h,r)=1$, and a generator $G$ of exact order $r$. Let
$H=\langle G\rangle$ and $K=\ker[h]$. Then addition identifies $H\times K$
with the curve group. For a public target $T=[d]G$, independent uniform
$k\in\mathbb Z/r$ and $U\in K$ give a uniform full-curve query

$$R=T+[k]G+U.$$

The helper accepts indices; it supplies no random sampler. The earlier draft
with $U=O$ samples only $H$, so it cannot justify full-curve relation yield.
Kernel randomization corrects that design before a projected-IC experiment.

For a verified full-curve relation $\sum_i\epsilon_iP_i=R$, each supplied log
must satisfy $[L_i]G=[h]P_i$. Then

$$S=\sum_i\epsilon_iL_i=h(d+k)\pmod r,
\qquad d=h^{-1}S-k\pmod r.$$

Native recovery checks every projected log, the signed full-curve relation and
finally $[d]G=T$. Identity hits, zero projections and repeated or negative terms
remain explicit. Neither equation discovers a relation or computes base logs.

\newpage

## Native implementation and verification

The original cyclic constructor checks $h\le4096$. The subsequent supplied-kernel
constructor accepts an ordered complete census with $h\le65536$, including
noncyclic kernels. It checks generator/order consistency, every element's
membership and annihilation, exact length, uniqueness and identity. It retains
caller order as the index map. Completeness is conditional on the certified
ambient order and correct backend; the helper does not create either certificate.

The bound includes 4232. Its corresponding source test uses an abstract cyclic
group, not the original E0 curve. No E0 kernel or new qualified-cell certificate
has been produced. The noncyclic fixture is $\mathbb Z/7\times(\mathbb Z/2)^2$;
all targets and all query indices are checked. A separate 7-bit binary-curve
fixture uses seed 202610260001 and independently enumerated polynomial-basis
points/logs; its generator uses at most 16 construction attempts. This is a source
test domain, not a measured IC rung or replacement for the n=22/25 panels.

| Check | Recorded outcome and scope |
|:--|:--|
| Original helper source `0670ec6fa` | Eight Cargo tests pass; all earlier sparse failures retained |
| Original default Clippy | Exit 0; seven unchanged library warnings, zero own-helper diagnostics |
| Supplied-kernel source gate | Source `84ddcb9b0`: 11 Cargo tests and 10 bare-core tests pass; default lint exits 0 with seven unchanged library warnings |
| Existing Lean query dependencies | Five pass, zero trusted additions; general group bridge open |
| Exact residual certificate and proof | `IDC1h08a8b0da9bbd7a55`: separate native check passes; one exact ring lemma passes, zero trusted additions |

The supplied-kernel gates ran at `84ddcb9b0`; earlier checks and the certificate
retain their own recorded revisions. Integration against a newer repository
base requires its own check. These are correctness gates on a shared Linux workspace, with no admitted
performance isolation, runtime metric or statistical effect. The 7-bit fixture
and abstract groups establish their finite checks only. All construction,
field, membership, kernel-storage and query work would need pricing in a full
instrument; a group-call ledger alone is incomplete.

\newpage

\begin{center}
\includegraphics[width=155mm,height=180mm,keepaspectratio]{query-law.pdf}
\end{center}

**Figure 1.** Solid steps are checked native support; dashed boxes remain missing. Source: query-law.dot and the native example. No runtime or relation-yield measurement is plotted.

The supplied kernel closes an interface limitation, conditional on an actual
complete census. The relation oracle, projected factor-base logs and mod-$r$
linear algebra are separate missing components. A useful projected relation
pipeline must also handle signed deduplication, target solving, every failed
query and final public-scalar verification.

\newpage

## Proof and certificate boundary

The five existing Lean declarations prove explicit additive-projection,
commutative-ring recovery/setup/identity-hit, and supplied-coordinate
uniqueness dependencies. They do not construct the finite-group decomposition,
certify curve cardinality or prove a concrete sampler uniform.

The new residual statement is the unconditional polynomial identity

$$h^{-1}_{\rm supplied}\bigl(h(d+k)\bigr)-k-d
=(h^{-1}_{\rm supplied}h-1)(d+k).$$

Here $h^{-1}_{\rm supplied}$ is just a variable; vanishing additionally requires
it to be an actual inverse. The identity-certificate v1 checker evaluates a
fixed modulus $2^{61}-1$ at 32 statement-derived points, with a separate checker
using 256 extra domain-separated points. Its Lean pointer must be checked
separately; fixed-modulus evaluations alone are not a proof over every ring.
The polynomial has degree 3 and coefficients of magnitude 1. The general-group
cardinality/uniformity bridge remains open.

## Sources, receipts and next test

The implementation follows the study's cofactor audit, PROTOCOL.md and lead
atlas entry LEAD-ICB-cofactor-torsion-quotients. SEC1v2 sections 3.1.2,
3.1.2.2 and 3.3.2 were read for binary-curve/order/cofactor conventions and
validation: <https://www.secg.org/sec1-v2.pdf>. Download SHA256:

`54687189af3de645756a30a801b596fd14c6566992c13bfd58eede399f324a89`.

Gaudry 2009, DOI 10.1016/j.jsc.2008.08.005, was located in official HAL metadata,
but its PDF returned 403; it is not claimed as read. No novelty or asymptotic
conclusion rests on that inaccessible source.

Repository paths below use this study root:

`research/ic_algebraic_crossover_20261023`.

- [Implementation/source limitations](../../PROJECTED_QUERY_LAW_IMPLEMENTATION_20261026.md).
- Native source: `examples/ic_projected_query_law.rs` and its `core.rs` module;
  separate certificate example `examples/ic_projected_recovery_identity.rs`.
- [Original exact gates](../../pipeline-receipts/projected-query-law-source-checks-20261026/attempt-3/status.json)
  on source `0670ec6fa`; failed attempts 1 and 2 remain alongside them.
- New gates: [supplied kernel](../../pipeline-receipts/projected-query-law-supplied-kernel-20261026/status.json) and [residual identity](../../pipeline-receipts/projected-recovery-identity-source-20261026/status.json).
- Proofs: `proofs/ProjectedQueryLaw.lean` and `proofs/ProjectedRecoveryIdentity.lean`.

Next, construct and independently verify complete order/kernel receipts for the
actual qualified models, implement a uniform bounded sampler and complete cold
projected relation/log/LA/recovery instrument, then freeze a controlled full-cost
comparison against the current IC baseline and strongest matched rho. Preserve
all failures, original workloads and total accounting. No additional VM launched;
the total additional-cloud cap remains USD100 including replay, with workspace
billing unmeasured. Larger 64/256 GiB requirements and complete wide quartic
adapters remain outstanding.

Canonical scoreboard, progress timeline and performance figures are unchanged:
these source checks introduce no admitted runtime, yield or crossover value.
