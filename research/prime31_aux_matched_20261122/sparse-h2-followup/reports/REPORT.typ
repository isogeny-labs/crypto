#let navy = rgb("192c42")
#let blue = rgb("345d98")
#let teal = rgb("087f82")
#let pale = rgb("eff4f8")
#let gold = rgb("fff4df")
#let muted = rgb("53647a")
#let data = json("report-data.json")
#set document(title: "Prime31 Sparse H2: Same Fiber, Sparse Route, Fair Comparison", author: "Isogeny Labs research workspace")
#set page(paper: "us-letter", margin: (top: 0.61in, bottom: 0.6in, x: 0.65in),
  header: align(right, text(size: 8pt, fill: muted)[PRIME31 / SPARSE H2 FOLLOW-UP / #data.report_date_label]),
  footer: [#text(size: 8pt, fill: muted)[Unsigned producer evidence | PCC standing: unverified] #h(1fr) #context counter(page).display("1")])
#set text(font: "DejaVu Sans", size: 10.1pt, fill: navy)
#set par(leading: 0.48em, spacing: 0.67em)
#set heading(numbering: none)
#show heading.where(level: 1): it => block(above: 0pt, below: 12pt)[#text(size: 25pt, weight: "bold")[#it.body]]
#show heading.where(level: 2): it => block(above: 10pt, below: 5pt)[#text(size: 13pt, weight: "bold", fill: teal)[#it.body]]
#set table(stroke: 0.5pt + rgb("d4dfe7"), inset: 6pt, align: left)
#let small(body) = text(size: 8.2pt, fill: muted, body)
#let note(body, fill: pale) = block(width: 100%, fill: fill, radius: 5pt, inset: 10pt, body)
#let tag(label, value) = block(fill: pale, inset: 10pt, radius: 5pt, width: 100%)[#text(size: 8pt, fill: muted)[#label] #linebreak() #text(size: 17pt, weight: "bold")[#value]]
#let path(body) = text(size: 8.2pt, font: "DejaVu Sans Mono", body)
#let base-url = "https://github.com/isogeny-labs/crypto/blob/lane/prime31-sparse-h2/research/prime31_aux_matched_20261122/"

= Same fiber, a sparser first step

The follow-up adds a different factorization of the *same degree-1,024 auxiliary isogeny*. Removing all four rational two-torsion points first gives a five-term quartic constraint. The original kernel, coset, final normalized map, and candidate set $V$ are preserved by the construction.

It also corrects the plain-control cost boundary and prepares a comparison in two stages: screening first, then selected attempts at complete recovery. This report explains those changes and their evidence; it supplements the earlier longer visual tutorial.

#grid(columns: (1fr, 1fr, 1fr), gutter: 9pt,
  tag("NATIVE QUALIFICATION", data.tests_label),
  tag("SCREENING INVENTORY", "77 inputs"),
  tag("RECOVERY CANDIDATES", "385 inputs"))

#if data.draft { note(fill: gold)[*Draft while qualification is pending.* No native pass count or issued identity identifier is supplied until it appears in the retained receipt.] }

== What changed, and what each change supports

#table(columns: (1.3fr, 2.7fr),
  table.header([*Change*], [*Meaning at this scope*]),
  [Sparse H2 route], [A different sequence of small quotients; exact fixed-fixture equality is checked by native contracts.],
  [Fair plain setup], [A plain solver uses the frozen matching column count and pays for its own construction, indexing, and solve.],
  [Staged comparison], [A 64-query screening cap is separated from bounded recovery attempts with a larger logical ceiling.],
  [Same-base pair control], [The maintained folded, batched pair table is compared on the same signed base and public-query stream.])

== Development evidence and the remaining dependency

#data.tests_description

Native checks cover the new factorization, legacy input compatibility, frozen plain references, staged-cell validation, pair-table witnesses, and the existing replay interfaces. Seventeen copied test executables retained matching before/after hashes. The initial build failure and its repair are preserved. The full root-library gate stopped before tests at an unchanged duplicate manifest key; that broader gate remains open.

The experiment remains *unlaunched*. The checkpoint gate lacks an upload token/AWS profile, and there is no authenticated runner submission, current shared-budget receipt, or completed canonical protocol freeze. Source publication does not provide those receipts. The next execution step is to resolve admission, then run a frozen screening selection through the gated wrapper.

#note[*Fixed public problem.* $p=2147483647$, $m=3$, and $E:y^2=x^3+847514846x+2087444724$, with prime order $n=2147446831$. The public generator is $(625037715,2100627251)$ and $Q=(984837385,1142774582)$. The collector receives no target logarithm.]

#small[Scope: implemented construction and prospective comparison; no new Prime31 natural-query campaign, census, timing ranking, or complete ECDLP recovery is reported. Sources: DEVELOPMENT_RECEIPT_v2.json, PROTOCOL.md, ACCOUNTING.md.]

#pagebreak()
= Why an isogeny merges a coset

Think of a kernel $H$ as the points the map sends to the identity $O$. Because an isogeny respects addition, adding a kernel point does not change the image:
$ phi(P+h)=phi(P)+phi(h)=phi(P), quad h in H. $
So every point in $P+H$ lands at the same point. For this separable quotient, *degree 1,024 means 1,024 kernel points*.

#image("../sparse_route.svg", width: 100%)
#small[Figure 1. Two grouping orders for the fixed coset $S+H_2$. This exact vector illustrates kernel factorization, not an isogeny volcano or measured running time. Source and derivation: MATHEMATICS.md; checks: sparse_h2_contract.rs.]

On $A_-:y^2=x^3-x$, take $S=(4,639874715)$, $K=[2^20]S$ of order 1,024, and $U=(0,0)$. Then $H_2=⟨2K,U⟩$ has structure $C_512 times C_2$. It needs *two generators*. Its four two-torsion points are $O,(0,0),(1,0),(-1,0)$.

#note[*What stays fixed:* the source curve, the original coset, and the full kernel. The two routes ultimately produce the same normalized map and the same 1,024 original x-values. Intermediate curves and equations may differ.]

#pagebreak()
= Small maps or one large equation?

An x-coordinate membership test can be expressed with many small-degree equations or one expanded polynomial. A chain introduces intermediate variables. Eliminating those variables raises the degree. The follow-up changes the *first block's algebra*, while retaining the existing solver and layouts.

== Derive the sparse block, one substitution at a time

First quotient $A_-$ by $U=(0,0)$; then quotient by the image of $(1,0)$:
$ r_1(x)=x-1/x, quad r_2(t)=t+4/t. $
The first codomain is $y^2=x^3+4x$, and the second is $B:y^2=x^3-16x$.
$ delta_(x)(x)=r_2(r_1(x))=((x^2-1)^2+4x^2)/(x(x^2-1))=(x^2+1)^2/(x(x^2-1)). $
Setting $z=delta_(x)(x)$ gives the five-term constraint
#note[$ x^4-z x^3+2x^2+z x+1=0. $]

The full normalized point map is $delta(P)=(4x([2]P),8y([2]P))$; its second coordinate is y times the derivative of $delta_x$. The residual point $delta(2K)$ has exact order 256. Eight further degree-two maps finish the same quotient; using $delta(K)$ instead would give the wrong residual subgroup.

== The existing three presentations

#table(columns: (1.7fr, 0.8fr, 1fr, 1fr),
  table.header([*Presentation*], [*Membership degree*], [*Finite-chart variables*], [*Identity-chart variables*]),
  [Ten quadratic stages], [2], [31], [30],
  [Five quartic blocks], [4], [16], [15],
  [One dense membership polynomial], [1,024], [4], [3])

