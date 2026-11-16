# Root-free divisor pencil: exact decompositions and signed rank

The pencil Q_t=(f-v²)/u−2tv−t²u admits a factorization-free grouping algorithm: scan the two rational sheets and bucket (y−v)/u. Every regular full fibre produces one decomposition of the supplied divisor. Its g+1 point-add transitions produce the same decomposition. This is classical Riemann–Roch arithmetic; it adds no relation information relative to all those point-add transitions.

The exact signed rank has a grounded-incidence description. After identifying conjugate sheets and dropping branch columns in an odd-prime projection, full fibres are graph vertices. Two-sheet occurrences are edges; single-sheet occurrences ground vertices. With F>0 full fibres and c closed components, the difference-row rank is F−1−c+delta, where delta indicates whether some closed component size is nonzero modulo the coefficient prime. This corrects the unsupported inference from disjoint oriented supports to signed rank. See [derivation](THEORY.md) and its input hypotheses. It is standard incidence algebra, with no mathematical novelty claim.

## Frozen finite gate

Scientific source 066cc22aa56e (full SHA in FREEZE/checkpoint), dependency-lock correction de08f9286c6f, S3 checkpoint 20261116T063449Z-de08f9286c6f before once-only execution. Native control exits0, stderr empty. Source, imported native crate and binary checksums passed. Fixed curves g2/F101/c1, g3/F61/c1, g4/F31/c1; 24 deterministic divisors per curve from [17..40]P. These are finite controls, not fresh population holdouts. Of 72 inputs, 69 satisfy full-degree, squarefree-u and nonbranch-support hypotheses; three exclusions remain in CONTROL.tsv. No replacement inputs or reruns.

All regular parameter fibres match independently formed norm-polynomial roots; original Jacobian principal identities, all point-add residuals, repeated-decomposition equality, oriented rank and signed graph-rank predictions pass. Exceptional parameters (support overlap or nonsquarefree residual) remain counted separately and are outside the implemented simple-fibre gate, although the written identity allows multiplicity.

| Genus / p | Eligible | Excluded inputs | Scanned points, summed | Exceptional parameters | Full fibres, summed | Point-add transitions | Sum of within-pencil signed ranks | Full fibres per eligible input |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 2 / 101 | 24 | 0 | 2740 | 43 | 487 | 1461 | 463 | 17–24 |
| 3 / 61 | 23 | 1 | 1555 | 38 | 70 | 280 | 47 | 1–6 |
| 4 / 31 | 22 | 2 | 716 | 48 | 5 | 25 | 0 | 0–1 |

These are exact deterministic counters, reduced in native Rust from CONTROL.tsv; no uncertainty estimate or wall timing. Rank sums are descriptive sums, **not the rank of a pooled multi-pencil matrix**. No closed components occur in this curve panel, so the finite checks exercise the c=0 case of the signed formula; the general c>0 statement rests on the explicit kernel proof, not an unrun finite fixture. Modulus1000003 is an algebra-control prime, not the subgroup order. Shared field/group code remains a validation dependency. No machine-checked group certificate or independent scientific validation is claimed.

## Cost decision and successor

A full sweep charges catalogue discovery, field evaluation and quotients, bucket storage, exceptions and rank work. The point/fibre capacity inequality and exact duplicate relation show why multiplying successful transitions by g+1 overstates useful output. The uniform-fibre heuristic p/(g+1)! is not a theorem for these selected divisors. A scan can amortize arithmetic, but it requires a whole rational-point catalogue while Compact works with a smaller target-seeded anchor system. There is no complete tenfold premise here across genus2+. No timing campaign or performance claim follows.

The unresolved useful mechanism is to obtain enough **anchor-supported independent constraints** from fibre incidence without scanning full fibres into a larger matrix. A successor must construct and charge that elimination, with setup and target descent included. The present rank law supplies an exact check for that future construction. The mathematical tenfold objective remains active.

## Artifacts and limits

[CONTROL.tsv](CONTROL.tsv), [native summary](SUMMARY.tsv), [protocol](PROTOCOL.md), [theory/source receipt](THEORY.md), [quantitative graph](COUNTS.svg), [PDF report](reports/run-20261116T0634Z/REPORT.pdf). Native reducer source is reduce.rs; it was written after the gate to summarize fixed raw counters and render layout, without changing selection or outcomes. Graph and all report pages were inspected. No benchmark record or benchmark-index row is added because this is a correctness/count screen, not elapsed-time measurement. Existing canonical performance graphs remain unchanged. PR unmerged. The shared daily log remains reserved; SESSION.md carries this task's handoff. Primary external PDF access failed; no priority claim is inferred from that gap.

## Post-control constructive anchor-elimination derivation

The incidence structure gives a concrete successor rather than a requirement to build a larger matrix. Using only off-anchor columns, let k be the number of ungrounded components and delta_O the rank of their size functional. The exact anchor-supported intersection dimension is (k−delta_O)−(c−delta). Sum fibre rows within each ungrounded off-component, restrict to anchors, then enforce the zero-sum coefficient condition using component sizes. These component combinations span the anchor intersection without dense elimination on the full catalogue. Proof and explicit basis are in THEORY.md. This derivation was made after the finite gate; it has **not** been tested by that gate. Next action is an independent full/off-rank and row-containment comparison under a new frozen protocol. It supplies a constructive algebraic route, not a positive k or complete-speed guarantee.
