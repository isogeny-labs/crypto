#set page(
  paper: "a4",
  margin: (left: 17mm, right: 17mm, top: 16mm, bottom: 22mm),
  header: [#text(size: 8pt, fill: rgb("5b6f7b"))[ISOGENY LABS / RR5 B677 CIRCUIT QUALIFICATION]],
  footer: context [#h(1fr)#text(size: 8pt, fill: rgb("5b6f7b"))[#counter(page).display("1")]],
)
#set text(font: "DejaVu Sans", size: 9pt, fill: rgb("172d3b"))
#set par(justify: false, leading: 0.55em, spacing: 0.7em)
#set heading(numbering: none)
#show heading.where(level: 1): set text(size: 19pt, weight: "bold", fill: rgb("165e7c"))
#show heading.where(level: 2): set text(size: 12pt, weight: "bold", fill: rgb("165e7c"))
#show heading.where(level: 3): set text(size: 10pt, weight: "bold")
#show raw: set text(font: "DejaVu Sans Mono", size: 7.5pt)
#show link: set text(fill: rgb("165e7c"))
#set table(inset: 4pt, stroke: 0.35pt + rgb("c7d3da"))
#set image(width: 100%)
= RR5 B677 selector circuit: construction and qualification
<rr5-b677-selector-circuit-construction-and-qualification>
2026-11-22 Pacific; execution receipts use UTC. Continuation of #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/REPORT.md")[the four-summand study];, initially tracked in #link("https://github.com/isogeny-labs/crypto/pull/4967")[PR \#4967];. This report distinguishes a public algebraic construction, local correctness controls, and the registered B677 extraction requests. The #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/PROTOCOL.md")[protocol] fixes the requests and limits. Evidence is unsigned and independently reviewed only within the same team. \[pcc:unverified\]

The source and raw evidence are published at #link("https://github.com/isogeny-labs/crypto/commit/5e536ed93302def2122968aa880b22c4b675d516")[commit 5e536ed93302];. Its Git tree is identical to the reviewed local publication tree. The separate #link("https://github.com/isogeny-labs/crypto/actions/runs/45118811731")[GitHub qualification run] was observed in progress on that exact commit; its conclusion is not part of these local test receipts.

== 1. Observed standing
<observed-standing>
The corrected native suite passed #strong[all 32 tests];, and the separate public-example suite passed #strong[all eight];, with none ignored or filtered out. The native constructor observed 122 variables, 123 quadratic equations and 686 terms, and rejected all 476 individual auxiliary mutations. The fixed B677 driver compiled successfully. No registered B677 extraction request has run: the checkpoint gate failed, and an authenticated EKS route for this exact executable remains unavailable.

The first build’s suite had #strong[27 passes and five failures];, all caused by the same rejected Y7 fixture constant before its B677 assertions. That failure, the independent scalar diagnostic and the corrected rerun are preserved separately.

#figure(
align(center)[#table(
  columns: 3,
  align: (col, row) => (auto,auto,auto,).at(col),
  inset: 6pt,
  [Gate], [Observed outcome], [Evidence],
  [Native and public-example compilation, build 01],
  [Exit 0; source and frozen-binary checks passed],
  [#link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/build-01/exit-status.txt")[Build receipt];, #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/build-01/source-manifest.json")[source manifest];],
  [Native suite 01],
  [Exit 101; 27 passed, 5 failed, none ignored],
  [#link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/native-tests-01/stdout.txt")[Output];, #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/native-tests-01/stderr.txt")[failures];],
  [Independent coset-7 diagnostic],
  [Exit 0; two scalar paths agree on 1852957275],
  [#link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/coset7-check-01/stdout.txt")[Diagnostic output];, #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/controls/coset7.rs")[native source];],
  [Checkpoint admission],
  [Exit 1; checkpoint upload unavailable],
  [#link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/admission-01/stderr.txt")[Failure receipt];],
  [Corrected build 02 and native suite 02],
  [Exit 0; 32 passed, none failed, ignored or filtered],
  [#link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/build-02/exit-status.txt")[Build];, #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/native-tests-02/stdout.txt")[native output];],
  [Public-example execution],
  [Exit 0; all 8 passed, none ignored or filtered],
  [#link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/public-example-01/stdout.txt")[Public output];],
  [Fixed-driver compilation],
  [Exit 0; driver not launched],
  [#link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/driver-build-01/command.txt")[Build command];, #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/driver-build-01/exit-status.txt")[exit];],
  [Full root-library gate],
  [Exit 124 at the 180-second limit; AWS SDK metadata allocation failed before timeout; no tests executed],
  [#link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/root-library-01/command.txt")[Command];, #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/root-library-01/stderr.txt")[diagnostic];, #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/root-library-01/exit-status.txt")[exit];],
  [Registered B677 positive/reference/H0/W0 calls],
  [Unexecuted],
  [#link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/ADMISSION.md")[Admission record];],
)]
)

The first fixture failure was #strong["public Y7 coset 7: y0^7 differs from its frozen constant."] Four fixture/API controls and the circuit constructor test aborted at this check. The failed run therefore supplies no observation of the circuit term count or its 476 planned auxiliary-mutation checks. Source and binary post-call checks both returned zero, so the failure belongs to the recorded immutable build rather than a changed executable.

The #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/coset7-check-01/stdout.txt")[independent native diagnostic] subsequently computed the seventh power of 136769428 modulo 2147483647 by repeated multiplication and a separate addition chain. Both returned #strong[1852957275];. The earlier source spelling was correct; the later 1852955275 spelling was a typo, and the first implementation had reversed that inference. The #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/SOURCE_CORRECTION.md")[source correction] changes only the new fixture constant and its independent assertion, retaining the wrong value as a negative assertion. Historical sources, the CSV, protocol targets and limits remain unchanged. The corrected suite reran the same 32 tests successfully.

#figure([#box(width: 100%, image("figures/construction.svg"));],
  caption: [
    Exact selector construction and the unexecuted extraction gate
  ]
)

Figure: algebraic dependencies, observed local controls and admission boundaries. Variable/equation counts are analytic source quantities, confirmed for the circuit by its constructor control. Arrows beyond admission describe the registered procedure, not an executed B677 solve. There is no comparative timing graph because this continuation has no benchmark samples.

== 2. Exact original base and selector conditions
<exact-original-base-and-selector-conditions>
The requested instance is the original PRIME31 curve $ p = 2147483647 , quad y^2 = x^3 + 847514846 x + 2087444724 , quad \# E (bb(F)_p) = 2147446831 . $ The #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_ic_20261115/native/lib.rs")[original arithmetic] validates a nonidentity generator of prime order and a Hasse-bound condition establishing full curve order. The canonical base combines the depth-ten Dickson fibre’s #strong[509 rational nonzero-y abscissae] with #strong[168 frozen Y7 abscissae];, with canonical signs and sorted distinct x-values.

The extra CSV is #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_f4_admission_20261121/RUN66_Y7_EXTRA168.csv")[RUN66\_Y7\_EXTRA168.csv];, SHA-256 `fc484615394ee423190747d05001cea526580fb0df0279fbcf789ab6cafc0c46`. A natural-x B677 scan would be a different experiment.

Write $f = X^3 + A X + B$, $D_1 = X$, $D_(2 m) = D_m^2 - 2$, and $ H = (D_1024 - tau) G (f^7) , #h(2em) G (T) = product_(i = 1)^8 (T - c_i^2) . $ The raw selector has degree $1024 + 168 = 1192$; the filtered base polynomial $h = product_(x in S) (X - x)$ has degree 677. The raw Dickson fibre includes 515 noncurve x-values.

The #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/native/b677_fixture.rs")[fixture constructor] requires 1,024 distinct Dickson roots, checks their selector evaluations and exact rational filtering, and validates the extra roots against the unchanged CSV. For each Y7 coset, it checks the complete degree-21 coefficient factorization, distinctness, disjointness from the full Dickson fibre, and signed ordinates. Its additional horizontal-triple checks concern public setup identities. They do not supply extracted target relations. These checks must pass before the structured candidate accepts the descriptor.

The theorem requires $H$ to split simply over the field and its nonzero-y on-curve roots to equal the supplied base exactly. It also requires the #strong[full rational group] to have odd order, a finite target $R = (r , s)$, and $H (r) eq.not 0$. The first build failed fixture admission; the corrected fixture and independent replay controls passed in #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/native-tests-02/stdout.txt")[suite 02];. Their success remains unsigned execution evidence for these exact public data.

== 3. Why the additional roots preserve the RR5 chart
<why-the-additional-roots-preserve-the-rr5-chart>
For three coefficient parameters $b , c , d$, set $ V = X + b , quad L = c X + d , quad U = (X^2 + r X + r^2 + A) V^2 - (X - r) L^2 - 2 s L V . $ The monic quartic satisfies $ f V^2 - #scale(x: 120%, y: 120%)[\(] (X - r) L + s V #scale(x: 120%, y: 120%)[\)]^2 = (X - r) U . $ The candidate enforces $U divides H$. Squarefreeness and target exclusion imply $gcd (U , V) = 1$: a common root would force either the excluded target abscissa or a repeated factor of $U$. Thus every quartic root $z$ has the unique signed lift $ y_z = - s - frac((z - r) L (z), V (z)) . $ The associated function has four lifted zeros and $- R$, with pole divisor $5 (O)$, so the four leaves sum to $R$.

The extra geometric point issue is resolved by the four-leaf argument in #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/MATHEMATICS.md")[MATHEMATICS.md];. Points with x in $bb(F)_p$ are rational or anti-Frobenius. Their two subgroups intersect in rational two-torsion, hence only in $O$ here. The sum of anti-Frobenius leaves must therefore be zero. One such leaf would be $O$; two would share x; three would force the remaining leaf to be $R$; four would force $R = O$. Each alternative violates an admitted condition. All four leaves consequently lie in the canonical rational base, even for coefficient solutions initially considered over the algebraic closure.

The note also proves reducedness using the tangent equations for squarefree divisibility and the pole bound on a varying normalized RR5 function. Consequently the four remainder ideals $I_H$ and $I_h$ agree as affine schemes. No parameter saturation or denominator inversion is added. These are written, unsigned arguments. The earlier evaluated RR5 norm certificate does not certify this selector replacement, reducedness, or its implementation. \[pcc:unverified\]

== 4. Exact circuit and analytic dimensions
<exact-circuit-and-analytic-dimensions>
Four quadratic equations bind explicit quartic coefficients $z_j$ to the RR5 expressions. A modular product of cubic residues introduces a cubic remainder $C$ and a quadratic quotient $Q$, then imposes all seven coefficients of $ A B - hat(U) Q - C = 0 , #h(2em) hat(U) = X^4 + z_3 X^3 + z_2 X^2 + z_1 X + z_0 . $ The monic high coefficients determine all three quotient coefficients with unit pivots; the remaining equations determine the four remainder coefficients. Every acyclic gate is therefore a polynomial graph extension over arbitrary coefficient algebras. Special degree drops introduce no exceptional division.

The #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/native/selector_circuit.rs")[circuit] inlines $D_4 med mod med U$, uses eight squarings to reach $D_1024$, four products for $f^7$, and four products for the degree-eight blocked evaluation of $G$. Its final product uses only three quotient variables and imposes all seven coefficients with remainder fixed to zero.

#figure(
align(center)[#table(
  columns: 3,
  align: (col, row) => (auto,right,right,).at(col),
  inset: 6pt,
  [Analytic source quantity], [Structured selector], [Exact cofactor reference],
  [Selector/base polynomial degree],
  [1192],
  [677],
  [Original RR5 parameters],
  [3],
  [3],
  [Additional variables],
  [$4 + 16 dot.op 7 + 3 = 119$],
  [673],
  [Total variables],
  [#strong[122];],
  [#strong[676];],
  [Equations],
  [$4 + 17 dot.op 7 = bold(123)$],
  [#strong[677];],
  [Maximum input total degree],
  [#strong[2];],
  [#strong[3];],
  [Monomial ordering],
  [Grevlex],
  [Grevlex],
)]
)

For the reference, $W = X^673 + sum_(j = 0)^672 q_j X^j$, and all 677 lower coefficients of $U W - h = 0$ are imposed. Both presentations encode the same admitted chart. Lower input degree and fewer variables establish no solving-degree, matrix-size, or speed bound. The circuit may still have difficult elimination and auxiliary directions at infinity after naive homogenization.

The constructor control independently forms the dense degree-1192 selector and performs scalar monic division at four fixed parameter triples. It compares the final remainder and perturbs each of 119 auxiliary coordinates once per triple. Its first invocation stopped at fixture admission. The corrected #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/native-tests-02/stdout.txt")[B677\_CIRCUIT record] reports #strong[122 variables, 123 equations, 686 terms, maximum total degree two, 17 modular product gates, four parameter controls and 476 rejected auxiliary mutations];. Its 1,899 symbolic additions and 1,139 symbolic multiplications count constructor work only. These are neither extraction-operation totals nor a timing comparison. No coefficient witness enters a registered extraction call.

== 5. Bounded F4 and regression scope
<bounded-f4-and-regression-scope>
The #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/src/cryptanalysis/f4_fp.rs")[production F4 changes] add opt-in admission for critical pairs, symbolic support, retained polynomial terms, matrix workspace, recording copies and contributor storage. Checks precede the corresponding growth. Bounded calls use guarded scalar final interreduction. Optional modes without admission coverage are explicitly refused.

A resource stop carries a diagnostic limit and remains incomplete. Native traversal stops before another root branch or solution callback; the ordinary solver returns `Undetermined`. The #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/native/extractor.rs")[extractor] distinguishes `ResourceLimit` from `Empty` and bypasses the univariate-availability diagnostic for an already interrupted native call. Defaults retain the prior algorithms and serialization when resource limits are absent.

The frozen limits are degree 10, a shared 30-second construction/solver/root deadline, 20,000 terms per intermediate construction polynomial, 4,096 assignments, 20,000 admitted pairs, 8,192 symbolic monomials, and 2,000,000 conservatively reserved matrix entries. Execution uses one worker, an outer 180-second timeout, and a 2 GiB address-space limit. The native counters bound admitted populations; caller storage, exponent widths and allocator overhead still require the process bound.

All eight new guard controls passed in both native suites: bounded/default equality, pair refusal, symbolic growth, recorded-workspace admission, replay under current limits, final-reduction expansion, unsupported modes, and deadline classification. The nineteen inherited native tests also passed, including both fixed B8 oracle gates and extractor-to-adapter checks. #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/native-tests-02/stdout.txt")[Suite 02] additionally passed four B677 fixture/API controls and the circuit constructor, giving 32 distinct tests. The zero-budget API control refuses before algebraic search; it does not execute a registered positive or public-target request.

Their scope remains unchanged. The F11 known-answer schedule recovers scalar 3 at rank two and ends at rank three. The PRIME31 B4 adapter boundary ends at rank one with recovered scalar `null`. The B8 controls compare against 1,120 test-only signed subsets per target. These small controls do not establish B677 extraction, rank, or complete ECDLP cost. The separate #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/public-example-01/stdout.txt")[public-example run] passed eight tests covering B4 extraction, exact signs/RHS, incomplete branches, invalid inputs and target-in-base exclusion.

== 6. Provenance and execution admission
<provenance-and-execution-admission>
#link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/build-01/command.txt")[Build 01] used Rust 1.100.0 and the actual production F4 closure plus the study modules. Its #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/build-01/pre-command-sources.sha256")[pre-build inventory] binds 69 files; the #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/build-01/post-command-source-check.txt")[post-build check] preserved those bytes. The #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/build-01/source-manifest.json")[source-selection manifest] identifies complete modules and exact verbatim dependency ranges. It explicitly does not claim a complete hermetic build or a full root-library pass.

The #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/build-02/pre-command-sources.sha256")[corrected build inventory] again binds 69 files. Both builds, the two native suite calls and the public-example call retain successful source/binary checks. Native and public-example executables were copied read-only and #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/build-02/frozen-binaries.sha256")[hashed separately];. The final native test binary has SHA-256 `a59a6d046edeef795cf13186da0af8c8d5cb21d96d0139831ace39c799a079da`; the #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/native-tests-02/command.txt")[suite command] pins that copy. The public-example binary retained the same hash across the two builds.

The first-build fixture and controls were copied before correction into the #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/source-snapshots/build-01/SNAPSHOT.md")[source snapshot];, whose hashes match the initial inventory. The independent diagnostic and failed run remain unchanged. The fixed driver’s #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/driver-build-01/exit-status.txt")[separate compilation] is successful; no launch receipt exists. Compilation, execution and mathematical proof remain separate evidence categories.

The separate #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/root-library-01/command.txt")[root-library call] ended at its 180-second outer limit with exit 124. Before timeout, `aws-sdk-s3` compilation reported an encoded-metadata allocation failure \(`Cannot allocate memory`, OS error 12), after which Cargo waited for other jobs. No root test executable or test outcome was produced. All 1,629 inventoried source/configuration files passed the post-call hash check. This incomplete build leaves the full-library gate open; it does not negate or extend the scoped test observations.

The #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/ADMISSION.md")[admission inspection] identifies a concrete operational blocker. The attempted #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/evidence/admission-01/command.txt")[checkpoint gate] exited 1 because neither the configured AWS profile nor the checkpoint upload token was available. No authenticated AWS/EKS caller or submitted job is established. This is an unavailable configured route, not a solver outcome or an IAM rejection.

The inspected standard EKS workflow builds other maintained payloads and has no RR5 payload selection. A changed specification path cannot supply this executable. The supported continuation needs an authorized command route and digest-pinned image containing the tested revision, or reviewed workflow support, together with checkpoint, publication, capacity and spending admission.

== 7. Exact next gate and accounting boundary
<exact-next-gate-and-accounting-boundary>
The fixture correction, scoped regressions and driver compilation are complete. The next scientific gate is to satisfy the existing execution admission requirements, preserving the exact tested source, frozen targets and limits. The separate root-library and publication checks must retain their own outcomes.

The registered positive target is built grader-side from the first two canonical Dickson points and first two extras with signs $(+ , - , + , +)$. Only the public curve, base, target and limits enter the candidate. Run the structured presentation once, then the exact cofactor reference once. Run $H_0 = (18 , 221278090)$ and $W_0 = (101000006 , 1077058598)$ only if the structured positive completes with independent signed replay.

The positive gate requires `Complete`, the expected row, exact replay of every returned row, full original-equation checking, and zero forbidden field/domain branches. A cap or missing univariate fails completion. The public x-walk targets have no supplied scalar labels.

Setup validation, symbolic construction, elimination, root recovery, signed replay, authentic-RHS rank and full cold cost must remain separately reported. For a genuinely labelled query $R = [a] G + [b] T$, the row equation is $ sum_i epsilon_i log_G (P_i) - b thin d = a med \( mod med n \) . $ No B677 row, useful rank, recovered scalar, or complete-cost result is available. Canonical performance graphs and leaderboards remain unchanged.

== Sources and evidence standing
<sources-and-evidence-standing>
The self-contained proof and source discussion are in #link("https://github.com/isogeny-labs/crypto/blob/5e536ed93302def2122968aa880b22c4b675d516/research/prime31_four_summand_20261122/scaling-b677/MATHEMATICS.md")[MATHEMATICS.md];. Paterson–Stockmeyer’s #link("https://doi.org/10.1137/0202007")[1973 paper, DOI 10.1137/0202007] motivates the blocked evaluation upper construction; it supplies no finite-field extraction lower bound. Faugère–Mou’s #link("https://arxiv.org/abs/1304.1238")[Sparse FGLM algorithms, arXiv:1304.1238] presuppose an available finite quotient/multiplication action and do not solve the present front-end elimination.

Prior context remains #link("https://eprint.iacr.org/2004/031")[Semaev 2004/031];, #link("https://www.iacr.org/archive/eurocrypt2014/84410158/84410158.pdf")[Faugère et al., EUROCRYPT 2014];, and the linked RR5/RR9 repository notes. The protocol records unavailable KB tools and the named-source fallback. No exhaustive novelty review, performance advantage, signed independent verification, or Lean proof is claimed. \[pcc:unverified\]
