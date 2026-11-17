// Generated layout only. Source: REPORT.md and pipeline.svg.
#set document(title: "Point hashing: five implementation leads", author: "Isogeny Labs — proposed research")
#set page(paper: "a4", margin: (x: 17mm, y: 17mm), numbering: "1",
  header: context text(size: 8pt, fill: rgb("52677d"))[Point hashing · proposed designs · 2026-11-16])
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false, leading: 0.65em)
#set heading(numbering: none)
#show heading.where(level: 1): set text(size: 20pt, fill: rgb("16395b"))
#show heading.where(level: 2): set text(size: 13pt, fill: rgb("16395b"))
#show raw: set text(size: 7.6pt)
#show link: set text(fill: rgb("176293"), size: 8pt)
#show image: it => block(breakable: false, inset: 2pt, it)
#heading(level: 1)[#text("Point hashing: five implementation leads")]

#text("Date: 2026-11-16 UTC. Status: proposed research designs, not measured results.")

#text("The strongest next steps are to remove repeated subgroup certification from") #text("internally generated queries, batch affine subtraction, and exploit the special") #text("graph structure of two-slot relations. A second tier is to select useful cycles") #text("and generate targets with short, independently anchored adding walks. These are") #text("integration hypotheses using established primitives; this report makes no") #text("novelty, runtime, significance, or challenge-recovery claim. [pcc:unverified]")

#text("This is a planning-only change. It does not modify the native solver, pending") #text("point-table PRs, the ECBench branch, frozen pilot evidence, registrations,") #text("canonical claim history, or experiment budget. No scientific job was launched.")

#heading(level: 2)[#text("1. Scope and source audit")]

#text("The immediate domain is the existing short-Weierstrass prime-field native") #text("solver, a certified prime-order subgroup of order r, and two independently") #text("configured factor-base slots. Every vertex, witness and cancellation retains its") #text("slot identity. No implementation or performance conclusion is transferred to") #text("the unfinished binary challenge pipeline or a Certicom challenge.")

#text("Source pins for this search:")

#list(tight: false, [#text("Crypto native feature snapshot: ")#raw("28b33d4a8c8a09b6c08e27a17e30ddacb3586cc0")#text(",") #raw("crates/hashed-ecdl-tensor/src/")#text(" [S1]. This is a feature snapshot, not a claim") #text("that the pending feature has landed on main.")], [#text("Cryptanalysis mirror snapshot: ")#raw("fdfc85c73307e113fcc5f4829e02f9914a575c12")#text(",") #raw("bindings/rust/hashed-ecdl-tensor/src/")#text(" [S2]. Mirror integration is outside") #text("this search; port eligibility must be checked again before implementation.")], [#text("Initial planning base: ")#raw("b478a9770ea35f84bddf4e9b9ffbf7efeab30f06")#text(" [S3].")], [#text("Lead-atlas snapshot: ")#raw("96f010d853a088d6ff5093ebedc4270b013484dd")#text(" [S4].")])

#text("Producer source observations, with exact paths and digests in") #raw("source-inputs.sha256")#text(": [pcc:unverified]")

#list(tight: false, [#raw("prepared.rs:152")#text(": each prepared query calls ")#raw("in_subgroup")#text(", then scans one") #text("selector-ordered base, computes a complement, performs a full-point lookup,") #text("and replays arithmetic membership and the group sum.")], [#raw("curve.rs:265")#text(": subgroup checking multiplies the point by r. ")#raw("add:273") #text("uses checked affine arithmetic, with an inversion in ordinary addition.")], [#raw("reduction.rs:799")#text(": the collector allocates a dense matrix for N+1 rows,") #text("with N the sum of the slot cardinalities. Every attempt draws fresh u,v.") #text("At ")#raw("847")#text(", only the first dependency is tried; zero combined v discards") #text("the trial. The verifier at ")#raw("945")#text(" demands exactly N+1 rows.")], [#raw("partial.rs:1")#text(": the existing partial-relation path uses sparse elimination") #text("over the odd subgroup field. Its own comment distinguishes this from binary") #text("graph-cycle cancellation. It is an important control, not a missing feature") #text("to relabel as new. ")#raw("joint.rs")#text(" already shares public coefficient doubling") #text("chains; simultaneous multiplication itself is not a new lead here.")])

#text("The research-KB MCP tools were absent. The repository wrapper's status and") #text("search commands failed because no executable was available. The lead-atlas") #text("wrapper also failed on an unavailable orchestration import. Direct pinned Git") #text("reads supplied the fallback; an empty sparse directory or failed wrapper was") #text("not interpreted as absence of prior work. Conductor was attempted and absent.")

