#let navy = rgb("192c42")
#let blue = rgb("345d98")
#let teal = rgb("087f82")
#let pale = rgb("eff4f8")
#let gold = rgb("fff4df")
#let muted = rgb("53647a")
#let data = json("render-data.json")
#set document(title: "Prime31: From Isogeny Fibers to Exact Relation Extraction", author: "Isogeny Labs research workspace")
#set page(paper: "us-letter", margin: (top: 0.6in, bottom: 0.57in, x: 0.64in),
  header: align(right, text(size: 8pt, fill: muted)[PRIME31 / FIBER-DOMAIN SUPPLEMENT / #data.report_date_label]),
  footer: [#text(size: 7.8pt, fill: muted)[Development evidence; complete-cost advantage not established] #h(1fr) #context counter(page).display("1")])
#set text(font: "DejaVu Sans", size: 9.7pt, fill: navy)
#set par(leading: 0.43em, spacing: 0.6em)
#set heading(numbering: none)
#show heading.where(level: 1): it => block(above: 0pt, below: 10pt)[#text(size: 24pt, weight: "bold")[#it.body]]
#show heading.where(level: 2): it => block(above: 8pt, below: 4pt)[#text(size: 12.5pt, weight: "bold", fill: teal)[#it.body]]
#set table(stroke: 0.45pt + rgb("d4dfe7"), inset: 5pt, align: left)
#let small(body) = text(size: 8pt, fill: muted, body)
#let note(body, fill: pale) = block(width: 100%, fill: fill, radius: 5pt, inset: 9pt, body)
#let tag(label, value) = block(fill: pale, inset: 9pt, radius: 5pt, width: 100%)[#text(size: 7.7pt, fill: muted)[#label] #linebreak() #text(size: 16pt, weight: "bold")[#value]]
#let mono(body) = text(font: "DejaVu Sans Mono", size: 7.6pt, body)
#let pkc = "https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf"
#let bdls = "https://arxiv.org/pdf/2003.10118"

= Two curves, two different jobs

The target curve holds the unknown logarithm. An auxiliary curve supplies a structured list of candidate coordinates. The isogeny is a tool for organizing that list. It does not reveal the target logarithm by itself.

#image("figures/roles.svg", width: 100%)

== Start with the prescribed prime

Here the field is fixed: $p=2^31-1=2147483647$. We choose an auxiliary curve over *this same field*, rather than replacing the problem with a more convenient prime. The retained target is
$ E: y^2=x^3+847514846x+2087444724, quad abs(E(bb(F)_p))=n=2147446831. $
Its public points are $G=(625037715,2100627251)$ and $Q=(984837385,1142774582)$. The task is to recover $s$ from $Q=[s]G$.

== What the kernel does

On $A_-:y^2=x^3-x$, the retained point $S=(4,639874715)$ has order $2^30$. Put $K=[2^20]S$ and $U=(0,0)$. The chosen subgroup is
$ H_2=⟨2K,U⟩ ≅ C_512 times C_2, quad abs(H_2)=1024. $
An isogeny is a group-respecting map with a finite kernel. Here $phi(h)=O$ for each $h in H_2$, so
$ phi(S+h)=phi(S)+phi(h)=phi(S). $
That is why the whole coset $S+H_2$ lies in one fiber. Its endpoint is finite and not two-torsion, giving 1,024 distinct x-values in this retained construction. [1]

#grid(columns: (1fr, 1fr, 1fr), gutter: 8pt,
  tag("FULL AUXILIARY FIBER", "1,024 x-values"),
  tag("RETAINED TARGET BASE", "515 x-values"),
  tag("NATIVE DEVELOPMENT CHECKS", data.tests_label))

#small[The 1,024/515 counts describe the existing original-target fixture, not a new yield or timing measurement. The target keeps both signs above each usable x, with one canonical logarithm column. Auxiliary translations must not be treated as addition symmetries on E.]

#if data.draft { note(fill: gold)[*Draft supplement.* Qualification/publication metadata is still being finalized. The mathematical component derivations below are distinct from complete-cold ECDLP performance.] }

#pagebreak()
= How small isogenies make a large map

The degree of a separable isogeny counts its kernel points. Degrees multiply when maps compose. A large degree can therefore be implemented as a chain of small maps with known kernels. The chain is also a way to express membership using low-degree equations. [1, §3.4]

#image("figures/degrees.svg", width: 100%)

== Why the first four kernel points are useful

$H_2$ contains all of $A_-[2]=\{O,(0,0),(1,0),(-1,0)\}$. Quotient those points first, using two normalized degree-two maps:
$ r_1(x)=x-1/x, quad r_2(t)=t+4/t. $
Their codomains are $y^2=x^3+4x$ and $y^2=x^3-16x$. Substituting one formula into the next gives
$ delta_x(x)=r_2(r_1(x))=(x^2+1)^2/(x(x^2-1)). $
#note[To say “x maps to z,” clear the denominator:
$ x^4-z x^3+2x^2+z x+1=0. $
This is a five-term quartic. Its full point map is $delta(P)=(4x([2]P),8y([2]P))$.]

The remaining generator is $delta(2K)$, of order 256. Eight further degree-two quotients finish the same degree-1,024 map. Using $delta(K)$ would select the wrong remaining subgroup.

== Three ways to describe the same membership condition

#table(columns: (1.65fr, 0.7fr, 0.9fr, 0.9fr),
  table.header([*Description*], [*Degrees*], [*Finite chart*], [*Infinity chart*]),
  [Ten small map equations], [2 each], [31 variables], [30 variables],
  [Five paired map equations], [4 each], [16 variables], [15 variables],
  [One expanded fiber polynomial], [1,024], [4 variables], [3 variables])

