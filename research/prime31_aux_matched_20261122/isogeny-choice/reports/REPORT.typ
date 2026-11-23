#let navy = rgb("142c42")
#let blue = rgb("214d70")
#let pale = rgb("eff4f8")
#let amber = rgb("9a5d14")
#let muted = rgb("546a7d")
#let receipt = json("../DEVELOPMENT_RECEIPT.json")
#set document(title: "Prime31 Isogeny Comparison — Implementation and Fixed Fixtures", author: "Isogeny Labs research workspace")
#set page(paper: "us-letter", margin: (top: 0.6in, bottom: 0.58in, x: 0.65in),
  header: align(right, text(size: 8pt, fill: muted)[PRIME31 / ISOGENY CHOICES / 22 NOV 2026]),
  footer: [#text(size: 8pt, fill: muted)[Development evidence · PCC standing: unverified] #h(1fr) #context counter(page).display("1")])
#set text(font: "DejaVu Sans", size: 10pt, fill: navy)
#set par(leading: 0.53em, spacing: 0.7em)
#set heading(numbering: none)
#show heading.where(level: 1): it => block(above: 0pt, below: 11pt)[#text(size: 25pt, weight: "bold", fill: navy)[#it.body]]
#show heading.where(level: 2): it => block(above: 11pt, below: 6pt)[#text(size: 13pt, weight: "bold", fill: blue)[#it.body]]
#set table(stroke: (bottom: 0.5pt + rgb("d4dfe7")), inset: 7pt, align: left)
#let small(body) = text(size: 8pt, fill: muted, body)
#let note(body) = block(width: 100%, fill: pale, radius: 5pt, inset: 11pt, body)
#let tag(label, value) = block(fill: pale, inset: 10pt, radius: 5pt, width: 100%)[#text(size: 8pt, fill: muted)[#label] #linebreak() #text(size: 17pt, weight: "bold")[#value]]

= The comparison is implemented

The Rust extension compares *three order-1,024 kernels* on $y^2=x^3-x$, an auxiliary $y^2=x^3+x$ control, three polynomial presentations, and two ways to use the target's degree-three isogeny. Complete arithmetic fixtures and failure-aware replay are included.

#grid(columns: (1fr, 1fr, 1fr), gutter: 9pt,
  tag("FOCUSED CONTRACT TESTS", str(receipt.tests_passed) + " passed"),
  tag("COMPARISON INPUTS", "230 prepared"),
  tag("CAMPAIGN CELLS RUN", "0"))

== What the fixed fixtures show

Each auxiliary fiber contains exactly *1,024 candidate x coordinates*. Only those that lift to points on the selected target become usable factor-base columns. A point and its negative share one column, with opposite signs.

#image("fixed_columns.svg", width: 100%)
#small[Figure 1. Exact cardinalities for the single fixed coset $S+H$ in each arm. Source: `fixtures/kernel_correctness_fixtures.csv`. These four deterministic fixtures are neither independent samples nor timing measurements.]

#table(columns: (1.2fr, 1fr, 1fr, 1.3fr),
  table.header([*Kernel*], [*Original E*], [*Native E₁*], [*Transported E₁*]),
  [A+ cyclic], [518], [506], [518],
  [A− / H0], [508], [508], [508],
  [A− / H1], [512], [519], [512],
  [A− / H2], [515], [533], [515])

#note[*How to read H2:* lifting the same candidate x coordinates on E₁ gives 533 columns. Transporting the original 515 point columns through the isogeny gives 515. These are different constructions, so the larger native count alone does not establish a lower solving cost.]

The performance campaign is *blocked before launch*. The required checkpoint upload failed, and this session has no authenticated IsoLab submission route or current shared-budget receipt. Source publication and correctness tests do not replace those admission requirements. No timing ratio or fastest isogeny is claimed.

#pagebreak()
= One large quotient, many small maps

