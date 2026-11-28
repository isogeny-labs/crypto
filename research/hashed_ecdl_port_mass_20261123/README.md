# Actual child port and vertex accounting

The source constructor has two different full-family identities for one fixed
coordinate, kernel and mode tuple. With M parent edges, r group labels,
P packets and V old vertices, its constructed children satisfy:

```
sum_g child_edges(g) = 2M
sum_g child_vertices(g) = V + rP
```

Both include multiplicities and isolates. A common natural precision weight p
gives `sum_g child_edges(g)*p = 2M*p`; this is structural accounting, not bit
operations. Formal claim **PCC-20261123-7df097** and native observation
**PCC-20261123-5429db** remain unsigned. [pcc:unverified]

The balanced real-constructor fixture has M=8192, r=64, P=4096 and V=16386.
Its 64 children each have 256 edges: 16384 edges in total, alongside **278530
vertices**, including **254016 isolated new packet vertices**. An edge-only
description therefore omits a substantial part of what this implementation
constructs. No timings or extrapolated exponent are assigned to these counts.
[pcc:unverified]

## Why the identities hold

`packing::pack_labels` checks that each original edge occurs exactly once.
`deformation::child` retains one new edge for every selected original left or
right endpoint. Each endpoint label selects exactly one group, even for
parallel edges, giving two child edges per parent edge over the full family.
Beta affects coefficients, not this incidence topology. Length/validity/error
conditions remain separate. [pcc:unverified]

The constructor separately retains every old vertex in its labeled group,
including old isolates, then unconditionally pushes one new packet vertex
for each packet in every group. Hence V+rP. Endpoint-port labels cannot stand
in for old-vertex labels, since they omit degree-zero vertices. The evaluator
may stop at a zero product; an actual visited prefix is a separately counted
consumer of this full-family bound. [pcc:unverified]