These totals are for three summands. They include the three candidate x-values; the finite chart adds one intermediate summation variable. Sparsity does not by itself establish lower solving degree, less F4 work, or faster recovery. #small("[pcc:unverified]")

== Why the rational denominators are safe here

At the prefix poles $x=0,1,-1$, the cleared equation has constant values $1,4,4$, and every z-term vanishes. No finite z therefore turns a pole into a spurious root. For the full map, the native certificate requires
$ N(x)-chi D(x)=product_(v in V)(x-v), quad chi=x(phi(S)), $
with $deg N=1024$, $deg D=1023$, and $D(v)!=0$. The endpoint is finite and not two-torsion, which prevents a coset point and its negative from sharing an x-value inside this fiber.

#small[*Identity artifact:* #data.certificate_id. #data.certificate_description The format evaluates in $bb(F)_(2^61-1)$, not Prime31; deterministic seeds do not provide an unconditional random-sampling bound. The record certifies neither the full kernel nor a speedup.]

#pagebreak()
= Pay for each algorithm's own work

The earlier plain-control path rebuilt the auxiliary reference before constructing a plain base of matching size. That made its cold interval include reference work a plain solver does not intrinsically need once the matching count is preregistered. Historical receipts keep that meaning.

The v2 path binds the frozen fixture by hash, kernel, coset, factorization, target arm, and column count. It then builds its *own* plain base without calling the auxiliary kernel builder. An offline auditor reconstructs the reference independently and checks the matching count.

