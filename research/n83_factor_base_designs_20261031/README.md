# Sixteen factor-base experiments: relation yield and certified degree

Status: research design and certificate tooling; no new factor-base measurements.
Date: 2026-10-31. Source baseline: `48c8a4767d2d1eed6a7b379f74410bc729239380`.
The active target is the primary binary Koblitz model
`icv1-f2m83-tm6151469093347-debefd74`, with equation
`y²+xy=x³+1`, field modulus `z^83+z^45+z²+z+1`, subgroup order
`2417851639230796216685689`, and cofactor 4. The field degree is 83;
the subgroup order is not 83. Generator, representation UID and subgroup
action must be copied from the existing exporter, verified and frozen before
execution. The separate 53-bit-subgroup degree-83 model is a diagnostic arm.

Deliverables: [candidate catalogue](CANDIDATES.md),
[preregistered protocol](PROTOCOL.md), [proof obligations](DEGREE_PROOF.md),
[screen diagram](screen.svg), and [renderable report](REPORT.typ).
The [three-page PDF](REPORT.pdf) includes the catalogue and diagram.
The native certificate audit lives in `native/degree_certificate.rs`.

| User requirement | Evidence or remaining obligation |
| --- | --- |
| Devise 10–20 factor-base ideas | Sixteen explicit constructions in CANDIDATES.md, including prior-art and algebraic admission checks |
| Improve actual relation yield | Exact all-target census on complete controls, then frozen unplanted degree-83 trials; execution pending |
| Establish lower degree | Independent upper and lower bounded-Macaulay certificates; whole-system Gröbner/regularity obligations remain explicit |
| Preserve naming and storage | Existing ICV1 model; canonical FB1 dumps and exact replay required before a candidate enters the run |
| Run reproducible experiments | Frozen seeds, panels, caps, branch-coverage and output contract in PROTOCOL.md |
| Publish and prepare PR | Local commit required; GitHub authentication currently rejects the injected token |

## Sources checked

Read before selecting this round's arms:

- Igor Semaev, *New algorithm for the discrete logarithm problem on elliptic
  curves*, [arXiv:1504.01175v1](https://arxiv.org/abs/1504.01175), sections
  2–4. The regularity-at-most-four argument depends on a first-fall-degree
  assumption. Chained S3 equations alone do not establish that bound. The
  chain equivalence also has hypotheses concerning shorter decompositions.
- Semaev's original summation-polynomial paper,
  [ePrint 2004/031](https://eprint.iacr.org/2004/031), was requested but its PDF
  returned HTTP 403 in this environment; its full text was not inspected.
- [Existing degree campaign](../factor_base_dor_campaign_20261013/REPORT.typ):
  historical subfield and dimension/slack controls, with their original
  implementation provenance retained. These motivate controls, not new data.
- [Shape-search results](../shape_search_g1_g6_20261015/README.md): several
  shapes give identical measured degrees; reciprocal-pole variation was
  negative; the trace-variety arm was not built.
- [Shape-search open grid](../notes/index-calculus/RESEARCH_FACTOR_BASE_SHAPE_SEARCH.md),
  section 7; [degree-measurement note](../notes/index-calculus/RESEARCH_DREG_MEASUREMENT.md).
- [Factor-base encyclopedia](../../docs/ic/factor-base-encyclopedia/README.md),
  C1–C11 and D1–D10: prior Hamming, trace, symmetrised and orbit-union studies.
- Current `koblitz_groebner::solving_degree` documentation: its legacy name
  denotes bounded-Macaulay resolution, not homogeneous degree of regularity.

No universal novelty claim is made. A sibling-repository lead-atlas review
and the remaining primary-paper review must accompany any candidate promotion.

## Execution status

The restored worker is connected. A fresh sparse checkout and Rust 1.94.1
toolchain were recovered after the earlier volume was lost. No experiment
survived that loss and no new experiment has been launched.

`conductor check` was attempted; Conductor is not installed. The coordination
documentation describes this as a warning/fail-open condition. Edits are
restricted to this new study, its session-log entry and a linked problem record.
`gh auth status` reports an invalid injected token. The worker has no configured
AWS/outbound identity or installed EKS MCP connection. The repository checkpoint
gate requires a durable S3 resume upload before a research run; its admission
failure is preserved in the session log. Local builds and correctness unit tests
remain authorized development work.

The next execution action is: pass the checkpoint gate, freeze a tested source
commit and image, then submit stage A through the maintained Iso Lab contract.
See [access dependency](../../problems/n83-factor-base-screen-execution-access.md).

## Dashboard disposition

Checked the scoreboard, progress timeline, leaderboard and lab-browser rules.
This study adds proposals and development tooling, with no measured factor-base
row, new curve, FB1 object or speed ratio. Existing numerical graphs remain
unchanged. The study diagram shows proposed execution stages only. A result PR
must update affected views from frozen verified records.

Experiment flow: 16 proposed families; 0 research jobs submitted; 0 completed;
0 candidate families excluded by a measured result. Algebraically inadmissible
variants are documented within their family and are not counted as failed runs.