The [shared laws](https://github.com/aburan28/crypto-lean/blob/50b95696db2e7a34b7c5f1ba47eb5408f49ca12d/CryptoLean/Combinatorics/PortMass.lean)
apply existing `Multiset.sum_count_eq_card` and count/card/map/add/scalar laws.
The study [instances](formal/PortControls.lean) add no replacement counting
engine or graph wrapper. Bounded Rust extraction checks bind the actual
objects separately; they are not a full Rust refinement proof. [pcc:unverified]

## Observed checks

| Check | Outcome |
| --- | --- |
| Frozen native library harness, one execution | 160 passed; 0 failed/ignored/filtered |
| Base contexts / distinct context-tuples | 2080 / 3388 |
| Base actual group constructions | 25490; below the prospective 46560 cap |
| Balanced actual group constructions | 64, counted separately |
| Malformed-beta/coverage and real-Limit probes | 6 child probes, counted separately |
| Positive-M edge-once and row-only model controls | 3128 tuples |
| Source-object identity checks | Edges, endpoints, packet ownership, Arc identity, creation precision and old/new/isolated vertices |
| Final offline locked build and all-target Clippy | Pass; warnings denied |
| Shared / study unchanged proof gates | 3 / 3 theorems; 0 failures, 0 trusted native proofs |
| Explicit axiom queries | 6, only propext / Classical.choice / Quot.sound |
| Official fresh kernel replay | `leanchecker --fresh PortControls`, exit0 |
| Raw native output | 3394 JSONL rows, 932711 bytes; retained locally |

The existing local PCC verifier also passes all three declaration bindings and its proof-gate checks on the prepared publication source; see [phase12](evidence/phases/12-publication-formal-pcc/stdout.txt). Signed standing remains unverified. [pcc:unverified]

These are unsigned producer observations, not a signed independent execution.
The native meter's 3596756 diagnostic steps and harness elapsed time are not
calibrated cost or performance comparisons. [pcc:unverified]

The base sweep uses eight (r,a) geometries, each coordinate, eight M values,
five graph families and four label schedules, including parallel ports,
stars, six-pair shared incidence and isolates. Real `modes` and `next_tuple`
produce the tested admissible, deduplicated first-mode tuples. The balanced
fixture uses actual radix64 packing bounds and z=0, hence no retained modes.
It has 262144 new packet vertices and 16386 old vertices, including two old
isolates. New tensors have creation precision7; old tensors retain precision3
and the same Arc objects. These are forced component geometries, not
Setup-admitted production recursion or a complete contraction evaluation.
[pcc:unverified]

Negative controls match exact causes: wrong beta length is Invalid; omitted
coverage and duplicated coverage require the specific Integrity errors. A
separate valid-packing construction actually reaches a resource Limit, which
must not be accepted as a coverage error. The retained v1 source was compiled
but never executed; both its three-pair family issue and permissive error
matching were corrected prospectively before the single v2 run. [pcc:unverified]

## Qualification and custody

Focused proofs use official Lean4.35.0 and Mathlib
`56a005855ea17e0b0958f73ec801ae140b0a1679`. The shared owner's default
`cfc55fc58714a59dc06e6e79edc6d1d95140d3c2` remains unchanged and is not
qualified by this run. The owning counting file is identical at both pins;
changed transitive sources prevent treating one cache as the other.
[pcc:unverified]

The frozen native crate has 88 inputs. In addition to the new test and its
four-line registration, it contains three pre-existing local formatting
variants in floor_sum, short_box and unimodular. Their exact versions are
retained in [attempts/frozen-crate](attempts/frozen-crate) and the local
capsule; they are excluded from publication changes. This qualifies that
frozen crate, not every byte of the current published crate. The constructor
algorithm bodies are unchanged. [pcc:unverified]

All input/binary pre/post hashes pass. The executable SHA256 is
`c8e95752b2f64029d065d7934482a7f501f90430b937447f60ae727fcf0d45a2`.
The test source SHA256 is
`8dd661b57073c21f6acc21b89de677290a9c4e1870c6a8f3a0183ef0b3cfbbff`.
[SOURCE-BINDING.json](SOURCE-BINDING.json), [native summary](evidence/native-summary.json),
[copied source map](evidence/SOURCE-MAP.json), [review02](reviews/PORT-MASS-SOURCE-REVIEW-02.json)
and [PDF report](reports/port-mass-20261123/report.pdf) preserve their scopes.
Reviews are same-team written source checks, with no reviewer execution.
[pcc:unverified]

Attempts01 and08 failed on assembly/library references; 05,07 and10 hit the
unchanged physical stop. All attempted sources and diagnostics remain, with
Lean attempts stored as `.lean.txt` evidence rather than active targets.
Phase11 passed after compared lossless archives freed idle storage. Limits
stayed at one worker, 600s per phase, 120s native execution and a sampled
15032385536-byte whole-cgroup stop; its peak was14816174080. No theorem
statement, gate, pin or limit was relaxed. Administrative preamble/path bugs
and the early proof-edit sequencing error are explicitly recorded in the
addenda and [task-only session excerpt](SESSION-LOG.md). [pcc:unverified]

## Sources, reproducibility and next obligation

Sources checked: pinned primary AppendixD.4.6/D.11, OpenAI/math42907c9e;
actual `packing.rs`, `deformation.rs`, `recursive.rs` and `Bipartite::validate`
at crypto5ec6aed0; Mathlib counting and multiset scalar/cardinality sources.
KB MCP was absent, wrapper127. Bounded GitHub searches and exact source
reads supply no novelty clearance. The counting mechanism is established
mathematics. Prospective plans and all addenda precede their corresponding
scientific executions, except the explicitly preserved sequencing error.
[pcc:unverified]

To reproduce the exact native source, copy the crate into a fresh directory,
overlay the three retained formatting variants, and verify all 88 inputs
against [the binding](evidence/frozen-crate-main-binding.json). Use official
Rust1.99.1 and the frozen Cargo.lock; build release/offline/locked with one
worker and run strict all-target Clippy. Execute the copied test binary with
`--test-threads=1 --nocapture` and `PORT_MASS_ROWS` set to a **new** output path;
the native writer refuses to overwrite an existing row file. Preserve the
same prospective limits and distinguish a new replay from this original run.
The full local capsule retains the qualified executable, all 88 input bytes,
raw rows, failed attempts, review inputs and exact restoration mappings.
[pcc:unverified]

The next cost model must include vertex/packet descriptions, entry evaluation,
mode enumeration, interpolation and arithmetic widths. The source's strict
`8Mchild<=Mparent` check is an upper bound, and does not imply a lower child
size. A subsequent history bridge should inspect the actual active-call leaf
guard. Isolated packet removal is a separate hypothesis: first establish its
creation-time scalar value, retain old isolates, and compare complete
contraction values with nonunit countercontrols before changing allocation.
No such pruning experiment has run in this study. [pcc:unverified]

The prior 360-call comparison, original protocol export failure and root35 /
conditional34 / history81 records remain unchanged. Canonical runtime charts
and rankings are unchanged because this study measures structural accounting.
Local-only storage honors the user's no-S3 instruction. PR5046 continues the
merged PR4996 lane; default-pin integration and signed standing remain open.
[pcc:unverified]
