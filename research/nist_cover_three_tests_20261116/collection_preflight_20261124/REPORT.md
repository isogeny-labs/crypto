---
title: "P-192 collection: frozen-input preflight"
date: "2026-11-24"
geometry: margin=0.75in
fontsize: 10pt
---

The successor collection driver now has an explicit frozen-input check before
new split-tangent root extraction. The native guard compares the regenerated
baseline with the fixed reference and pins the cover, quartic model and
backmap inputs. Recorded-data qualification and isolated driver transport
checks pass. The live scientific collection remains unexecuted.
[pcc:unverified]

## Gap and resulting behavior

The preceding driver regenerates its baseline, but did not compare every
fixed point/base/reference binding against the frozen record before entering
collection. Its downstream checks establish equations and counts, which do
not replace an explicit input-epoch comparison. The new driver requires
both consistent mathematics-bearing baseline fields and exact construction
inputs before invoking the collector. The old pinned driver stays byte-equal.

The [native guard](preflight.cpp) checks 17 deterministic root fields,
including pool ordering and reference support, individual images and signs,
canonical base, relation rows, public composition companions and counts/rank.
It requires the same top-level schema. Only standing, stages and cost_scope
values may differ; those metadata values confer no verification authority.
[pcc:unverified]

The reference is bound by an exact SHA256 pin. Three further pins bind the
cover, plane-quartic model and rational backmap. The separate typed copy of
the repository's recorded-JSON reader preserves scalar types and numeric
lexemes. It rejects unsupported encodings instead of silently coercing
values. This is input consistency, not independent curve/Jacobian arithmetic,
admission validation or a signed mathematical verdict. [pcc:unverified]

## Native qualification actually executed

The [protocol](PROTOCOL.md) froze the controls and caps first. Matching
recorded reference/local baselines pass. Changing only the three declared
metadata fields also passes. All twelve semantic mutations reject:
coordinates, point order, old-base coordinate, projection sign, image,
reference support, row coefficient, public norm companion, rank, missing
field, extra field and a value-type change. Four exact-byte pin mutations
reject even when the change is only JSON whitespace. [pcc:unverified]

Number/string and array/object distinctions pass their two parser controls.
Both self-test and normal check modes exit zero on recorded input. Unknown
mode, ambiguous arguments and an existing output reject; output-collision
calls preserve the original result bytes. The strict native build passes.
Raw [self-test](evidence/qualification-v1/self-test-result.json) and
[normal-mode](evidence/qualification-v1/check-result.json) results are retained
with complete source/input/binary bindings, exits and process diagnostics.
Record PCC-20261124-5b707d remains producer-unverified. [pcc:unverified]

## Driver ordering checked with scientific dependencies stubbed

The [successor driver](run-admitted.sh) qualifies sources, runs the fixed
baseline panel, then runs the native preflight before collector --collect.
It records the guard's exit/cost and binds complete source, input and guard
result hashes into the pre-collection manifest. A failed guard stops the
pipeline. It adds no budget authorization or signer configuration.

To check that actual shell control flow without new mathematical work, the
[transport fixture](transport_fixture.py) copies the exact driver into isolated
study trees. The baseline, collector, checker and qualifier dependencies
are explicit shell stubs. Only the real qualified native guard executes.
The simulated envelope is labelled not admitted, and paths contain spaces.
[pcc:unverified]

| Recorded-data transport case | Guard | Pipeline | Collector/checker sentinels |
|---|---:|---:|---|
| Matching baseline | 0 | 0 | Both reached |
| Literal rank-field drift | 1 | 1 | Neither reached |

In the failing case no collector result exists. This checks path transport,
ordering and exit propagation only. It is not a real baseline replay,
relation collection, target-principality check or performance result.
Records PCC-20261124-751e73 and6b6a41 retain unsigned producer standing.
[pcc:unverified]

![Actual transport fixture: only the input guard runs real validation code. Scientific dependencies are sentinels; changed data stops before collection.](transport.pdf){width=93%}

## Reproduction and limits

The original transport helper's first-run qualification path remains pinned.
The [portable wrapper](replay_transport.py) accepts a supplied native
qualification directory and stages a minimal isolated mirror. It runs the
unchanged helper/driver with byte-equality bindings and reproduces both
frozen outcomes without modifying original evidence or requiring its ignored
local binary. The supplied binary is hash-bound. [pcc:unverified]

From this component directory, use fresh output paths:

```sh
bash qualify.sh /tmp/new-preflight
python3 replay_transport.py /tmp/new-preflight /tmp/new-transport
```

These commands perform recorded-data guard and stub transport checks only.
They do not launch the collection. The actual scientific worker remains the
prepared individual-image collector from PR5101; its hypothesis and frozen
102-section domain are unchanged. No new curve/group/field/Jacobian work
was performed in this component. [pcc:unverified]

Guard builds have a 300second/1GiB cap, native calls 30seconds/1GiB and
transport fixtures 120seconds/1GiB. Costs are local process diagnostics,
not admitted performance or billing measurements. One Python syntax check
failed before any fixture execution; its exact source/error is preserved.
Corrected transport and portable replay both pass their declared outcomes.
[pcc:unverified]

The [isolab-eks skill](https://github.com/isogeny-labs/workspace/blob/main/.agents/skills/isolab-eks/SKILL.md)
requires: “Stop new launches when the budget is exhausted or its remaining
balance is unknown.” The live collection still needs external admission.
Independent signing and complete original map-synthesis/setup accounting
remain open. No unchanged denied probe or paid launch was repeated.

Sources checked: the previous admitted driver and protocols, fixed replay
records, native json.hpp and metadata validator. System JSON libraries were
unavailable; the guard adds no dependency beyond the existing compiler and
OpenSSL. KB is unavailable in this runtime; no novelty claim is made.
