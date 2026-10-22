#set page(paper: "a4", margin: 17mm)
#set text(font: "Liberation Sans", size: 9pt)
#set heading(numbering: "1.")

= Twist symmetry in quartic extension index calculus
22 October 2026. Full source-linked record: `QUARTIC_TWIST24_REPORT.md`.

== Question and protocol
We tested a transported Frobenius on a quadratic twist over $F_(p^4)$, combined it with the cubic $j=0$ automorphism, and measured both an unchanged physical factor base and the union of two invariant abscissa lines. Seven positive toy curves, three seeds, one public target $[271]G$ per run; the $p=13$ empty-base control is retained. The signed four-factor oracle, three matrix encodings, and full 24-fold rho reference are native Rust. The frozen protocol and raw records are in the same study directory.

The construction uses $F=F_p[t]/(t^4-d)$ with $d$ the least nonsquare, $E: y^2=x^3+b_0 t^3$, $h=t^((1-p)/2)$, $omega^3=1$, and
$psi(x,y)=(h^2 x^p,h^3 y^p)$, $iota(x,y)=(omega x,y)$.
The norm identity $h^(1+p+p^2+p^3)=-1$ gives $psi^4=[-1]$. The two maps commute. On the certified prime-order group their checked eigenvalues have orders eight and three, yielding order 24 on signed points and 12 on nonzero abscissae. Thus a stable base with $B$ abscissae has $B/12$ unknown logs, compared with $B/3$ for the cubic automorphism alone.

For the four lines $t^j F_p$, $psi$ multiplies abscissae by $z^(j-1)$, $z=t^(p-1)$. Lines 1 and 3 cannot contain a nonzero odd-order point: $psi$ or $psi^2$ would fix its abscissa and hence act as either plus one or minus one, contradicting $psi^4=[-1]$. The complete sweep found only lines 0 and 2 usable, with line 0/2 counts shown below.

#table(columns: (0.7fr, 1fr, .7fr, .7fr, 1.2fr), inset: 4pt,
  [$p$], [$r$], [line 0], [line 2], [24-fold columns 0→0+2],
  [13], [28,753], [0], [0], [none],
  [37], [1,873,633], [36], [12], [3→4],
  [73], [28,408,033], [48], [48], [4→8],
  [97], [88,538,353], [36], [36], [3→6],
  [157], [607,561,153], [108], [84], [9→16],
  [229], [2,750,120,881], [96], [96], [8→16],
  [241], [3,373,515,841], [96], [96], [8→16],
  [337], [12,897,718,513], [180], [180], [15→30],
)

#pagebreak()
== Fixed physical base: what the fold changes
The same signed points and residual/decomposition stream feed flat, cubic-only and combined matrices. All 63 IC solves and 1,008 rho walks recovered $271$. Independent native replay reconstructs 1,821 selected relations, all row encodings and matrix spans. The full A/A rerun matched every non-timing field.

#table(columns: (.55fr, 1fr, 1fr, 1fr, 1fr), inset: 4pt,
  [$p$], [cubic/twist queries], [cubic/twist row ops], [cubic/twist $M$], [twist/rho24 $M$],
  [37], [3.27], [10.78], [0.998], [55.5],
  [73], [2.57], [11.24], [1.49], [112.9],
  [97], [2.96], [12.51], [2.66], [192.9],
  [157], [3.70], [11.03], [2.49], [201.9],
  [229], [3.48], [14.79], [3.18], [404.6],
  [241], [3.81], [13.30], [3.80], [376.8],
  [337], [3.33], [14.45], [2.99], [402.1],
)

#image("quartic_twist24_cost_v2.png", width: 100%)

$M$ is recorded base-field multiplications on each path with its relevant model, input, eigen, base, map, table, stream and verification work. Row operations are a separate unit. The 64-query batch floor erases the field-multiplication improvement at $p=37$. The rho comparison is same-target and includes its 24-fold endomorphism; the large gap is a partial-cost diagnostic, not an admitted speed ratio.

