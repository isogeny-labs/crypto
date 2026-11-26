# Offline audit of the retained F127 parametrizations

2026-11-25. Frozen diagnostic protocol; no solver invocation.

## Question and sources

The liftable-fiber panel's cofactor record for F127 target(41,95) returns an
empty signed answer, while the previous factorwise-extraction panel returns
all13 reference rows. Their prepared equations, serialized input and fixed
form are byte-identical; the RUR bytes differ. Both pass the existing
eliminant-degree/dimension check. Preserve both records and the stopped
panel's missing suffix.

Sources checked (KB): status and searches for "msolve rational univariate
parametrization coordinate normalization denominator derivative" and "RUR
equation residual mismatch finite field extraction" fail with wrapper exit127;
no KB MCP is loaded. Read the actual parser/adapter and SEPARATING_FORMS.md,
the installed msolve0.7.5 tutorial sections on parametrization and genericity,
upstream README (Git blobe2a3eefe8cdfecf2d71f3314052cf5d2b318846b), and
Sage's `src/sage/rings/polynomial/msolve.py` (Git
blob20c9ed17f274d9096d707feae19885dcda0d704c).

The [upstream tutorial](https://msolve.lip6.fr/downloads/msolve-tutorial.pdf)
specifies increasing-degree coefficient arrays and coordinates -v_i/den,
with the last variable as parameter. The installed prime-field convention
has den=1. The [Sage parser](https://github.com/sagemath/sage/blob/develop/src/sage/rings/polynomial/msolve.py)
also decodes -v_i(root)/den(root), appending the root as the last coordinate.
It rejects certain small-characteristic degree failures but does not thereby
establish semantic correctness of every returned coordinate. The
[README](https://github.com/algebraic-solving/msolve/blob/master/README.md)
requires each monomial to occur once in each input polynomial; duplicates
have undefined input-parser behavior. These sources guide the checks below.
Their current metadata is not a source-level diagnosis of the installed solver.

## Frozen diagnostic

Add a standalone Rust example `rur-audit` that reads the following two original
JSONL files, selecting exactly the cofactor record for F127 depth4 target(41,95):

1. `evidence/factorwise_extract_v1/f127_d4.jsonl`, prior successful record.
2. `evidence/liftable_fibers_v1/f127_d4.jsonl`, new mismatching record.

It receives no solver path, private logarithm or replacement output. Parse the
recorded input prefix and fixed form, requiring distinct variable names,
known variables, canonical nonnegative monomials and no repeated monomial.
Parse the RUR with the same documented coordinate convention, validating
declared degrees, names, characteristic, scaling and numerator count. Enumerate
the eliminant's roots in F127 for this offline diagnostic, decode every
coordinate (including cofactors and parameter), and evaluate every serialized
equation and the exact fixed form. Record all roots, assignments, denominator
values and equation/form residuals, plus distinct-root and dimension counts.
This is direct finite validation of retained outputs, not a relation oracle,
independent solver or general algebraic-closure certificate.

Predicted control: the old successful output should satisfy its equations
and form; the mismatching output may violate this contract or reveal that
the issue lies later in extraction. Keep whichever outcome is observed.
If both satisfy all input equations, diagnose extraction next. If either
fails, identify the precise residual and preserve that output; a matching
degree alone is not its semantic certificate. No retry or format change is
preselected from the failure.

Before reading the actual records through the new auditor, tests use a tiny
synthetic two-root parametrization: correct positive, a coordinate mutation,
malformed polynomial metadata and repeated/unknown input monomials. Publish
source, tests/build, and a separately frozen executable/input identity.
Then run the two-record audit once with one worker,1GiB address space and
30 seconds plus five seconds termination grace. It emits one descriptive
record for each input; invalid residuals are recorded outcomes, not hidden by
an early exit. Parsing failures remain explicit and make the command fail.
No new msolve call, solver rebuild, source guard change, recovery draw or
EKS/paid execution belongs to this component.

Retain raw output, exit/stderr, both original input hashes, executable/source
identity, checksums, unsigned claims, report/figure/PDF and reproduction.
No historical file, successful record, mismatch or frozen acceptance rule
is edited. Future enforcement or solver changes require separate qualification.
