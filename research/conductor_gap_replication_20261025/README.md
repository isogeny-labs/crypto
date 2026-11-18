# Conductor-gap replication, 25 October 2026

Status: the bounded campaign is measured, replayed and independently checked.
The [frozen design](PROTOCOL.md) defines two separate panels: balanced rho
measurements across eight classes and additional volcanoes, and CM-aware
relation collection with direct-map, scalar and generic-recurrence controls.
The [interpretation corrections](CLAIM_CORRECTIONS.md) narrow the earlier
study's scaling and transfer language without rewriting its sealed evidence.

| Requirement | Current status |
|---|---|
| Eight distinct classes from four CM fields | verified; exact parameter and order certificates retained |
| Three S2 volcanoes per class, common S1 generator | verified; 32 volcanoes, 1,472 vertices and 1,440 independently checked edges |
| Equal sampling effort at each level | verified; 192 separate solves per level/volcano, with shared scalar indices retained as class clusters |
| CM-aware collector in the hard-kernel-field regime | verified on four additional prime-order classes; all 640 matched construction comparisons retain identical non-base work |
| Complete deterministic replay and independent checks | 43,520 main attempts plus 128 excluded calibration attempts verify/replay; separate C++ arithmetic/statistics/graph checks pass on this host |
| Report, figures and PDF | [results](REPORT.md), [PDF](REPORT.pdf), [session reports](reports/README.md); inspection recorded beside the report |

The equal-level secondary S2/S1 counted-rho ratio is **0.99943**, with 95%
whole-class bootstrap interval **[0.98606, 1.01455]**. All five levels pass
the nominal +/-5% rule. Four pass its simultaneous correction; the crater
remains inconclusive at 98% ([0.97429, 1.05660]). The fixed cohort and
shared target indices limit inference beyond these cases.
Class 7's observed crater and deepest-level ratios are about 1.12; pooled
parity does not establish uniform equivalence across individual classes.

The [crater reanalysis and repetition design](../conductor_gap_crater_design_20261025/REPORT.md)
enumerates the complete class-bootstrap distribution: its 98% interval is
[0.97323, 1.05706], also inconclusive. It explains why distinct targets,
solver seeds and classes are different repetition units, and records that
this replication's prime subgroups are only 25–32 bits. Its proposed new
measurement counts are planning scenarios, not completed solver runs; the
frozen data and original interval above are unchanged.

CM scalar/recurrence and scalar/map comparisons keep queries, relations,
ranks and answers identical. Their differences are factor-base construction
work. Complete native setup counts/prices, a prime-field F4/SAT oracle,
hard-gap target transport and cryptographic-scale coverage remain open.

Extract the immutable raw tree from this directory with
`sha256sum -c runs.tar.xz.sha256` followed by `tar -xJf runs.tar.xz`.
The archive contains `sessions/`; the audit workflow replays all 68 sessions.
No raw session is edited by reporting or postprocessing. The postprocessing
example accepts `MODE PREPARED_DIR [ANALYSIS_OUTPUT_DIR]` so summaries can
be regenerated in a fresh directory. The measured source is pinned in
[PREFLIGHT-v3.json](PREFLIGHT-v3.json); postprocessing additions do not
change the measured ecbench solver.

The [catalogue correction](REGISTRY_CORRECTION.md) documents and retracts an
insufficient legacy-preservation assertion in the first registry merger.
The lossless replacement preserves all measured models; a separate
[post-repair replay](validation/post-repair-replay.json) reproduces every
main and calibration record. The [independent validator report](validation/validator-report.json)
pins the raw CSVs and compares 114 native/C++ headline values.

The branch is stacked on PR #2430. No evidence is promoted to a hardness,
scaling, fastest-method or deployed-curve-security claim. The CM collector's
cofactor-one restriction is retained; it is not run on the deep-volcano
panel. A prime-field algebraic F4/SAT collector remains unimplemented.

CI follow-up, 2026-10-25: the delivered head `b9d5caad` passed the
[archive replay and independent-check workflow](https://github.com/aburan28/crypto/actions/runs/40179357412).
The [CI receipt](validation/ci-20261025-b9d5caad.json) pins the retained,
hash-matched artifact: all 43,648 archived records replay exactly, with
43,520 independent main-scalar checks and 114 headline comparisons.
[CI follow-up notes](checks/CI_FOLLOWUP-20261025.md) distinguish this
completed verification from the remaining PR-wide CI gates. Scientific
scope and the open CM accounting/algebraic-oracle obligations are unchanged.
