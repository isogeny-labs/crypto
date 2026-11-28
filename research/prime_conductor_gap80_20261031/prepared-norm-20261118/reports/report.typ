#set page(paper:"a4",margin:16mm,footer:context align(right)[#counter(page).display()])
#set text(font:"DejaVu Sans",size:10pt)
#set par(leading:0.5em)
#let s=json("../summary-final.json")
= Reusing native log-norm preparation
18 November 2026 · native development component · draft PR 4723

The optional `--prepared-norm` path prepares the fixed baby polynomial once
per block and consumes native FLINT jets. It retains the existing formulas,
collision/coverage gates, target inputs and default textual backend.

*Observed correctness:* #s.prepared_pipeline_replays pipeline replays,
#s.point_input_evaluations point-input evaluations and #s.pipeline_fields
fields agree with the old output and independent PARI references under the
explicit reuse policy below. Default bytes match; #s.negative_cli_controls
CLI rejection controls pass. No performance session ran. [pcc:unverified]

#image("reuse.svg",width:100%)

The two bars count different operations. They describe context reuse; elapsed time, field-operation costs and
end-to-end accounting remain unmeasured. Counts include repeated
and exceptional inputs. This is one development replay per displayed cell.

== Exact arithmetic contract
For each monic h, cache its reversed-series inverse and trace weights from
rev(h′)/rev(h). With u=g₁/g₀, v=g₂/g₀, w=g₃/g₀ in Fp[X]/h, trace

  u,     v−u²/2,     w−uv+u³/3.

The native context caches inverses 2/3 and reusable scratch. gcd still
rejects noninvertible g₀. h is borrowed immutable and outlives the context.
The existing validated prime field has characteristic greater than 3.
No new mathematical identity is asserted.

== Memory and time scope
Two size-dependent cache polynomials and scratch persist during later tile
builds. Logical storage/RSS, full-window feasibility, setup, I/O and actual
runtime remain unmeasured. The diagnostic counts exclude scalar walks and
all arithmetic inside a norm. The complete 80-bit quotient and non-kernel
images are absent; the optional backend does not close that obligation.

#pagebreak()
= Controls, provenance and limits
#table(columns:(1fr,1.65fr),inset:5pt,stroke:0.4pt,
 [*Control*],[*Outcome*],
 [Primitive native oracle],[768 direct-root/reuse evaluations, 384 collisions and six invalid moduli; 96 contexts across three fields including actual 96/202-bit fields],
 [Pipeline panel],[Seven existing schedules × affine/7/64; four fields, including the 96-bit degree-5 map],
 [Independent oracle],[Degree-10009 reuses completed same-input PARI result; smaller degrees use fresh PARI],
 [Qualifier A],[300-second cap reached; exit 124 retained],
 [Qualifier B],[Same cap; exit 0 with reference reuse; source/binary unchanged],
 [Builds],[Native CLI/checker pass with warnings denied; image not built/published],
)

The default fixture includes unchanged tiny known-answer recovery controls.
No new ECDLP method, scalar timing, index-calculus adapter or admitted cloud
session is supplied by this component. Large control degree is not a
conductor-gap certificate. [pcc:unverified]

== Retained failures and corrections
Repeated fresh reference expansion exceeded the first whole qualifier cap.
The completed same-input degree-10009 oracle is reused by the second
qualifier; its output reference flag stays false without a new PARI call.
All quotient/images/inputs and existing counters still compare.

Frozen shell functions accumulated label suffixes in filenames. Actual
native metadata confirms all seven groups used modes 0/7/64. Raw names
stay unchanged; a separate current driver fixes labels without being
claimed as the producer of these results.

An initial process interpretation omitted state; the explicit check showed
an already defunct child after the timeout. No active post-cap computation
is established. The earlier summary wording is retained and corrected
additively in summary-final.json and PCC-20261118-2521f3. [pcc:unverified]

== Evidence and original requirements
Source/input/binary and qualifier manifests match before/after. Full raw
outputs, initial timeout and rejected options are archived deterministically;
named proof subjects stay loose. Historical source and executables remain
separate. Producer standing is unsigned/unverified, verdict not_covered.
The PDF and graph are presentation of recorded counts only. [pcc:unverified]

The original minimum 80-bit ordinary prime-conductor crossing still needs
its lower endpoint, complete evaluated map and subgroup/conductor evidence,
then repeated paired scalar/ECDLP/IC measurements with the required cloud,
reference and cost admission. The separately certified 4774-bit Frobenius
conductor remains a distinct arithmetic result. No record or timing is
promoted here. Earlier S3 publication remains blocked; PR stays draft.
[pcc:unverified]

Sources: RESULTS.md, immutable README/CONTINUATION scopes, existing native
optimized.c/direct-root oracle and FLINT reference/installed 3.1.3 headers.
KB unavailable, wrapper exit 127. Exact receipts and reproduction are linked
in RESULTS.md. No target or benchmark job is active.
