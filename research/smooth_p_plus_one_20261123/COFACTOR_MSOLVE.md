# Same production cofactor equations, msolve backend

2026-11-24. Frozen before the new external-solver panels. Local development
protocol; [pcc:unverified]. The thirteen earlier native F4 cap failures remain
at their original source and limits in `evidence/production_v1`.

## Discriminating question

Can the existing production quotient-product equations complete the full-base
signed-set controls using msolve 0.7.5 under the same enclosing three-second
query deadline? This separates a property of the representation from the
observed native F4 configuration. It does not increase the old F4 limits or
claim equal internal resources between different solvers.

## Exact construction boundary

`prepare_cofactor` takes the same public curve, base, target and construction
limits as `extract_cofactor`. It passes through the same input validation,
RR5 chart exclusions, coefficient construction and normalization. Immediately
before native F4, it returns the normalized equation vectors instead of
starting elimination. The original F4 execution branch retains its input,
options and resource controls.

For the production parameters \(b,c,d\), write
\(V=X+b\), \(A=d+cX\), and target \((r,s)\). The production quartic is

\[
U=(X^2+rX+r^2+a)V^2-(X-r)A^2-2sAV.
\]

Its cofactor constraints are the coefficients of \(UW-h\), with monic
\(W\) of degree \(B-4\) and
\(h=\prod_{P\in\mathcal F}(X-x(P))\).
The exported equations are exactly the native normalized vectors; a recorded
nonzero multiplier relates each to the corresponding raw coefficient.
Quartic coefficient polynomials remain unscaled. Their order is ascending in X.

The existing Dickson RUR parser uses `t0=b`, `a1=c`, `a0=d`. The transport
labels the variables accordingly, followed by `w0,w1,...`. It writes the same
native equation vectors in msolve's text format. A scalar binding control
compares the exported quartic and all cofactor equations with the independent
numeric Dickson construction on both finite fields and 27 parameter choices
per field. Invalid/unsupported inputs and an expired outer deadline also have
controls. The initial export metadata incorrectly monic-normalized the quartic
coefficients; that failed test is preserved, and the correction precedes all
panel calls. The historical production equations were unaffected.

## Fixed calls and acceptance

- Use the same four F127 depth-four targets, four F8191 depth-four targets,
  and five F8191 depth-five targets, including the existing planted positive.
  Run only `cofactor-msolve`; do not repeat the unchanged candidate panels.
- One absolute three-second cooperative deadline includes public input
  conversion, production preparation, serialization, process launch, solving,
  rational-root extraction, signed replay and record assembly. The existing
  msolve wrapper kills the process on expiry. Noninterruptible native work can
  overrun; such a call is still undetermined.
- Retain the original 20,000-term construction limit. Each panel process has
  one worker, a 2 GiB whole-process address-space ceiling and a 60-second outer
  timeout with five-second kill grace. There are no retries or cap increases.
  Native F4's degree, pair and matrix limits are not controls on msolve; report
  that backend distinction rather than claiming a matched internal budget.
- Require the complete signed set to equal the direct reference on the full
  distinct-x, target-disjoint chart. Preserve unsupported charts and every
  failure. Also retain the projection to the Refined regular-image chart;
  compare it with the earlier fixed results only after full-set acceptance.
- Retain exact serialized inputs, preparation diagnostics and signed rows.
  Include raw RUR text up to 262,144 bytes per query; if it is larger, record
  its byte count and explicit omission instead of silently truncating it.
  This bounds the live JSONL transport. Such a record alone would not support
  independent RUR replay.
- Freeze and publish code, protocol, executable/source hashes, exact production
  source inventory and msolve identity before the calls. Rebuild after the
  adapter, original production-library and public-example checks pass.

Acceptance is a correctness result, not a cold-cost measurement. No scalar
recovery extension, new target search, EKS job or performance claim is part
of these thirteen queries. The pending all-phase comparison remains in the
[problem record](../../problems/refined-rr5-cold-comparison.md).

## Sources checked

The production `extractor.rs`, its
[quotient-product addendum](../prime31_four_summand_20261122/QUOTIENT_PRODUCT_ADDENDUM.md),
the [bounded native outcomes](PRODUCTION_RESULT.md), and the existing Dickson
msolve wrapper supply the actual construction and transport contracts.
The earlier study's Petit--Kosters--Messeng references provide the composed-map
context. This is reuse of that representation and external solver, with no
novelty claim. The KB CLI remains unavailable (query recorded in the journal),
so this is a targeted primary-source review rather than an exhaustive survey.
