#set page(paper: "us-letter", flipped: true, margin: 0.55in)
#set document(date: datetime(year: 2026, month: 10, day: 31))
#set text(font: "DejaVu Sans", size: 10pt)
#set heading(numbering: none)
#set par(leading: 0.5em)
= Split-graph factor bases: exact audit and measured F4 degree
2026-10-31 · run-20261031T201044Z · two summands · prime fields

The complete toy audit has zero mismatches over 707,472 structured and 10,404
dense assignments. All nine prospective graph-construction cells accepted.
The actual F4 handled degree drops, while four-branch row work exceeds the
same-base dense reference. The plot reports native aggregates, with all signs
included; it makes no timing or complete field-operation claim.

#align(center)[#image("comparison.svg", width: 82%)]

Source: #link("../results-summary.json")[native compact summary],
#link("../RESULTS.md")[results and evidence],
#link("../protocol.json")[frozen protocol]. L0 shared-host execution.

#pagebreak()
= Run record, comparison boundary and limits

*Requested and executed:* three tracks in parallel, followed by a PR:
complete F17 audit; matched stock neogb F4; prospective F83/F101/F127
construction at graph degrees 3, 4 and 5. F4 consumed the larger exports after
construction, retaining both low-x and hash-x controls and every affine target.

#table(columns: (1.5fr, 3fr), inset: 5pt, stroke: 0.5pt + luma(80%),
  [Original F4 flow], [14,220 attempted; 14,194 verified; 26 root-extraction assignment caps.],
  [Separate offline replay], [14,220/14,220 saved finite-field root sets verified; zero new CAS calls. Original extraction statuses remain unchanged.],
  [Observed handled degree], [max(input-generator degree, maximum selected-pair LCM degree). Not sugar or full matrix-row degree.],
  [Homogeneous theorem], [Specified top-ideal invariant d+1 versus 2d+1 under complete squarefree splitting. Distinct from measured F4 degree.],
  [All-branch work], [Toy graph secant: 2,102 reduced rows; same-base dense: 736. Matched nongraph secant: 2,522. These are component proxies.],
  [Natural toy yield], [Graph covers18/18 finite targets; control16/18. Both have56 total finite ordered relations, eight doubles and32 unordered pairs.],
  [Unavailable], [Complete field-operation cost, independent-row yield, complete cost per usable relation and qualified timing speedup remain null.],
)

*Model:* completely split graph intersection h(T)²−f(T) supplies the exact
signed factor base. The secant model retains x,z,m and all four ordinate-sign
branches. Doubles and target-equal points are included; the identity is handled
separately by inverse pairs. Offline replay proves finite-field root-set
equality by two inclusions and exact group references; it does not certify
ideal equality or multiplicities, or improve relation-discovery runtime.

*Execution and freeze:* native Rust/C; pinned stock msolve-neogb revision
`3672113ee2c696a2ac9c637636e0f7214f500ba1`, exact sparse mode2, seed315,
one thread. The bounded local exception was recorded:1 CPU/2GiB per track;
audit120s, ladder180s outer/120s internal, F4 1,800s combined/10s per child.
No paid resources. Checkpoint admission remained failed; it was not relabelled.

*Evidence:* #link("../source-lock.json")[source/binary lock],
#link("../REPRODUCE.md")[archive and replay commands],
#link("../post-data-amendment-v2.json")[post-data amendment],
#link("../review/DATA.md")[independent data review],
#link("../review/MATHEMATICS.md")[mathematics review].
Raw run and companion are retained in the deterministic, hash-addressed archive.

*Curve identities:* toy `icv1-fp5-tm1-a77cc6af`;
F83 `icv1-fp7-t5-de12415d`;
F101 `icv1-fp7-t5-b172aa85`;
F127 `icv1-fp7-tm11-1cabae54`.
Central registry/EC1 linkage and literature-wide novelty remain open.

*Prior work:* #link("https://math.mit.edu/classes/18.783/2023/LectureNotes2.pdf")[MIT group-law notes];
#link("https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf")[Petit–Kosters–Messeng, PKC2016];
#link("https://github.com/algebraic-solving/msolve")[stock msolve].
The earlier identity certificate is IDC1h621ed1a51b5f3ca8.

*Next direction:* share elimination across signs and implement exact triangular
root extraction, then prospectively compare complete same-base relation cost.
Canonical priced-pipeline dashboards receive no unvalidated cost row from
this component study. The task dashboard displays the measured comparison.