The variable counts are for three target summands. Small equations add intermediate variables; eliminating them produces a high-degree equation. Neither fewer variables nor fewer input terms guarantees cheaper solving. The new extractor uses the known inverse fibers as well as the equations.

#note(fill: gold)[*Which isogenies are useful here?* Small, separable maps with known rational kernels make construction and evaluation concrete. The full-two-torsion prefix gives a particularly simple quartic. A large degree is useful as a structured composition; its size alone supplies no ECDLP speedup. Fast large-prime-degree evaluation also needs specific compatible index structure, not merely a smooth auxiliary group. [2, §§4.8-4.15]]

#pagebreak()
= Pay for a fixed check once

The old query wrapper revalidated every coordinate through the entire fixed tower on each query. The prepared object validates the immutable field, factor base, full fiber and tower in cold setup, then reuses them. Each query still checks its own target and limits.

== Derive the component saving

Each retained degree-two step is $r(x)=x+tau/(x-xi)$. Its existing validation computes the inverse by exponentiation to $p-2$. For this particular p, that code performs 30 result multiplications and 30 squarings, followed by one multiplication by $tau$:
$ C=1024 times 10 times (30+30+1)=624640. $

#image("figures/validation.svg", width: 100%)

These are *source-derived operation counts*, not elapsed times. Both prepared modes perform this validation once, saving $(q-1)C$ against q completed legacy validations. The fiber mode captures the already-computed block endpoints in a private validated object and builds its index from them. It removes the earlier second map traversal, saving a further C in that setup component. These counts apply to the retained Vélu tower; Dense and Dickson have different paths. The public standalone index constructor still checks every original map independently.

Index construction still costs work. It checks full-fiber splitting, live leaves and parent links, and allocates its maps. The five-block path buffer holds $1024 times 6 times 8=49152$ logical bytes before container overhead. These costs remain in cold setup and must be repaid through extraction savings. Zero *additional map multiplications* is not zero index cost.

== Keep all equations; restrict only impossible target leaves

The complete auxiliary fiber V still appears in the membership equations. The solver's leaf domains use only x-values represented in the validated target factor base. A nonlifting x cannot participate in a signed rational relation on E, so removing it from the leaf search loses no such relation.

The shared index is built from *single-coordinate paths*. It does not precompute all pairs or triples of target points. In the five-block presentation, the full node counts by level are 1,024, 256, 64, 16, 4 and 1; target-liftability pruning can remove nodes and edges.

#note[*Keep the three claims separate.* Fixed validation is reused; impossible target leaves are pruned; and a new bounded extractor uses conditional fibers. Each has its own cost. The full comparison still includes auxiliary construction, indexing, query generation, F4, root finding, signed replay, matrix work and scalar recovery.]