#text("Atlas dispositions materially changed the shortlist [S4]:")

#list(tight: false, [#raw("LEAD-RHO-batched-inversion-shared-pm")#text(": closed-known. L2 below investigates") #text("its placement inside prepared decomposition, not discovery of batch inversion.")], [#raw("LEAD-RHO-r-adding-walk-mixing")#text(": closed-known. L5 investigates useful") #text("relation dependencies under correlated targets, not a new rho walk constant.")], [#raw("LEAD-ICE-relation-graph-rank-large-prime")#text(": open-next-step. L3/L4 are a") #text("narrower integration question; rank-aware collection and cycle enrichment") #text("are already in the programme.")], [#raw("LEAD-ICP-generic-table-decomposition")#text(": closed-obstructed. These generic") #text("table optimizations do not establish a new asymptotic ECDLP algorithm.")], [#raw("LEAD-ICB-frobenius-orbit-factor-base")#text(": closed-obstructed. Ordinary orbit") #text("folding is excluded from the new shortlist; no second gain is established.")])

#text("Primary reading: Bos, Kaihara and Montgomery, SHARCS 2009, sections 3.4-3.5") #text("and 4.3 [P1], and Godsil and Guo, section 2 and the bipartite discussion in") #text("section 3 [P2]. The Teske paper [P3] was located but direct full-text retrieval") #text("returned HTTP 403; [P1] is the actually read source for the adding-walk") #text("construction. Atlas numerical summaries were not independently replayed here.")

#heading(level: 2)[#text("2. Ranked leads and dependency map")]

#text("Priority is an engineering judgment, not a measured ranking. L1 and L2 can") #text("preserve the current target stream and first-witness contract. L3 starts with") #text("the existing certificate format. L4 and L5 deliberately change search policy") #text("and need separate controls.")

#enum(tight: false, [#text("L1: carry sealed subgroup provenance through internal target construction.")], [#text("L2: batch the affine denominators used by complement queries.")], [#text("L3: a weighted forest and cycle backend for exactly two slots.")], [#text("L4: prefer useful cycle opportunities with bounded extra witness work.")], [#text("L5: two-dimensional adding walks between independent target anchors.")])

#image("pipeline.svg", width: 100%)

#text("Figure 1. Dashed boxes are proposed paths. Solid boxes are existing replay") #text("obligations, not independently signed execution evidence. There are no") #text("measured quantities in the figure. All routes must finish with relation") #text("cancellation and scalar replay; a hash lookup alone is insufficient.")

#heading(level: 2)[#text("3. L1: retain subgroup provenance")]

#text("Hypothesis: repeated order-r multiplication can be avoided when a query is") #text("constructed exclusively from already validated subgroup points. Hashing is") #text("an index, never a proof of point validity.")

#text("Design an internal opaque ")#raw("SubgroupPoint")#text(" tied to a specific validated curve") #text("context. Only validated input or checked operations on that same context may") #text("create it. Keep the public untrusted-point API's subgroup check. Do not add") #text("an unchecked public constructor, serialized trusted token, or caller-controlled") #text("boolean. A Rust lifetime alone does not identify a curve; context binding") #text("must distinguish two curve objects that happen to share a lifetime.")

#text("The collector validates Q and already knows certified G. Prepared and joint") #text("target operations could preserve the internal capability. Only that internal") #text("query path may elide the repeated subgroup multiplication. Membership checks,") #text("full point equality, relation sum replay and final scalar verification stay.")

#text("Controls: same seeded target/witness stream against the checked API; wrong") #text("curve context; noncanonical coordinates; outside-subgroup points on cofactor") #text("fixtures; identity and exceptional additions; attempted token forgery through") #text("every exposed constructor. Document which checks occur at each boundary.")

#text("Accounting: count subgroup multiplications, context construction, copies,") #text("normalizations and all retained replays separately. Eventually compare full") #text("cold setup, query and verification cost, not just the trusted call.")

#text("Reject or redesign if an untrusted point can enter the path, errors change") #text("into misses, or context overhead removes the full-pipeline benefit. This is") #text("the first implementation candidate because its search semantics can stay fixed.")

#heading(level: 2)[#text("4. L2: batch complement subtraction")]

#text("Hypothesis: scan a chunk of prepared P values and share inversion across the") #text("ordinary affine additions computing R-P. Montgomery simultaneous inversion") #text("is established prior art [P1, section 3.5]; its benefit in this early-exit") #text("query is unmeasured.")

