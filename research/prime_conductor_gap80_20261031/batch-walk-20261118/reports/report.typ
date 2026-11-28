#set page(paper: "a4", margin: (x: 15mm, y: 14mm),
  footer: context align(right)[#counter(page).display()])
#set text(size: 10pt, font: "DejaVu Sans")
#set par(justify: false, leading: 0.55em)
#set heading(numbering: none)
#let s = json("../summary.json")
#let c = s.arithmetic_controls

= Bounded projective walks
Native evaluator development evidence · 18 November 2026 · PR #4703

The complete evaluator now has an optional bounded projective-coordinate
backend for baby and giant walks. Its default remains affine. The existing
coverage journal, quotient/image formulas, kernel inputs and direct-PARI cap
remain in force. This is an implementation and correctness report; no runtime
advantage is measured.

== What completed
#table(columns: (3fr, 2fr), inset: 5pt, stroke: 0.4pt,
  [*Recorded native check*], [*Outcome*],
  [Projective small-pipeline replays], [#s.pipeline_projective_runs],
  [Point-input evaluations, three supplied fields], [#s.projective_point_input_evaluations],
  [Mixed additions checked against PARI], [#c.mixed_add_pari_pairs],
  [Small walk samples / expected infinity rejections],
    [#c.walk_samples / #c.walk_infinity_rejections],
  [202-bit target-field x-coordinate comparisons],
    [#s.wide_field_coordinate_control.total_x_comparisons],
  [Invalid batch-option controls], [5 rejected, exit 2],
  [New target quotient / images / ECDLP timings], [None],
)

The projective pipeline covers six schedules over p=6666679, 71 and 11, with
batch caps 1, 7, 64 and 4096. Quotient coefficients, images and original stage
counters agree with the baseline; new affine outputs and default fixture bytes
also agree. Counts include repeats, infinity and kernel points. They are not
counts of distinct non-kernel images or conductor crossings. [pcc:unverified]

== Coordinate contract and costs
Finite Jacobian coordinates represent x=X/Z² and y=Y/Z³. Mixed additions use
EFD madd-2004-hmv; doubling uses general-a dbl-2007-bl. Infinity, inverse pairs,
doubling and y=0 doubling have explicit branches. A walk containing infinity
is rejected. Input fields must have characteristic greater than 3.

Three GMP-integer vectors retain X, Z and prefix products for at most CAP
points. A backwards pass normalizes using the inverse of the accumulated
nonzero Z product. Original emission order is preserved. CAP is restricted
to 1–65536; it is not a total memory cap. The vectors' limbs, scalar starts,
root/product arrays, jets, log norms and serialization remain separate costs.
The leftover loop stays affine. Extra multiplications and squarings are not
priced by the inversion counter.

== Actual target field, bounded primitive scope
The additional native control compares 4096 baby-walk positions and 4096
giant-walk positions at four batch caps against direct PARI additions over the
frozen 202-bit field. The repeated checks total 32768; this is a prefix of
coordinate walks only. Its 120-second process deadline is a ceiling, not an
elapsed-time result. It performs no target isogeny construction. [pcc:unverified]

#pagebreak()
= Recorded normalization calls
#image("normalization.svg", width: 100%)

*Boundary:* walk-local batch inversion calls, one development replay per cell.
These integer counters have no timing uncertainty interval. The graph compares
batch caps of the projective backend; it is not a runtime or speedup plot.
The block-1 control has no affine recurrence additions to eliminate, yet still
performs normalization inversions. [pcc:unverified]

== Evidence and verification status
PCC-20261118-3560cd binds the summary, both pre-call manifests and the arithmetic
and target-field raw controls. Its statement standing is *not_covered* and
unverified: no signed independent or execution-proof verdict is supplied.
The separately copied frozen qualifier was hashed before calls and was not
edited. The future template now refuses an existing output directory before
writing; the executed historical script retains its original late sentinel.

Source pins: evaluator 61e26c0e403e…; helper 298849c0341b…;
candidate binary 4256b649dba8…. Full hashes, raw output, negative controls and
completion exits are in the companion study. The older window audit stays
pinned to its pre-batch source, not this new file. [pcc:unverified]

== Remaining obligations
The full 80-bit ordinary prime-conductor crossing still needs its quotient,
evaluated non-kernel images, endpoint/subgroup certificates and matched scalar
and ECDLP timing trials. The IC comparison remains separate. The worker build
input follow-up in PR #4690 copies and hashes the new helper; it does not
reassign the older image receipt to this source. A new tested image, registry
digest, budget/reservations/capacity and admitted cloud run remain open.

No wall-clock or peak-RSS result is reported. Cargo and the conductor session
tool remain unavailable in the executor. Native development controls do not
replace remote merge checks. No scientific leaderboard or performance graph
is promoted by these counts.

*Primary source:* https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html.
The pinned general-a page and selected formulas are recorded in sources.json.
KB unavailable (wrapper exit 127); no novelty or world-record claim is made.