#image("accounting_flow.svg", width: 100%)
#small[Figure 2. Cost boundaries, not measured durations. The internal clock starts after input parsing returns and ends before final-summary serialization. Intermediate event output is included. Sources: ACCOUNTING.md; choice_main.rs.]

#table(columns: (1.2fr, 2.8fr),
  table.header([*Candidate*], [*Work charged inside its algorithm interval*]),
  [Auxiliary IC], [Kernel/fiber construction and certification; target lifts/transport; indexing; optional pair table; collection, matrix work, and scalar replay.],
  [Plain IC], [Own target validation/transport; plain-base construction/indexing; optional pair table; collection, matrix work, and scalar replay.],
  [Rho], [Target validation/transport, selected implementation's setup and walk, and destination/source scalar replay.])

#note(fill: gold)[*Unknown is not free.* Historical reference-preparation time and priced cost are retained as `null`, with the explicit status `historical_matching_work_unmeasured_not_zero`. Study expenditure still includes preparation, qualification, failed attempts, and controls. No subtraction from an old clock fabricates a new measurement.]

The outer observation must include input reading, fixture hashing, final output, and process overhead excluded by the internal clock. Future reports must retain wall/CPU/RSS where actually observed. Field work follows the current `bitop/v1` rules; other native work retains its required 0/0.1/1 GAE band. Partial or overlapping counters must not be added into a fictional complete total.

#pagebreak()
= Screen first, then attempt recovery

Both stages use natural queries and accept at most one witness per query. The larger recovery ceiling removes the old 64-row bottleneck; it does not guarantee enough relations or enough time to solve.

#image("stages.svg", width: 100%)
#small[Figure 3. Prospective inventories and selection order. Current experiment flow: 0 attempted, 0 completed; 0 recovery candidates selected. Input generation is development work. Source: PROTOCOL.md and staged-panel index metadata.]

#table(columns: (2.5fr, 0.8fr, 0.9fr),
  table.header([*Family*], [*One stream*], [*Five-stream pool*]),
  [Auxiliary polynomial cells, including sparse H2], [35], [175],
  [Auxiliary pair-table controls], [12], [60],
  [Plain polynomial controls], [12], [60],
  [Plain pair-table controls], [12], [60],
  [Three rho controls on two destinations], [6], [30],
  [*Total*], [*77*], [*385*])

Screening uses seed *4188*. The recovery candidate pool uses *4188, 104729, 130363, 169087, 224737*. Each configuration in a stream receives the same seed. These are different search streams for *one public target and one fixed coset*, not five independent targets or timing repeats. Order interleaves rho controls, rotates each stream, and reverses odd streams; cell hashes bind the exact order.

== The resource envelope stays fixed