#text("Proposed chunk sizes are 1 (control), 8, 32 and 128. Preserve selector order") #text("and choose the same first full witness. Partition exceptional cases before") #text("batch inversion: identity inputs, equal x coordinates, R=P, R=-P, doubling") #text("and zero denominators. Use the current checked arithmetic for those cases.") #text("Only nonzero denominators enter the batch. Both signed point coordinates") #text("must be materialized before the existing full-point lookup and replay.")

#text("The source's arithmetic model replaces k independent inversions by one") #text("inversion and 3(k-1) multiplications for the inversion stage. This is a cited") #text("operation-count model, not measured runtime or total addition cost. Slope") #text("arithmetic, zero handling, coordinates, allocation, hashing and unused tail") #text("work after an early hit must all be charged.")

#text("Controls: exhaustive small fixture queries; identical first witness and") #text("misses with uncapped resources; forced table hash collisions; exceptional") #text("points; chunk boundaries; limits that interrupt allocation or arithmetic.") #text("Budget exhaustion stays unknown/error, never a certified negative. Test") #text("failure semantics explicitly because batching changes work ordering.")

#text("Stratify early-hit, late-hit and no-hit queries. Reject a universal chunk-size") #text("claim if early-hit overhead dominates; a deterministic threshold policy may") #text("be useful, but must be chosen on training fixtures and frozen for held-out") #text("ones. Compare with the best available same-computation native batch primitive,") #text("not just an intentionally unbatched implementation.")

#heading(level: 2)[#text("5. L3: use the two-slot relation graph")]

#text("Hypothesis: replace dense row/provenance elimination in the two-slot special") #text("case with a spanning forest and coefficient potentials. The graph incidence") #text("and cycle-space connection is established [P2]. Its finite-field, parallel-edge") #text("and certificate integration here is a proof and implementation obligation.")

#text("Create a vertex for each ")#raw("(slot, canonical point)")#text(". An accepted two-point") #text("relation is an edge carrying u,v and a reference to its original witness.") #text("Keep distinct slot vertices even for equal physical points. Orient edges") #text("from slot zero to slot one; account for the slot sign convention explicitly.") #text("Use modular potentials in a weighted union-find structure to screen closing") #text("edges. Keep a separate immutable spanning forest for reconstructing signed") #text("relation provenance. Avoid reconstructing every long path for a null cycle.") #text("This is not XOR cancellation over an odd-order field and does not directly") #text("handle three or more slots.")

#text("Stage A preserves N+1 successful relations and the strict v1 verifier.") #text("Compare the first dependency, normalization and certificate bytes where") #text("possible against the dense backend on identical ordered relation streams.") #text("If exact matching fails for a legitimate choice of dependency, document and") #text("test that change rather than asserting byte equality.")

#text("Stage B tries additional fundamental-cycle dependencies when the first has") #text("zero combined v, before rebuilding bases. This isolates a current discard") #text("policy from any early-stopping proposal. Replay all accepted combinations.") #text("The proposed completeness obligation is that the retained cycle basis spans") #text("all dependencies, so a useful dependency is not missed merely by choosing") #text("the first cycle. It is not yet certified by this planning round.")

#text("Stage C is a separate certificate v2 for short, useful cycles and early") #text("stopping. Do not relax v1's exact N+1 rule. Specify bounded row counts,") #text("slot identities, membership, group equations, modular cancellation, nonzero") #text("combined v and final scalar replay. Whether v2 can avoid full base") #text("enumeration is a separate soundness question, not an assumed saving.")

#text("Controls: dense elimination and existing odd-order sparse elimination;") #text("parallel edges; disconnected components; isolated columns; same point in") #text("both slots; first cycle unusable but a later one useful; all cycles unusable;") #text("malformed signs/provenance; resource limits; outside-two-slot rejection.") #text("Measure full storage including witnesses, BigUint labels, forest, cycle") #text("reconstruction and verification. Sparse graph bookkeeping alone is not an") #text("end-to-end cost verdict.")

#heading(level: 2)[#text("6. L4: useful-cycle-aware witness choice")]

#text("Hypothesis: after L3, a bounded number of alternative decompositions per") #text("target could produce useful closing cycles sooner than always taking the") #text("first witness. It may also lose because extra subtractions cost more than") #text("the saved target attempts. This overlaps the atlas's rank-feedback lead;") #text("the new task is a precise bounded policy in the prepared two-slot API.")

