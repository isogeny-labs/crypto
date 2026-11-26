# Cofactor output controls: partial-RUR rejection and bounded forms

**2026-11-24 · Local development evidence · [pcc:unverified]**

The harness now exports the exact normalized Isogeny Labs cofactor equations
to msolve and rejects a captured incomplete rational univariate representation
(RUR). With a fixed parameter coordinate, all eight degree-16 controls return
the full signed reference sets. All five degree-32 controls still time out.
Nine Refined regressions pass on their narrower regular chart. This closes an
output-acceptance defect; it does not establish a fully charged cost advantage.

![The full cofactor and regular-image arms keep distinct output scopes.](cofactor-output.pdf){width=90%}

## What changed, and what the guard establishes

`prepare_cofactor` stops the existing production extractor at its pre-F4
boundary. It exports the same normalized equation vectors passed to native F4,
their nonzero normalization multipliers, and the **unscaled** RR5 quartic
coefficients. The adapter maps native `(b,c,d)` to `(t0,a1,a0)` and serializes
those equations. Native F4 and its previous allocation caps remain unchanged.
The [production source inventory](evidence/separating_v1/production_source_manifest.json)
records the exact dependency closure.

The first added binding test caught an export-only error: making each quartic
coefficient polynomial monic changes the quartic. The corrected export keeps
raw coefficients; scaling a whole zero equation remains valid. The initial
[failed test](evidence/cofactor_msolve_v1/initial_export_test_failure.txt) is
retained. The corrected binding test checks 54 scalar assignments across both
toy fields against the independently implemented numeric quartic and every
cofactor coefficient equation. [pcc:unverified]

The initial external panel returned 9 correct rows, then 7 of 10 on F127 target
(116,69), and exited 1. A separately frozen diagnostic returned 8 of 10. Its
[captured RUR](evidence/cofactor_diagnostic_v1/f127_d4.jsonl) reports quotient
dimension 10 but an eliminant of degree 9. A single-valued degree-9
parametrization cannot encode ten distinct points. The parser now requires
eliminant degree equal to reported quotient dimension before extraction.
This is conservative: complete reduced descriptions of nonradical ideals
may be refused. Equality alone is not a certificate of algebraic completeness;
the toy controls still compare whole signed sets with an independent reference.
[pcc:unverified]

\newpage

## Fixed parameter choices and observed outcomes

The new [protocol](SEPARATING_FORMS.md) appends a coordinate and equation

\[
A=b+jc+j^2d,\qquad j\in\{0,\ldots,15\}.
\]

For every original solution there is exactly one such A, and projection
recovers the original solution. This extension preserves the solution set;
it supplies no guarantee that any of the sixteen forms separates it.
`msolve -c 0` disables automatic genericity repair. The adapter advances only
after a degree/dimension mismatch or the exact genericity-failure message
established by the retained [two-point fixtures](native/fixtures/).
Other errors stop the query. Preparation, all attempts, extraction and replay
share one three-second deadline; partial outputs are never combined.

Source **3176dae78f371c9b5f4357f74835c161ecf52ce1** and the executable were
published/frozen before calls. Each panel used one worker, 2 GiB address space,
and a 60-second outer timeout with five-second kill grace. Every cofactor
query attempted only j=0: degree-16 queries completed, and degree-32 queries
reached their deadline. No panel was retried or granted a larger cap.
[pcc:unverified]

| Fixture / selector | Base size | Full cofactor exact | Full-reference row counts | Refined exact / regular rows |
|---|---:|---:|---|---|
| F127 / degree 16 | 8 | 4 / 4 | 9, 10, 13, 13 | 4 / 4; 1, 0, 3, 3 |
| F8191 / degree 16 | 7 | 4 / 4 | 0, 1, 0, 0 | Not rerun in this protocol |
| F8191 / degree 32 | 14 | 0 / 5 | 1, 5, 2, 3, 4 | 5 / 5; 0, 0, 0, 0, 1 |

The last row's full-reference counts come from test-only direct enumeration,
not from a complete cofactor answer. All five cofactor outcomes are
`undetermined`; each attempt records `msolve timeout`. Every panel process
exited zero because these incomplete outcomes are valid records. Exit status
alone does not indicate query completion. All accepted cofactor RURs fit the
262,144-byte cumulative per-query capture cap and are retained. [pcc:unverified]

[F127 records](evidence/separating_v1/f127_d4.jsonl) ·
[F8191 degree-16 records](evidence/separating_v1/f8191_d4.jsonl) ·
[F8191 degree-32 records](evidence/separating_v1/f8191_d5.jsonl) ·
[Before-call identity](evidence/separating_v1/pre_call.txt) ·
[Exact commands](evidence/separating_v1/commands.txt)

The original native-F4 panel's thirteen allocation refusals remain in
[PRODUCTION_RESULT.md](PRODUCTION_RESULT.md). The first external qualification
and diagnostic also remain unchanged. Eleven queries from that original
thirteen-query qualification were never run after its mismatch; the new
protocol is separate evidence. No earlier failure is reclassified as success.

## Validation standing and remaining work

The final feature suite has 15 passing tests; the default suite has 6.
The preceding qualified export passed the original production library's 32
tests and 8 public-example tests; their source has not changed since.
Tests cover the retained partial RUR, parameter binding, signed replay,
expired deadlines, and stopping on an unrelated solver error after one known
genericity failure. [pcc:unverified]

Claims `PCC-20261124-b0e9a1`, `PCC-20261124-94df17`,
`PCC-20261124-b40e9d`, and `PCC-20261124-96339c` retain unsigned standing.
The earlier failures are claims `PCC-20261124-0eec2a` and
`PCC-20261124-6f4565`. See [test and build receipts](evidence/separating_v1/),
[reproduction instructions](COFACTOR_OUTPUT_REPRODUCE.md), and
[development limits](DEVELOPMENT_CHECKS.md). No completed root-suite check,
independent signed replay, or admitted cost measurement is claimed.

The [open comparison problem](../../problems/refined-rr5-cold-comparison.md)
still requires the complete four-leaf oriented-S3 control and a recovery
comparison charging cold setup, failed queries, extraction, rank and linear
algebra, and final scalar verification. The eight positive controls establish
only a local full-output boundary for those fixtures. The degree-32 boundary
is unresolved. This component supplies no new isogeny construction or
general-complexity theorem. Canonical cost charts and leaderboard entries
remain unchanged because there is no admitted cost result. No EKS job ran.
The local streaming spool contains 215 records; external delivery awaits the
user's receiving HTTPS URL. [pcc:unverified]