Applying an endomorphism to a found relation transforms the target and multiplies both the folded row and its right side by the eigenvalue. The audit verifies this covariance on every selected relation. Orbit images are not independent rows. Therefore the unchanged physical base gains relation *utility*, not raw hit probability.

#pagebreak()
== Two invariant lines: actual relation yield
For each curve and seed, $L_0$ and $L_0 union L_2$ receive the exact same target stream, with identical $L_0$ points and signs. The independent native oracle recomputed every one of 69,045 query hit counts using a fully signed two-pair hash, re-added 5,298 selected group relations, and checked all 126 matrix solves. The producer recorded 7,275 decompositions. Both full runs matched on every non-timing field. Medians are over three paired, separate one-target runs.

#table(columns: (.55fr, .8fr, .8fr, .8fr, 1.35fr), inset: 4pt,
  [$p$], [union/single hit yield], [single/union queries], [single/union row ops], [single/union counted $M$],
  [37], [2.04], [1.67], [0.61], [0.57],
  [73], [8.82], [4.82], [0.30], [0.26],
  [97], [14.60], [6.73], [0.30], [1.01],
  [157], [5.68], [3.72], [0.34], [0.63],
  [229], [17.55], [11.07], [0.33], [2.48],
  [241], [14.85], [7.08], [0.23], [1.49],
  [337], [11.28], [5.08], [0.26], [1.24],
)

#image("quartic_mixed_lines_v1.png", width: 100%)

The union improves same-target hit yield on every curve and lowers query count, but doubles or nearly doubles matrix columns and increases row work. Counted field multiplications improve in all three seeds at $p=229,241$; they worsen at $p=37,73,157$ and straddle one at $p=97,337$. The figure keeps the row-operation and field-multiplication units distinct. This is a relation-yield result with a cost tradeoff, not a uniform whole-method win.

#pagebreak()
== Interpretation, limits, and provenance
The combination of Frobenius-invariant factor bases and matrix-orbit savings is prior work (Galbraith–Granger–Merz–Petit). GLS twist endomorphisms are prior work (Galbraith–Lin–Scott). This study makes a specific canonical quartic construction and paired mixed-line measurement explicit. An earlier repository E18 run already used the cubic $j=0$ fold; this panel provides the same-base cubic control. The earlier repository E2b GLS experiment implemented an $S_3$ Weil-descent resultant oracle. The main quartic relation panel uses exact four-sum meet-in-the-middle; a separate six-query pilot runs the native $S_5$ solver.

The experiment is L0 on a shared Intel Xeon 6975P-C host. The main panel does not emit IC-stream group additions, hash/table operations or a calibrated group-addition-equivalent band. Its field-multiplication path projection excludes work done solely for the other two matrix arms. Whole-process wall times were not isolated and are not used for a speed claim. A single-arm sealed run and full polynomial relation stream compared with msolve are needed before an end-to-end cost or asymptotic claim. The four one-dimensional Frobenius eigenspaces were exhausted. A separate full census tested the two-dimensional space $F_p+t^2 F_p$; other invariant subspaces and curve models remain open.

The field/twist proof above is handwritten and computationally checked. `QuarticTwistSymmetry.lean` proves conditional consequences (fourth-to-eighth iterate, composed eigenvalue, row covariance) with an audited Lean kernel. It does not formalize the concrete finite field or twist isomorphism. The full proof map and source links are in `QUARTIC_TWIST24_REPORT.md`.

Evidence: `quartic_twist24_full_v2.ndjson`, `quartic_twist24_native_audit_v2.json`, `quartic_twist_mixed_v1.ndjson`, `quartic_twist_mixed_native_audit_v1.json`, `quartic_twist_mixed_summary_v1.json`, the frozen protocol, and `OPEN_PROBLEMS.md`. Write-once run reports with graphs and PDFs are under `reports/`. All artifacts are in this study directory.