#text("Compare extra-candidate caps 0 (current first hit), 4 and 16. A graph oracle") #text("may expose component IDs and edge structure to the selector, but never the") #text("planted scalar or target coefficients u,v. Evaluate combined v in the") #text("collector after a witness is chosen; a closed cycle is not automatically") #text("informative. Version the changed witness policy and record deterministic") #text("tie-breaking and every rejected or alternate candidate.")

#text("Use the identical target stream, factor bases and limits for all policies.") #text("Run a shuffled/rewired graph control preserving candidate counts and degree") #text("information where appropriate; it is a null model, not substitute solver") #text("data. Record useful cycles, zero-v cycles, repeated targets, extra lookups,") #text("replays and cold scalar recoveries. All generated candidate work belongs in") #text("the denominator. Stop if no held-out total-cost benefit survives the cap.")

#text("This lead depends on L3's correctness and cannot be justified solely by an") #text("increase in raw relation yield or graph cycle count.")

#heading(level: 2)[#text("7. L5: two-dimensional target walks")]

#text("Hypothesis: use an established adding-walk construction [P1, section 3.4]") #text("to reduce repeated target scalar multiplication between independent anchors.") #text("Define prepared increments Delta_i = a_i G + b_i Q; a step updates R by") #text("Delta_i and updates the public labels by a_i,b_i modulo r. Choose the step") #text("from a domain-separated hash of a canonical point. The relation oracle") #text("receives points, not coefficients or the planted secret.")

#text("Draft controls: independent uniform targets with the existing joint path;") #text("adding tables of 16 and 32 increments; independently anchored blocks of") #text("16, 64 and 256 steps. Freeze exact tables, anchors, seed derivation, curve") #text("files and restart rules before any run. These values are proposed design") #text("choices, not measured optimal settings or a frozen protocol.")

#text("Critical negative control: repeatedly adding G keeps v constant. The") #text("proposed degeneracy lemma to establish is that slot cancellation then also") #text("cancels combined v, preventing extraction through that dependency. More") #text("generally, test a single fixed nonidentity increment as a deliberately") #text("one-dimensional label policy. Do not confuse this with a genuine labelled") #text("point collision: an identity increment with a useful coefficient difference") #text("may already encode a solution and must be replayed, not silently excluded.")

#text("Use a two-dimensional increment-label design, but do not impose a universal") #text("rank-three rule on accepted relations: a useful pair collision can solve") #text("with only two rows. Label-span diagnostics are not a validity criterion.") #text("The fixed-v lemma, finite-field graph reduction and provenance closure are") #text("explicit proof obligations. No new algebraic identity certificate was issued") #text("here; they remain unverified rationale, not certified theorems.")

#text("Record target autocorrelation, repeated states, trapping/restarts, coefficient") #text("degeneracy, useful cycles, relation yield and total work. Do not transfer") #text("an iid success theorem to correlated targets. Charge increment construction,") #text("anchor multiplications, canonical hashing, reseeding, replay and verification.") #text("Reject if cheap steps only generate redundant or unusable relations, or if") #text("independent anchors erase the full-pipeline benefit.")

#heading(level: 2)[#text("8. Experiment design and decision gates")]

#text("These are designs, not an execution-ready preregistration. Exact fixture") #text("files, subgroup parameters, seeds, commands, limits and source hashes are") #text("still pending. Before implementation, recheck current source and relevant") #text("prior results, specify the proof obligations above, and keep each changed") #text("search policy separate from semantics-preserving arithmetic work.")

#text("Recommended sequence:")

#enum(tight: false, [#text("Implement L1 and L2 independently in Rust, preserving the checked API and") #text("selector order. Port the same interface and controls to the mirror.")], [#text("Qualify exact combined source with native correctness, negative controls,") #text("lint and supported toolchains. Distinguish operation counts from runtime.")], [#text("Implement L3 A/B on recorded identical edge streams. Version v2 separately") #text("only after its soundness contract and malformed-certificate controls exist.")], [#text("Freeze L4 and L5 independently; combine only after each has a held-out win.")], [#text("For a proposed runtime study, freeze a matched panel and compare the full") #text("cold pipeline with the current solver and strongest applicable generic") #text("reference. Include setup, failed attempts, memory, verification and I/O.")])