#small[Derivation: native/src/oracle.rs, counted_power and full-tower validation; native/src/fiber_extract.rs, the private validated-path constructor. Every original denominator and terminal is still checked. #data.at("path_reuse_qualification_status", default: "The later path-reuse refactor awaits separate qualification; the earlier receipt remains bound to its original source.") No component saving alone establishes a measured complete-recovery advantage.]

#pagebreak()
= Branch only when an equation helps

Suppose an internal selector coordinate has already been fixed. Its children are the exact inverse fiber under one degree-two or degree-four block. With at most four children, the driver can evaluate the available univariates directly instead of factoring another polynomial.

#image("figures/branching.svg", width: 100%)

In the illustration, the quartic $x^4-5x^3+2x^2+5x+1$ has the displayed four children modulo 31. The extra h supplies information the selector alone did not contain. This is a small explanatory example; the actual task keeps its original Prime31 field.

== What the driver actually does

It propagates forced ancestors, tests all available univariates on a small conditional domain, and otherwise calls the pinned finite-field root routine. It uses per-child exact substitution. The old visitor's `domain_roots` and `sibling_batch` options do not configure this new driver; the effective policy is recorded explicitly.

When no informative univariate is present, one bounded F4 call at that node may derive one. The driver retains the incoming equations *and* F4's consequences. If it still has no useful univariate, it returns Unknown. There is no fallback loop through all field values or all factor-base tuples.

#table(columns: (1fr, 3fr),
  [*Complete*], [Every branch in the exact cover closed; all emitted coordinates satisfy every original equation.],
  [*First witness*], [A signed relation replayed correctly and the caller stopped early; other solutions may remain.],
  [*Unknown*], [No informative consequence, a bound, or a deadline prevented closure. This is not proof of no relation.])

#note(fill: gold)[*Do not confuse two kinds of intermediate coordinate.* A selector coordinate belongs to the auxiliary map chain. The coordinate of the target sum $P_1+P_2$ belongs to E and may lie outside V. Only selector coordinates receive fiber domains. Every accepted witness still checks target signs, cancellation and infinity charts.]

#small[Early falsifier: if initial F4 times out, or leaves a nonterminal residual with no informative univariate, this driver returns Unknown. An exact contradiction can close a branch. Defaults: 4,096 nodes; 64 F4 calls; root degree 1,024; 100,000 retained terms; shared query deadline. These are caps, not measured complexity.]

#pagebreak()
= One matrix, and a careful twist

== Recover the target before learning every base logarithm

Write a replayed relation as $A ell=alpha+beta s$. Over $bb(F)_n$, keeping $B=[A | -beta]$ suffices to derive the rank of A, and recovers s when its last column becomes pinned.

#image("figures/rank.svg", width: 100%)

Project the row space of B onto its first columns. The image is the row space of A. Its kernel is either zero or the one-dimensional span of $(0,...,0,1)$. Hence
$ op("rank")(A)=op("rank")(B)-bold(1)_("last column pinned"). $
In reduced row-echelon form, “pinned” means the last unknown has a row with no other nonzero coefficient. For example, $ell_1-s=alpha_1$ and $ell_1-3s=alpha_2$ give $2s=alpha_1-alpha_2$, even if another base logarithm is still unknown. Duplicate base rows with different right-hand data must therefore be retained.

== A target twist is a new rational problem

For nonsquare $d in bb(F)_p$, a monic quadratic twist is
$ E^((d)):Y^2=X^3+d^2 a X+d^3 b. $
Its order is $2(p+1)-abs(E(bb(F)_p))$, so this fixture's twist order is 2,147,520,465. The original prime-order subgroup does not transfer as the same rational subgroup. Choose valid new subgroup data $(n,G,Q)$ and rebuild the usable base.