#table(columns: (1.3fr, 2.7fr),
  [*Query ceilings*], [64 screening; 65,536 complete-recovery candidates],
  [*Physical bounds*], [One cell, one affinity-bound CPU/thread, 4 GiB; zero retries],
  [*Time bounds*], [120 s outer process; TERM at 115 s plus 5 s grace; up to 110 s collection and 2 s per query],
  [*Other limits*], [Degree 1,024; 48 variables; 32 pairs/step; 4,096 solutions; 100,000 input terms; 1,000,000 signed lifts; rho cap 2,000,000 steps])

Pair-table setup is charged to cold setup and bounded by the outer envelope, not by the per-query clock. If a process dies before its setup report, missing partial counters stay unavailable. The inherited USD 5 aggregate ceiling includes earlier lane expenditure; unknown remaining headroom blocks spending.

#note[*Correct success condition.* From $A ell=alpha+beta s$, a left dependency with $lambda^T A=0$ and $lambda^T beta!=0$ gives $s=-(lambda^T alpha)/(lambda^T beta) mod n$. Full factor-base rank is not required. Replay $[s]G=Q$ on the original target, and on the destination when transported.]

#pagebreak()
= Controls and twist adaptation

== Which comparisons answer which question?

#table(columns: (1.3fr, 2.7fr),
  table.header([*Control or arm*], [*What it isolates*]),
  [Standard vs sparse H2], [Same source, kernel, V, and final map; different intermediate equations and setup work.],
  [Same-base pair table], [The same signed point base and collector, using the maintained folded batched table and complement scan. Table setup remains charged.],
  [Matched plain base], [Same destination and usable-column count, with independently checked matching and its own setup.],
  [Three rho references], [Historical counted negation, counted v6, and tuned one-thread prime-fast; no global fastest-method claim.])

The pair adapter checks cancellation first, covering the identity pair sums omitted by the production table. It then probes signed complements and replays each literal three-point witness. A positive is *first-witness*, not complete enumeration. A complete negative requires the full scan; caps/timeouts remain unknown. The offline auditor checks returned witnesses and metadata, not every table entry, negative exhaustion, or operation count.

On the degree-three isogenous target $E_1:y^2=x^3+323891735x+1985248359$, reusing V builds a native base; transporting original points builds a different base. For H2, the retained counts are 515 on E, 533 native on E₁, and 515 transported. Exact relation-set transport preserves the scalar, but traversal and first-hit behavior can change execution.

== A target twist changes the rational problem

For nonsquare $d$, use
$ E^((d)):Y^2=X^3+d^2 a X+d^3 b, quad abs(E^((d))(bb(F)_p))=2(p+1)-abs(E(bb(F)_p)). $
In same-x coordinates, $d y^2=f_(E)(x)$ reuses V but flips square/nonsquare lifts when $f_(E)(x)!=0$. The monic model uses candidate coordinates $d V$. The fixed target's twist order is *2147520465*, so its rational group does not contain the original order-2147446831 subgroup. A new twist arm needs valid $(n,G,Q)$ and its own reference fixture.