All arithmetic uses the prescribed prime $p=2147483647=2^31-1$. For the A− auxiliary curve, use
$ S=(4,639874715), quad K=[2^20]S, quad U=(0,0). $
Here $K$ has order 1,024; $U$ is an independent point of order two. The kernels are
$ H_0=⟨ K ⟩, quad H_1=⟨ K+U ⟩, quad H_2=⟨ 2K,U ⟩. $
H0 and H1 are cyclic; H2 has structure $C_512 times C_2$. Their common subgroup $⟨ 2K ⟩$ has order 512. Each quotient collapses exactly its kernel to the identity.

#image("kernel_routes.svg", width: 100%)
#small[Figure 2. The three A− quotient chains share their first nine degree-two maps. Their final degree-two kernel differs. Each complete composition has degree $2^10=1024$. This is a kernel-factorization diagram, not an isogeny-volcano measurement.]

== Why the chain describes a factor base

Choose a coset $S+H$, and let $chi=x(phi(S))$. Every point in that coset has the same image under $phi$. If the terminal image is not two-torsion, its coset has 1,024 distinct x coordinates. Write the x map as $N(x)/D(x)$. Then the candidate set is exactly the roots of
$ L(x)=N(x)-chi D(x)=product_(v in V)(x-v). $
The fixtures compare this coefficient identity, every kernel point, every coset point, and all finite kernel poles. Here $deg N=1024$ and $deg D=1023$.

== Three equivalent ways to express membership

#table(columns: (1.35fr, 1fr, 1fr, 1.05fr),
  table.header([*Presentation*], [*Map degree*], [*Internal variables per x*], [*Total variables, m = 3*]),
  [Ten small maps], [2], [9], [31],
  [Five paired blocks], [4], [4], [16],
  [Expanded polynomial], [1,024], [0], [4])

The last column includes the three factor-base x variables and one intermediate sum variable in the finite $S_3$ chart. Identity-intermediate charts are handled separately. Reducing the number of variables raises the membership degree; the polynomial solver must determine the practical tradeoff.

For two successive x maps, write
$ N=x(x-alpha)+tau, quad D=x-alpha, $
$ P=N(N-beta D)+sigma D^2, quad Q=D(N-beta D). $
The pair becomes the single constraint $P-z Q=0$ of rational-map degree four. The standalone arithmetic verifier also checks projective degree-two and degree-four evaluation with one final inversion. This panel compares polynomial constraints; it does not benchmark those map evaluators.

#pagebreak()
= Moving the target and its points

The fixed public ECDLP lives on
$ E: y^2=x^3+847514846 x+2087444724, $
with prime order $n=2147446831$, generator $(625037715,2100627251)$ and public target $(984837385,1142774582)$. The comparison never supplies the target logarithm to its collector.