If a normalized isogeny is $eta(x,y)=(r(x),y r'(x))$, twisting both endpoints gives
#note[$ eta_d(X,Y)=(d r(X/d),Y r'(X/d)). $]
In same-x form $d y^2=x^3+a x+b$, V can be retained while liftability changes. In the monic model $X=d x$, the corresponding candidate set is dV. These coordinate formulas do not automatically transport the original rational DLP.

== The auxiliary twist needs its own kernel check

For $A_-^((d)):Y^2=X^3-d^2X$, the sparse prefix becomes
$ delta_(d,x)(X)=(X^2+d^2)^2/(X(X^2-d^2)). $
Here $p equiv 3 mod 4$, so $-d$ is a square. Choosing $u^2=-d$ gives a rational auxiliary isomorphism $(x,y) mapsto (u^2x,u^3y)$: it carries the full rational H2 and sends V to $-d V$. This auxiliary fact supplies no target-group symmetry. Also, $y^2=x^3+x$ and $y^2=x^3-x$ are different quartic-twist classes here, not a quadratic-twist pair.

#small[Rank argument: native/src/collector.rs and fixed recovery contracts. Twist formulas and rationality distinction: the preceding sparse-h2-followup/MATHEMATICS.md; normalized isogeny coordinates are consistent with [2]. No twist recovery campaign is claimed.]

#pagebreak()
= What would count as an advantage?

A component improvement becomes a complete ECDLP advantage only if the final scalar replays on the original target and the complete cold cost beats the strongest applicable controls under matched conditions.

#table(columns: (1.45fr, 2.55fr),
  table.header([*Comparison*], [*What must be charged and checked*]),
  [Prepared vs legacy], [Same base and equations; count once-only setup, exact outputs and remaining query work.],
  [Fiber vs prepared], [Charge the extra index and dynamic-domain work; retain Unknown outcomes and signed-row/rank effects.],
  [Applicable IC controls], [Same-base folded batched pair tables, specialized prime batched MITM, and maintained complete prime-fast IC with its own base/setup.],
  [Optimized rho], [Matched original-target negation-map controls, including the tuned one-thread implementation; setup and scalar replay included.])

#small[Complete cost includes construction and validation, memory/indexing, queries, extraction and signed replay, matrix/recovery work, final $[s]G=Q$ verification, and outer process overhead. Different search seeds on one fixed target are not independent target trials.]

== Current development evidence

#data.tests_description
#small[#data.qualification_scope]
#small[*Qualification:* #data.qualification_status. *Root library:* #data.root_library_status. *Specialized baseline:* #data.specialized_baseline_status. *Campaigns attempted:* #data.campaigns_label.]
#small[*Compiled source:* #mono(data.source_commit) #linebreak()
*Source manifest SHA-256:* #mono(data.source_manifest_sha256) #linebreak()
*Receipt:* #data.qualification_receipt.]

== Step-by-step checklist

#set enum(spacing: 4pt)
+ *Fix the problem.* Record p, target E, subgroup order, G and Q.
+ *Separate curve roles.* A− builds the selector; E supplies signed factor-base points.
+ *Certify the fiber.* Check H2, the coset, non-poles, endpoint and all of V; retain only valid target lifts for leaf domains.
+ *Choose the presentation.* Ten quadratics, five quartics, or a dense polynomial; preserve the same mathematical fiber.
+ *Charge cold preparation.* Validate once; include the conditional index, temporary path buffer and allocation in setup.
+ *Use exact conditional branches.* Keep all univariate roots in the current fiber; retain Unknown when no informative equation or resource remains.
+ *Replay every relation.* Check original equations, target point signs, cancellation and infinity.
+ *Recover with the augmented matrix.* Retain duplicate coefficient rows; verify every candidate scalar with $[s]G=Q$.
+ *Handle a twist as a new rational instance.* Recheck its subgroup, transported maps, kernel and lifts.
+ *Compare complete cold executions.* Freeze inputs, sources, limits and controls; retain failures and missing costs before claiming an advantage.

#small[*Sources.* [1] Petit, Kosters and Messeng, #link(pkc)[PKC 2016, §§3.4 and 5]. [2] Bernstein, De Feo, Leroux and Smith, #link(bdls)[§§4.8-4.15]. Closest implementation: crypto PR #4826, finite-domain roots and sibling extraction. Current code: oracle.rs, fiber_extract.rs, collector.rs. Shared research KB unavailable; no novelty claim is inferred.]
#if data.pr_url != "" { small[*Implementation review:* #link(data.pr_url)[continuation PR].] }