If a normalized rational isogeny is $eta(x,y)=(r(x),y r'(x))$, twisting both endpoints gives
$ eta_(d)(X,Y)=(d r(X/d), Y r'(X/d)). $
This transports the map formulas; it does not transfer the original rational discrete-log problem automatically.

== An auxiliary twist needs rational kernel care

For $A_-^((d)):Y^2=X^3-d^2X$, the sparse prefix adapts to
$ delta_(d,x)(X)=(X^2+d^2)^2/(X(X^2-d^2)). $
The canonical extension-field twist map preserves only the four two-torsion points as rational images of H2. At this $p equiv 3 mod 4$, choose $u^2=-d$ in $bb(F)_p$: the rational isomorphism $(x,y) mapsto (u^2x,u^3y)$ preserves the whole rational kernel and maps V to $-d V$. A+ and A− are different *quartic* twist classes, not a quadratic-twist pair. These formulas are adaptation guidance; the current panel runs only E and E₁.

#pagebreak()
= Sources and a final checklist

== Evidence you can inspect

#set par(spacing: 0.53em)
#small[
*Implementation and exact fixtures.* #link(base-url + "native/src/kernel_fibers.rs")[kernel_fibers.rs] and #link(base-url + "native/tests/sparse_h2_contract.rs")[sparse_h2_contract.rs]; unchanged #link(base-url + "isogeny-choice/fixtures/kernel_correctness_fixtures.json")[kernel_correctness_fixtures.json].

*Protocol and accounting.* #link(base-url + "sparse-h2-followup/PROTOCOL.md")[PROTOCOL.md], #link(base-url + "sparse-h2-followup/ACCOUNTING.md")[ACCOUNTING.md], and #link(base-url + "sparse-h2-followup/BASELINE_REVIEW.md")[BASELINE_REVIEW.md]. Source snapshots and raw build/test attempts are retained in the adjacent evidence directory. The old 230-cell report and the longer tutorial remain historical companions.

*Native qualification.* #link(base-url + "sparse-h2-followup/DEVELOPMENT_RECEIPT_v2.json")[DEVELOPMENT_RECEIPT_v2.json]. Receipt status: #data.receipt_status. Receipt SHA-256: #linebreak() #path(data.receipt_sha256). The retained ERRATA.md explains the corrected decimal-field metadata; native arithmetic and binaries are unchanged.

*Identity record.* #data.certificate_id. Exact artifact path: #path(data.certificate_path). The native record uses `identity.certificate/v1`, `proof: null`, and unsigned producer replay. Its 2⁶¹−1 evaluation field and deterministic-seed limits are distinct from the Prime31 kernel proof.

*Mathematical derivation.* #link(base-url + "sparse-h2-followup/MATHEMATICS.md")[MATHEMATICS.md] gives the full point formula, normalization argument, exact-map equality, and twist-rationality distinction. Background: Andrew Sutherland, #link("https://ocw.mit.edu/courses/18-783-elliptic-curves-fall-2025/mit18_783_f25_lec05.pdf")[MIT 18.783 Lecture 5, isogeny kernels] and #link("https://math.mit.edu/classes/18.783/2021/LectureNotes7.pdf")[Lecture 7, §7.6, quadratic twists].

*Evidence standing.* Claim #data.claim_id binds the unsigned native receipt and identity record. Its PCC standing is unverified. A published claim record or a successful self-check does not create a signed verdict. No measured change is available to update a performance ranking; canonical performance figures remain unchanged.
]

== Work through the construction in this order

#set enum(spacing: 7pt)
+ *Separate the two curve roles.* The auxiliary A− creates V. The target E holds $Q=[s]G$.
+ *Write down the kernel.* Use $H_2=⟨2K,U⟩$, with generator orders 512 and 2. Its size is 1,024.
+ *See why a coset is one fiber.* For every $h in H_2$, $phi(S+h)=phi(S)$.
+ *Remove the four two-torsion points first.* Compose $x-1/x$ and $t+4/t$ to obtain $delta_x$.
+ *Use the correct remaining generator.* $delta(2K)$ has order 256; eight degree-two maps finish the quotient.
+ *Check equality before comparing cost.* Require the same full kernel, final curve, endpoint, V, dense N and D, and fiber polynomial.
+ *Choose a presentation explicitly.* Ten quadratics, five quartics, or one dense equation; the sparse quartic has five terms, while speed remains a question for measurement.
+ *Match controls fairly.* Give the plain arm its own setup; retain reference preparation as study expenditure. Charge pair-table construction and all replay work.
+ *Admit and screen, then freeze a recovery subset.* Keep every failure, timeout, and unknown outcome; five seeds still concern one target.
+ *Accept only replayed recovery and complete cost evidence.* Check $[s]G=Q$, preserve outer process observations and missing counters, and obtain independent signed standing before promoting a result.