The degree-three map lands on
$ E_1: y^2=x^3+323891735 x+1985248359. $
Its coordinate formula is
$ r(x)=x+963718081/(x-533591203)+1037789400/(x-533591203)^2, $
$ psi(x,y)=(r(x), y r'(x)). $
The generator maps to $(1767020261,634125456)$. The nonidentity geometric kernel points are not rational over $bb(F)_p$; a rationally defined isogeny can still have such a kernel. On these rational prime-order groups, $psi$ is a bijection and preserves the scalar in $Q=[d]G$.

#image("target_routes.svg", width: 100%)
#small[Figure 3. Reusing V on E₁ builds a native destination base. Mapping the original points builds a transported base. Only the second route carries the exact original relation set along with it.]

== What transport preserves—and what execution can change

If $R=P_1+P_2+P_3$, then $psi(R)=psi(P_1)+psi(P_2)+psi(P_3)$. The auditor checks each canonical sign change and column permutation, as well as the transformed natural query $R=[a]G+[b]Q$. Exact signed rows and their ranks correspond.

The bounded solver stops after its first relation. Polynomial representation and traversal order can change which witness it finds first, whether it reaches a cap, and the observed rank trajectory. Thus exact relation-set equivalence does *not* imply identical measured execution.

All four fixed transported candidate sets retain 1,024 distinct finite values, with no pole or duplicate image. The code still records poles and collisions rather than assuming this holds for arbitrary fibers. Transported membership is rebuilt from $r(V)$ and uses the dense presentation.

#note[*Twist reminder.* A+ and A− here are the two quartic twist classes, not a claimed rational quadratic-twist isomorphism. Twisting the target is a separate change of rational group. This 230-cell panel compares E with its degree-three isogenous curve E₁; it does not execute a target-twist campaign. The companion visual guide explains auxiliary and target twists in detail.]

#pagebreak()
= What is checked; what runs next

== Development evidence

The optimized standalone crate passed *#receipt.tests_passed focused contracts*, with #receipt.tests_failed failures and #receipt.tests_ignored ignored tests. Test executables were copied to a private qualifier directory and hashed before execution; the retained after hashes match. The checks cover kernel/fiber identities, degree-four constraints, target transport, three rho controls on small fields, finite-domain root modes, row replay, failure paths and tampering.

This is a producer development receipt, *not a signed PCC execution verdict*. See `DEVELOPMENT_RECEIPT.json` and its raw logs. The complete root library was not qualified by this check. The earlier root gate and the unrelated historical benchmark-record gate remain open.

F4 and rho sources are pinned to revision `33b3bc3c7485df0fe80aae41e2093cf940d87ff3`. The build audits 43 production/dependency files. Three unused, malformed sibling-pair members in that revision's `ic_boundary.rs` are omitted by declared exact byte ranges; no replacement bodies are inserted. The selected call closure does not use them. Snapshot bytes, selected intervals and the limitation are recorded in `upstream/`.

== The registered comparison

#table(columns: (2.5fr, 1fr, 1fr),
  table.header([*Cell family*], [*Per round*], [*Five rounds*]),
  [Auxiliary IC: four kernels × seven arms], [28], [140],
  [Plain IC matched by actual usable columns], [12], [60],
  [Three rho controls × two target curves], [6], [30],
  [*Total*], [*46*], [*230*])

Every round uses $m=3$, coset scalar 1 and query/rho seed 4188. These are repeated executions of the same workloads. The recorded order interleaves controls, rotates each round and reverses alternate rounds. Each cell has a 120-second cold envelope, 4 GiB memory bound and one CPU/thread. The inherited aggregate USD 5 ceiling includes prior lane spending; its remaining headroom is unknown.

Cold cost includes construction, lifting, transfer, plain-control matching, relation search, elimination and original-target scalar replay. Native operation counters are partial and sometimes overlapping; `prime_fast` table counters retain an upstream accounting convention. New domain/batching counts remain in per-chart visit records. Missing aggregate fields are not zeros.

== Step-by-step execution checklist

+ *Inspect the prepared panel.* `panel/index.json` records all 230 inputs and hashes. The current flow is 230 planned, 0 attempted, 0 completed.
+ *Resolve admission.* Obtain the authenticated IsoLab route, a successful checkpoint upload, canonical protocol validation/freeze and live shared-budget/capacity receipts. Freeze the original absolute campaign deadline.
+ *Publish and bind the worker.* Use the exact source, release binary and image; check source/input hashes. Complete separately registered warmup and A/A admission before timing claims.
+ *Execute in recorded order.* Use `run-choice.sh`; keep setup failures, oracle errors, caps, timeouts and OOMs. Do not call the binary directly to bypass admission.
+ *Replay every transcript.* Check each row, both matrix ranks, useful dependencies and recovered $d G=Q$ on the original curve. An incomplete stream stays incomplete.
+ *Compare the full outcomes.* Report complete cold solve costs and censored cells beside the matched plain and rho controls. Choose a useful isogeny or presentation only from the admitted evidence.

#small[Evidence: PR #4923, `isogeny-choice/PROTOCOL.md`, `DEVELOPMENT_RECEIPT.json`, `fixtures/`, `upstream/`, and `panel/index.json`. Mathematical background: Petit–Kosters–Messeng, §3.4, “Algebraic Approaches for the Elliptic Curve Discrete Logarithm Problem over Prime Fields,” PKC 2016. The companion 18-page guide provides the longer tutorial.]
