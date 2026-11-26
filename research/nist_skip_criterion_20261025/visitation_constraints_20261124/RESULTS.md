# Index witnesses exclude two pure-visitation explanations

24November2026. The retained integer observations yield contradictions for
entire caller-model families, rather than another finite parameter scan.
The native checker consumes the existing21candidate rows and six reported
base-point attempt counts; it performs no curve arithmetic. This is producer
data/interpretation under PCC-20261124-94d371, standing unverified. [pcc:unverified]

## Fixed work after every candidate: all nonnegative costs excluded

A seed uses20successive lrand48calls. If every candidate then consumes d>=0
extra calls within a field, candidate starts are an arithmetic progression
with spacing20+d, with arbitrary initial phase. Thus20+d divides the gcd
of differences between published offsets20(i−1).

P192 published indices are139,5228,6013. Their successive offset differences
are101780and15700, with gcd20. P239 published indices2185,2320,3048 give
differences2700and14560, also with gcd20. In either field,20+d>=20 forces
d=0. But spacing20 visits the skipped eligible indices lying between those
publications. Therefore no such fixed-cost visitation-only model fits, for
any cost or warmup, even if both are allowed to differ by field.

This is stronger in scope than searching draws0–64. It does not extend to
state-dependent costs after rejected candidates, which break the arithmetic
progression assumption.

## Work after retained curves: monotone point-search costs fail

Assume every visited eligible candidate is retained and extra calls occur
only immediately after a retained curve. No hidden accepted outputs, calls
after rejects, reseeding or restarts are allowed. The next retained seed is
again on the20call grid, so intervening extra calls D must be divisible by20.
With q=D/20, missing index s after retained a requires q>=s−a, while reaching
the next retained b requires q<=b−a−1.

All observed publication-gap constraints are:

| Field | Retained a → next b | Reported point attempts at a | Required extra seed blocks q |
| --- | --- | --- | --- |
| 192 |139→5228|2|2059≤q≤5088|
| 192 |5228→6013|1|177≤q≤784|
| 239 |2185→2320|3|0≤q≤134|
| 239 |2320→3048|2|543≤q≤727|

For239bits, three point-search attempts therefore require D<=2680calls,
but two attempts require D>=10860calls to jump beyond omitted index2863.
Any nondecreasing D=f(attempts) within this field contradicts these bounds.
This excludes every fixed overhead plus fixed per-attempt draw cost, and
also any nonlinear monotone cost. The six attempt counts are existing output
from the published FreeLIP reconstruction, not newly generated point data.

P192 independently contradicts D=C+L*r with C,L>=0. Its two-attempt event
needs D2>=41180calls, while its one-attempt event allows D1<=15680calls.
But C+2L<=2(C+L), giving D2<=31360<41180. This contradiction allows real
nonnegative C,L, and hence all integer costs and any output-word scaling.

The point-cost family hypothesizes a link between these reported base-point
search costs and the curve-seed stream. FreeLIP itself has a separate saved
state; no such link is asserted as historical fact. If the reported point
search happened in a later phase, this family is inapplicable rather than
an explanation of skips.

## One shared initial burn is also incompatible

Under the retained-only model, initial warmup before the first publication
must jump over every earlier eligible skip but not that first publication.
P224requires5736–7025seed blocks; P192allows0–138. Therefore a common initial
burn across fields fails regardless of later costs. Allowing field-specific
initial burns does not repair the independent239bit point-cost contradiction.

## What changes and what remains open

Constant candidate overhead and monotone post-retention base-point cost cannot
alone produce the observed labels under these stated models. This directly
tests a possible G/b feedback mechanism: b affects point-lifting retries, but
retry-driven monotone consumption after retained curves has the wrong ordering.

The next visitation model must contain an explicit state-consuming path on
rejected candidates, an independently motivated state-dependent acceptance
operation, a restart or a hidden-output policy. These are different hypotheses;
they cannot be inserted per curve to rescue this failed model. The earlier
finite search already covers many fixed root/point-triggered paths, with zero
exact fits. Its exact source and results were reviewed, not rerun. No original
caller or decision log has been recovered, and no complete skip criterion is
established by these contradictions.

## Reproduction and failure record

The C++17 Wall/Wextra/Werror build and10second bounded integer check exit0.
The stride60positive control is accepted; feasible monotone intervals are
accepted and an incompatible pair is rejected. All source rows are retained.
Raw constraints.tsv/check.stderr retain the full intervals and witnesses.
These are producer self-checks, not independent signed verification.

First build failed Werror on misleading indentation after two continue
statements. Exact failed source/script/stdout/stderr/status remain under
evidence/attempt-001/. Splitting those statements changed no model or input.
No prior frozen source, evidence, claim or signed subject is rewritten. No
new point count, generated curve, RNG draw, scalar search, factoring, DLP,
scientific checkpoint/upload, paid job or credential change occurred.

## Visual and proof status

The [recorded interval plot](point-cost.svg) and inspected one-page
[PDF](REPORT.pdf) render the native bounds; build-report.sh and render.py
retain editable sources. The first PDF command used a duplicate relative
path and did not find its renderer; the first layout also had an overfull
source citation. Subsequent layout builds corrected those presentation issues.
The final PDF has embedded fonts and no overfull boxes. Ghostscript preserves
its vector content while reducing the report below64KiB. Raw PDF build logs
remain retained. These operations lay out existing values; they do not run a
research calculation in Python.

PROOF-MAP.md distinguishes written conditional integer arguments and native
self-checks from formal proof. Lean/lake both exit127; no kernel-checked
theorem or finite-field identity-certificate verdict is claimed for these
ordered-integer constraints. A formalization of the model implications,
not merely the small integer comparisons, remains open.

Existing rule-fit/calibration plots and the G/b comparison keep their original
counts and values. Canonical IC scoreboard, progress timeline and performance
graphs are unchanged: no solver or performance result is introduced. The new
interval plot records only the new caller-model contradiction.