#pagebreak()
== Polynomial pilot and larger invariant space
The registered native $S_5$ pilot ran one successful and one unsuccessful line-0 residual at each $p=37,73,97$. All six polynomial outputs exactly matched the signed group oracle; every returned quadruple passed group addition. Each curve's symmetrisation precomputation used 1.226 billion $F_p$ multiplications. The same degree-29 Macaulay matrix shape appeared on all six solves: 41,780 rows and 40,920 columns. The plot shows counted solve multiplications; shared-host L0 solve times were 49–58 seconds and are not isolated speed evidence.

#table(columns: (.5fr, 1fr, 1fr, .9fr), inset: 4pt,
  [$p$], [hit query / $10^12 M$], [miss query / $10^12 M$], [oracle agreement],
  [37], [0 / 1.158], [1 / 1.159], [2/2],
  [73], [25 / 1.203], [0 / 1.189], [2/2],
  [97], [14 / 1.211], [0 / 1.197], [2/2],
)

The two-dimensional invariant abscissa space $F_p+t^2 F_p$ was exhaustively scanned on all eight curves. Independent norm-character and point-orbit replay checked all 265,008 candidate abscissae. Every nonempty abscissa orbit has size 12. The formerly empty-line $p=13$ curve has 72 liftable interior abscissae and six folded columns. At $p=337$, the direct exact-oracle pair construction would generate 3.178 billion signed entries; this count does not measure allocated memory.

#image("reports/run-20261022T0621Z-quartic-2d-census/space_census.png", width: 100%)

On the $p=13$ two-dimensional base, a separately frozen one-target exact-oracle IC pilot processed seven queries, found 4,041 decompositions, selected seven verified relations, reached rank seven, and recovered $271$. Independent replay recomputed all query counts and rows; the full A/A rerun matched every non-timing field. This shows that the larger invariant base revives the toy empty-line case. It is not a polynomial-system or rho speed result.

#grid(columns: (1fr, 1fr), gutter: 8pt,
  image("reports/run-20261022T0616Z-quartic-s5-p37/s5_cost.png", width: 100%),
  image("reports/run-20261022T0626Z-quartic-2d-p13/p13_relations.png", width: 100%),
)

The complete measurements, per-curve S5 records, two-dimensional census, native audits, p=13 run, figures and write-once run reports are linked from `QUARTIC_TWIST24_REPORT.md`.

#pagebreak()
== Twist symmetry in the relation-finder pair table
On the same p=13 two-dimensional base, the signed two-factor table has 5,112 entries modulo global sign. The 12 abscissa actions partition it into 426 orbits, each of size 12. The direct quotient stores one pair representative per orbit under 350 canonical sum-abscissa keys. It reconstructs transformed pairs when queried. On every one of the seven previously recorded residuals, its *complete signed four-factor decomposition set* equals that of the full pair table; all 4,041 decompositions and seven selected IC relations remain available. A full A/A rerun matched every non-timing field.

#table(columns: (1.2fr,1fr,1fr,1fr,1fr), inset: 5pt,
  [table arm], [stored pair records], [setup $F_p M$], [seven-query $F_p M$], [setup group ops],
  [full], [5,112], [495,864], [7,029,782], [5,112],
  [orbit quotient], [426], [544,746], [96,036,422], [426],
)

#image("reports/run-20261022T0644Z-quartic-pair-quotient/pair_quotient.png", width: 100%)

This exact symmetry quotient reduces stored pair *records* by 12 times and setup group additions by 12 times. It does not measure allocated bytes; action maps and hash buckets also consume memory. Its canonical lookup uses 13.66 times the query base-field multiplications of the full table, while query group-operation counts are equal. Therefore the tested direct quotient preserves relation yield and trades arithmetic for a smaller representative set. It is not a relation-lookup speed improvement on these targets. A tagged orbit index might alter the tradeoff but has not been tested.

Raw records: `quartic_twist_pair_quotient_p13_v1.json` and its A/A copy. Source: `examples/quartic_twist_pair_quotient_p13.rs`; the frozen protocol, full run report and exact operation counts are linked in `QUARTIC_TWIST24_REPORT.md`.
