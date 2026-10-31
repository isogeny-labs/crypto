#set page(paper: "a4", margin: 1.7cm)
#set text(font: "DejaVu Sans", size: 9pt)
#set heading(numbering: "1.")
#set table(stroke: 0.4pt, inset: 5pt)
= Sixteen factor-base experiments with a degree audit
2026-10-31 · Research design and native certificate tooling

The catalogue defines sixteen constructions, their yield hypotheses, algebraic membership systems and falsification experiments. The native audit verifies an upper refutation identity at degree D and a lower dual certificate at D−1. Seven correctness tests pass, including corrupt-certificate checks and all 256 pairs of two-variable generators. These are tooling tests; factor-base measurements await execution admission.

The primary curve is `icv1-f2m83-tm6151469093347-debefd74`, with equation $y^2+x y=x^3+1$, field modulus $z^83+z^45+z^2+z+1$, subgroup order `2417851639230796216685689` and cofactor 4. Field degree and subgroup size are distinct. The separate degree-83 model with a 53-bit subgroup remains a diagnostic.

#figure(image("screen.svg", width: 100%), caption: [Proposed screen and current tooling checks. Source: `PROTOCOL.md`, `DEGREE_PROOF.md`, and `native/degree_certificate.rs`. No new factor-base yield or regularity measurement is plotted.])

== First tranche and algebraic corrections

Start with shallow Frobenius forests, trace-clean forests, quadratic trace slices inside a forest, and bilinear tensor images. These test whether reducing each component's dimension or imposing cheap nonlinear membership can improve verified yield together with certified degree.

The Artin–Schreier image of an affine F2-space is another affine space; keep it as an equal-set control. A quadratic input representation does not establish quadratic solving degree. Odd-weight normal-basis x coordinates have trace one and cannot directly describe odd-order subgroup points on the primary a=0 curve. Use even weights or explicitly encode a separate projection. Arbitrary trace-form coefficients do not individually preserve Frobenius invariance. The simple invariant trace family alone does not justify a roughly 16-bit base from the full 83-bit field.

#pagebreak()
== Sixteen proposed families

#table(columns: (0.4fr, 2fr, 3fr),
  [ID], [Family], [Discriminating experiment],
  [1], [Shallow Frobenius forest], [Hold seed entropy and admitted K fixed while sweeping component depth; measure the global selector and all fixed-tag branches.],
  [2], [Trace-clean forest], [Match post-filter subgroup points against family 1; test actual conditional yield rather than assuming a factor two.],
  [3], [Low product-span forest], [Compare geometric and random seed spaces at fixed d and K; include complete chain degree and product-space dimension.],
  [4], [Shifted affine forest], [Freeze shifts by density/intersection statistics, then compare against unshifted spaces on disjoint targets.],
  [5], [Quadratic trace slices], [Restrict a small ambient space with trace quadratics; compare equal-cardinality random subsets and exact membership systems.],
  [6], [Artin–Schreier image control], [Deduplicate against the directly constructed affine image; measure both encodings of the same set.],
  [7], [Bilinear tensor image], [Use c+u+v+λuv; count image fibers, compare direct and auxiliary encodings, and retain every selector.],
  [8], [Reciprocal forest], [Exclude poles and compare exact point sets, with the established β=0 identity control.],
  [9], [Rational torsion coordinate frame], [Use the existing symmetrised S3/S4 representation on a forest; measure denominator exceptions and complete selector cost.],
  [10], [Even-weight normal-basis necklaces], [Compare exact weight circuits and lookup encodings of identical admitted subgroup points.],
  [11], [Cyclotomic sparse sums], [Verify order-167 construction and compare position selectors, weight counters and lookups; prior yield neutrality is retained.],
  [12], [Phase-staggered typed slots], [Measure unique sums against plain and union-domain controls; no automatic factorial yield multiplier.],
  [13], [Collision-optimized orbit code], [Choose K columns using group-sum collision energy, freeze them, then evaluate disjoint natural targets.],
  [14], [Joint bipartite 3+2 domains], [Optimize two pools jointly at fixed combined K; measure unique support, complete joins and algebraic degree.],
  [15], [Small core plus residual envelope], [Measure full rows after graph cycles, duplicates and rank filtering; partial hits alone are not usable yield.],
  [16], [Dimension/slack and arity co-design], [Sweep dimension and arity independently, then test the joint yield-adjusted work frontier.],
)

All entries are proposed experiments. Prior negative results and controls are linked in `CANDIDATES.md`; this catalogue makes no universal novelty claim.

#pagebreak()
== Certificate and experimental contract

For Boolean generators $f_i$, construct bounded rows $m f_i$ with formal degree $deg(m)+deg(f_i) <= D$, then reduce using $x_j^2=x_j$. An upper certificate is a combination of those rows equal to 1. A lower certificate is a dual vector annihilating every row at D−1 and evaluating to one on the constant column. Together they certify the exact bounded-Macaulay refutation degree for those generators.

This metric is distinct from first fall degree, productive degree in an iterative solver, largest matrix degree, final basis degree and homogeneous regularity. The tool checks the stated identities and truth table. Full Gröbner and uniform regularity claims have additional obligations in `DEGREE_PROOF.md`.

Stage A exhausts small registered E0 instances at degrees 5, 7 and 9. Stage B certifies degrees on their complete target systems. Stage C uses frozen training seeds 315/316 and holdout seeds 1315/1316 on larger controls, retaining exact references wherever feasible. Stage D builds and verifies the primary degree-83 bases at K=512 and K=600, with five summands and deterministic planted/unplanted panels. Stage E evaluates usable rows, cycles and rank.

Every arm retains equation/base/target hashes, complete tuple and branch coverage where asserted, candidate and survivor counts, field/group and Boolean operations, matrix widths, memory, disk traffic, and exact replay. Cap exits remain UNKNOWN. Promotion needs zero reference disagreements, nonzero natural relation yield, a certified degree or width/work gain, and a holdout improvement in yield-adjusted work and storage. The structured degree-at-most-five target at degree 83 is an acceptance condition.

== Sources and execution status

Read: Semaev, #link("https://arxiv.org/abs/1504.01175")[arXiv:1504.01175v1], sections 2–4; repository degree campaign `research/factor_base_dor_campaign_20261013/REPORT.typ`; shape results `research/shape_search_g1_g6_20261015/README.md`; degree/shape notes; factor-base encyclopedia C1–C11 and D1–D10; and current `solving_degree` API documentation. The original ePrint 2004/031 PDF returned HTTP 403; its full text was not read. Further primary-paper and sibling lead-atlas review remains owed before promotion.

The worker and native toolchain are ready. GitHub authentication rejects the injected token. The worker has no configured AWS/outbound identity or EKS MCP. Repository instructions require a durable S3 resume checkpoint before research execution; its upload and remote submission access are unavailable. Local correctness tests are complete. No research job has been submitted. The next action is a successful checkpoint upload and a tested-image EKS submission of stage A.

Numerical scoreboard, timeline, leaderboard and lab-browser rows remain unchanged because this deliverable adds no new measured factor-base row, curve or FB1 object. Report sources, protocol, catalogue and tests are in `research/n83_factor_base_designs_20261031/`. All requested experiment stages remain open until their frozen results and replay records arrive.
