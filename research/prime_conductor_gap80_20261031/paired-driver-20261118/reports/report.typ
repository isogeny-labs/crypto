#set page(paper: "a4", margin: (x: 15mm, y: 14mm), footer: context align(right)[#counter(page).display()])
#set text(size: 10pt, font: "DejaVu Sans")
#set par(leading: 0.5em)
#set heading(numbering: none)
#let s = json("../summary.json")
= Transported endpoint inputs
Native preparation and public correctness controls · 18 November 2026 · PR #4721

The native bridge evaluates the quotient map on a generator and shared-scalar
targets, then exports inputs for the existing ecbench harness. It also emits
a concrete scalar multiplication input matrix and a coordinate-isomorphism
control. This component adds no timer or ECDLP method.

#image("flow.svg", width: 100%)
*Conceptual flow:* the three small cases use exact finite-field equations and
evaluated maps. This diagram assigns no conductor strata or registered IDs.

== Recorded controls
#table(columns: (2fr, 1fr, 1fr, 1.8fr), inset: 5pt, stroke: 0.4pt,
  [*Field p*], [*Degree*], [*Targets*], [*Recovery per end*],
  [6666679], [37], [32 distinct], [32],
  [223], [5], [8 development], [8],
  [229], [5], [8 development], [8],
)

Direct PARI compares quotient coefficients, three generator images, 48 target
images and six exceptional inputs. It recovers 48 known scalars on each
endpoint. The pinned consumer plan agrees on 144 scalar/coordinate rows
across source, destination and coordinate control. No times were measured.
[pcc:unverified]

The shared law hashes canonical ASCII with domain
uniform_scalar_sha256_shared_v1 and parts [r, seed, index], taking the first
eight digest bytes in big-endian order, then 1 + v mod (r−1). The main packet
requires 32 distinct scalars. The coordinate control uses u=2, sending
(x,y) to (4x,8y) and (a,b) to (16a,64b).

== Compatibility and admission
Current ecbench PrimeExplicit accepts p below 2⁶². Both the frozen 202-bit
target and 96-bit pilot reject before large construction. The exact guards
prevent truncation; small inputs do not replace these targets. The accepted
prospective spec includes rho.negation, two BSGS arms, ic.pipeline and a rho
A/A control, with five rounds, one warmup and L2. No arm was executed.

#pagebreak()
= Evidence and open measurements
#image("counts.svg", width: 100%)
Counts are exact recorded correctness observations, not performance samples.
No timing uncertainty interval applies. [pcc:unverified]

== Original requirement status
#table(columns: (2fr, 3fr), inset: 5pt, stroke: 0.4pt,
  [*Requested deliverable*], [*Current status*],
  [Certified prime gap at least 80 bits], [Frozen source/kernel evidence; full target quotient and endpoint certificate open],
  [Explicit target map], [Small map implementation checked; full work/memory schedule unresolved],
  [Dozens of points, several fields], [32 distinct prepared targets plus two eight-target development fields],
  [Repeated scalar timings], [Concrete input matrix; canonical measurement integration pending],
  [Matched endpoint ECDLP and IC], [Prospective arms accepted by planner; no measured session],
  [High fidelity], [Endpoint order, algorithm-seed pairing, strongest wall reference, registry and cloud admission open],
)

Alternating arms alone does not alternate endpoint order. Shared scalar values
do not establish algorithm-seed pairing. External construction, conversion,
verification, memory and setup/online/recovery costs still require explicit
accounting. high_fidelity_admitted is false; runtime_measurements is null.

== Replay and standing
The raw packet is in runs.tar.xz, with named proof subjects loose. Fresh-output
qualifiers, exact input files, pre-call source/binary hashes, build logs,
negative controls and exit records are retained. GP attempt A's syntax error
is retained alongside clean v2. Raw plans preserve full u64 seeds.

PCC-20261118-07cc48 and PCC-20261118-83a506 are unsigned producer attestations:
verdict not_covered, standing unverified. They do not supply an independent
execution verdict. The official v0.1.6221 consumer, source a2be660914e8, is
used for plan/codec compatibility only. Cargo and conductor are unavailable.
[pcc:unverified]

*Sources:* repository workload.rs (PrimeExplicit and SHARED_TARGET_LAW),
spec.rs, canonical.rs and docs/ecbench/README.md at parent 979ab9100a36;
PARI Elliptic Curves reference (ellisogeny, ellisogenyapply, elllog);
official release v0.1.6221; parent complete_pipeline.cpp and its projective
controls. Full links and exact consumer hash are in the companion README.
KB wrapper unavailable, exit 127. No novelty or world-record claim is made.
