// Render with ./render.sh (Typst 0.15.1 and Ghostscript 10.05.1).
#let ink = rgb("182f45")
#let muted = rgb("53687b")
#let pale = rgb("edf3f7")
#let repo = "https://github.com/isogeny-labs/crypto"
#let source = repo + "/blob/059f1a493b5c68d20b4023b1c00329593bca8510/"
#let study = source + "research/koblitz_halftrace_20261030/"
#let recheck = study + "development/recheck-20261104T1548Z/"
#let closure = recheck + "closure-20261104T1707Z/"
#set page(paper: "a4", margin: (top: 17mm, bottom: 17mm, x: 17mm), footer: context align(right, text(size: 8pt, fill: muted)[Native Rust IC · 6 November 2026 · #counter(page).display("1")]))
#set text(font: "DejaVu Sans", size: 9.7pt, fill: ink)
#set par(leading: 0.55em, spacing: 0.55em)
#set heading(numbering: none)
#show heading.where(level: 1): set text(size: 23pt, weight: "bold", fill: ink)
#show heading.where(level: 2): set text(size: 12.5pt, weight: "bold", fill: ink)
#set table(inset: 6pt, stroke: 0.5pt + rgb("cdd8e1"), align: left, fill: (x, y) => if y == 0 { pale } else { none })

= Native Rust index calculus
#text(size: 14pt, fill: muted)[Factor-base halftrace cache · implementation and measurement report]

6 November 2026 · #link(repo + "/pull/2955")[PR \#2955] · Evidence reviewed at 23:23 UTC

#block(fill: rgb("dff0eb"), inset: 10pt, radius: 5pt)[
  *Implemented in Rust; targeted native tests pass.* The opt-in cache is integrated into the existing factor-base pipeline. The latest source-bound results are 12 cache-filter passes, one ordered-lift pass and two E2E passes. Broader full-library validation remains incomplete.
]

#block(fill: rgb("fff1dd"), inset: 10pt, radius: 5pt)[
  *Performance remains unmeasured.* No scientific execution has started. A speedup ratio, end-to-end cost change and cold cache payback cannot yet be reported.
]

== What changed
The point-lift stage repeatedly solves the binary equation `u² + u = x + a + b/x²` for nonzero `x`, then forms `y = x*u`. The implementation replaces repeated halftrace squarings with lazy byte lookup tables for repeated public inputs. The zero-coordinate case retains its existing path. The first returned root and the order of the point and its negation are preserved.

#image("pipeline.svg", width: 100%)

The factor-base recipe `koblitz-orbit:divisor=1;2,halftrace_tables=1` selects the candidate. Recipes without the flag retain the legacy lift. The cache is local to the curve/field instance, initialized once and shared by initialized clones. A degree/modulus guard prevents reuse after public field replacement. The synchronous selector restores its prior value on nesting and unwinding; threads have independent selections.

== Why this component was selected
Repeated lifting is a concrete cost inside factor-base construction. At degree 31, the declared table layout contains four arrays of 256 `u64` entries: *8 KiB of table payload*, derived from the source layout, excluding allocation and object overhead. Evaluation combines up to four entries plus a trace-admission check. Whether this saves enough work to repay construction is the hypothesis to measure; the cache's cold setup must be charged.

Existing Frobenius folding, joins, relation collection and linear algebra are reused. This change is an engineering candidate in the *generic decomposition* track. It adds no new decomposition oracle. The optimized `FastCurve` path is capped at field degree 62; the cache handles odd degrees, with tests through degree 61.

#text(size: 8.4pt, fill: muted)[Implementation sources: #link(source + "src/cryptanalysis/koblitz_fast.rs")[koblitz_fast.rs], #link(source + "src/cryptanalysis/koblitz_index_calculus.rs")[koblitz_index_calculus.rs], #link(source + "src/cryptanalysis/ic_framework/plugins.rs")[plugins.rs], and #link(source + "examples/koblitz_halftrace.rs")[native E2E example]. Binary point recovery follows #link("https://www.secg.org/sec1-v2.pdf")[SEC 1 v2.0 §2.3.4]. The repository's general trace/halftrace code supplies the reference behavior.]

#pagebreak()
= Validation and evidence

The table separates executable/source scopes. A historical pass is not substituted for a later merged-source result. The latest native source is `507829726fbc731543f4527348b55c5e479242c9`; subsequent published commits through `059f1a493` change workflow support or evidence, with native source hashes retained.