#text("Use the frozen standard panel's prime rungs P24a, P32a, P36a and P40a") #text("(")#raw("docs/ic/STANDARD_PANEL.md")#text(", ")#raw("standard_panel.json")#text("); pin each ICV1 model") #text("and subgroup before a run. Start with the smallest admitted rung, with larger") #text("rungs gated by cost and memory. Tiny exhaustive and cofactor fixtures are") #text("off-panel correctness controls, not performance evidence. The independent") #text("unit is a public curve/target seed; use disjoint training/held-out seeds,") #text("preserve failures/timeouts, and report completion probability, paired cold") #text("cost and peak memory. Never select only successful relations or seeds.")

#text("A suggested practical success threshold to freeze before execution is a") #text("paired held-out cold-cost reduction of at least 10%, with the uncertainty") #text("interval excluding a zero reduction, unchanged scalar correctness, and an") #text("explicit memory ceiling. This is a proposed decision rule, not a significance") #text("result. If the admitted budget cannot resolve it, report inconclusive rather") #text("than extending the sample adaptively until a win appears. A predeclared memory") #text("reduction at equal throughput can be a separate useful outcome.")

#text("Launch gates remain fresh current-source S3 checkpoint upload/readback and") #text("authenticated exact native execution/image/L2 admission. This search did not") #text("retest cloud access and did not use chat credentials. The existing first-pilot") #text("budget authorization is retained; it does not supply missing execution identity.") #text("Neither PR integration nor a rendered PDF satisfies these run gates.")

#heading(level: 2)[#text("9. Deliverables, graph status and open work")]

#text("This report, editable DOT, SVG, rendering source and PDF are the search") #text("deliverable. ")#raw("source-inputs.sha256")#text(" pins read native inputs, and") #raw("retrieval.md")#text(" records unavailable retrieval paths. Mathematical proof,") #text("implementation, experiments, independent replay and signed claim standing") #text("remain pending. No performance record or canonical producer row is added.")

#text("Canonical graph inventory reviewed by tracked path: ")#raw("docs/index-calculus-scoreboard.html")#text(",") #raw("docs/ic/progress-timeline.json")#text(", ")#raw("docs/curves/")#text(", ")#raw("docs/performance-gains/")#text(",") #raw("figures/")#text(", and the prepared/joint study reports. All stay unchanged: the") #text("search proposes routes and adds no new measured point, evidence-backed edge or") #text("correction to an existing graph. No benchmark or leaderboard row is appropriate.")

#text("The pending point-table integration tracker remains separate. The original") #text("pilot, frozen matched A/B panel, profiling, binary pipeline, challenge scalars") #text("and independent full audit are not closed by this plan.")

#heading(level: 2)[#text("References")]

#set text(size: 8pt)

#list(tight: false, [#text("[S1] Crypto feature source: https://github.com/isogeny-labs/crypto/tree/28b33d4a8c8a09b6c08e27a17e30ddacb3586cc0/crates/hashed-ecdl-tensor/src")], [#text("[S2] Cryptanalysis mirror: https://github.com/isogeny-labs/cryptanalysis/tree/fdfc85c73307e113fcc5f4829e02f9914a575c12/bindings/rust/hashed-ecdl-tensor/src")], [#text("[S3] Planning base: https://github.com/isogeny-labs/crypto/commit/b478a9770ea35f84bddf4e9b9ffbf7efeab30f06")], [#text("[S4] Programme atlas: https://github.com/isogeny-labs/crypto-autoresearcher/tree/96f010d853a088d6ff5093ebedc4270b013484dd/knowledge/leads/ecdlp")], [#text("[P1] J. W. Bos, M. E. Kaihara, P. L. Montgomery, \"Pollard Rho on the") #text("PlayStation 3\", SHARCS '09 record, pp. 35-50, especially sections 3.4,") #text("3.5, 4.3. Read construction and arithmetic model, not transferred timings.") #text("https://www.hyperelliptic.org/tanja/SHARCS/record2.pdf")], [#text("[P2] C. Godsil, K. Guo, \"Cycle Spaces of Digraphs\", arXiv:1609.09118v1,") #text("section 2 (incidence/flow space), section 3 (bipartite sign change).") #text("The paper's stated real/complex-space results do not replace a native") #text("finite-field multigraph correctness argument for this solver.") #text("https://arxiv.org/html/1609.09118")], [#text("[P3] E. Teske, \"On random walks for Pollard's rho method\", Mathematics") #text("of Computation 70 (2001), 809-825, DOI 10.1090/S0025-5718-00-01213-8.") #text("Located, full text blocked by HTTP 403; construction read through [P1].") #text("https://doi.org/10.1090/S0025-5718-00-01213-8")])