#table(columns: (1.05fr, 0.8fr, 1.8fr),
  table.header([*Check*], [*Observed result*], [*Scope and evidence*]),
  [Original release library], [5,447 passed\ 0 failed\ 123 ignored], [Historical feature executable, before two equivalent parity-expression lint cleanups. #link(study + "development/full-library-result.txt")[Parent result].],
  [Latest `cached_` filter], [12 passed\ 0 failed], [Native source `507829726f`; includes cache, scope and factor-base parity checks. #link(recheck + "filtered-receipt.json")[Receipt].],
  [Ordered point lift], [1 passed\ 0 failed], [Same native source; exact roots and ordering compared. #link(recheck + "ordered.txt")[Log].],
  [Native E2E example], [2 passed\ 0 failed], [Complete harness summary, including public ecbench recovery. Original exec handle unavailable; separately collected Cargo exit status is unknown. #link(closure + "e2e.raw.txt")[Raw log].],
  [Later full release suite], [Incomplete], [5,937 tests started; no parent summary or exit receipt. Two failures were observed. #link(closure + "outcome.json")[Outcome].],
)

== What the correctness checks cover
Halftrace roots are checked against the general field definition: exhaustive inputs for degrees 5, 7, 9 and 11, and sampled inputs for 19, 31, 53 and 61. Checks cover field separation, initialized-cache sharing, changed-field handling and selector restoration. Factor-base checks compare ordered points, columns and coefficients. E2E checks recover known scalars through the existing pipeline and check their point equations. Original final-source strict library Clippy and formatting results are retained separately from later rechecks.

== Why the full-suite result remains incomplete
Two resource-publication tests were reproduced in isolation: that module reports two passes and two failures. Its diagnostic clock requires `/proc/self/ns/time` and `/proc/self/timens_offsets`, absent on the development host. The inspected diagnostic source matched upstream main. No tests were disabled and no unrelated clock fallback was added.

The full-suite processes later became defunct without a parent summary or exit receipt. *The interruption cause is unknown.* The one-pass summary in the partial output has 5,936 filtered tests and belongs to a nested child; it is not a result for the 5,937-test parent. The complete partial log is preserved in a deterministic 54,340-byte archive, with its SHA-256 and an extracted-log digest. #link(closure + "README.md")[Archive and reproduction instructions].

== PR state at the report snapshot
PR \#2955 is open and conflicting; auto-merge is paused. The P1 review still requires the saved native baseline/candidate comparison. On head `059f1a493`, two checks are recorded and none is `gate`, `size` or `artifacts`: *current-head merge admission is absent*. A previous artifacts-wiring failure was addressed by giving the metadata-only checkpoint checkout a dedicated path and working directory. Its focused generator check and actionlint passed; this does not replace required CI.

#text(size: 8.4pt, fill: muted)[Evidence is source-bound development output and repository receipts. Test elapsed times are not benchmark results. No aggregate pass count is inferred for the incomplete parent suite.]

#pagebreak()
= Measurement plan and next steps

== The frozen comparison
#table(columns: (1fr, 2.5fr),
  table.header([*Item*], [*Prospective protocol*]),
  [Instance], [Existing `K_0` constructor over degree 31; eight independent one-target workloads, not one eight-target batch. First four primary, last four untouched holdouts.],
  [Five arms], [Strong signed-Frobenius rho; rho A/A; native IC legacy lift; IC A/A; native IC cached lift. Same IC base policy, oracle and checked linear algebra.],
  [Executions], [Six alternating measured rounds plus one warmup: 240 measured + 40 warmup = 280 planned executions. *Zero attempted; zero completed.*],
  [Resources], [Intended EKS Spot budget: 900 seconds including provisioning, four CPUs, 4 GiB. Per-execution timeout in the spec: 120 seconds. Zero automatic measurement retries.],
  [Admission], [L2, IC/rho A/A controls, exact factor-base parity, independently replayed scalars, retained failures, complete cold cost and target-dependent online phases; unpriced-operation band at 0, 0.1 and 1 GAE.],
)

The IC legacy arm uses the preserved algorithm in the candidate executable. This flag comparison does not establish default-path regression against an unmodified-source image. That source-pinned image comparison remains separate. Coarse Artin–Schreier solve counts alone do not price the cache delta.

== What blocks execution
Checkpoint-only Actions #link(repo + "/actions/runs/41082358257")[run 41082358257] failed with `Approved CHECKPOINT_UPLOAD_TOKEN secret is not configured.` Contracts, build and experiment jobs were skipped. No S3 resume checkpoint or scientific job resulted. The current managed environment has no ready lab identity or secret, and no approved connection answer has arrived. The failed preparation has not been retried without a changed cause.

== Required sequence before landing
1. Restore an existing approved checkpoint/lab connection and complete a fresh source-pinned checkpoint. Preserve its receipt and the original run budget.
2. Build the scientific image from the frozen source; run the unchanged protocol on the required EKS/L2 environment. Retain all outcomes, identities and the canonical export; independently replay scalars.
3. Report cold and online costs with A/A admission and the prescribed accounting band. Keep losses, timeouts and missing completions visible; do not infer totals from a single phase.
4. Complete full-library validation on a supported host, resolve P1 with actual comparison evidence, and satisfy `gate`, `size` and `artifacts` on the final head before merging.

== Source identities and scope
#set text(size: 8.4pt)
Saved unmodified baseline: `3b34d504bc13ce668e86d8e68cdf57e3fbaba89a`\
Scientific executable pin: `7c58661935b79153ff74c1208d0592aa56ed04a1`\
Latest native development source: `507829726fbc731543f4527348b55c5e479242c9`\
Evidence/PR head inspected: `059f1a493b5c68d20b4023b1c00329593bca8510`

#link(study + "PROTOCOL.md")[Frozen protocol] · #link(study + "spec.json")[Native spec] · #link(study + "protocol.json")[Machine protocol] · #link(study + "development/checkpoint-actions-receipt.json")[Checkpoint failure] · #link(source + "problems/native-koblitz-measurement-access.md")[Open access problem]. Full hashes and report snapshot: `SNAPSHOT.json` beside this report.

This degree-31 study is exploratory. Its Frobenius-module structure differs from degree 83 and the degree-131 challenge; no degree-83 measurement or ECC2K-130 transfer is established. External arithmetic comparisons are unrun. No new mathematical theorem or performance value is claimed here. The scoreboard, progress timeline, leaderboard and existing study diagram are left unchanged because this report adds no measurement or curve finding.
