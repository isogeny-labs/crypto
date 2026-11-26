#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 10pt)
= Balanced complete isogeny evaluator
18 November 2026 · PR 4680 · development correctness checks.

The existing native complete evaluator now builds balanced baby-root products,
reuses them across giant blocks, and generates kernel progressions by addition
after block starts. A guarded supplied-case interface exposes complete quotient
and point evaluation. Coverage and collision checks remain before output.

The producer's before/after fixture bytes match each other and the frozen
receipt. Supplied-case checks compare 84 point evaluations against direct PARI
over three prime fields, including two block schedules on the larger control.
PCC-20261118-8b4a2c is a statement record with standing not_covered; these are
not signed independent findings. [pcc:unverified]

#table(
  columns: (1fr, 1fr, 1fr, 1fr), inset: 5pt,
  [Field p], [Degree], [Block], [Input records],
  [6666679], [10009], [17], [36],
  [6666679], [10009], [64], [36],
  [71], [19], [1], [6],
  [11], [5], [2], [6],
)

Counts include repeated bases, infinity and kernel points. The 84 evaluations
are not distinct non-kernel points. The original controls retain 42 map inputs,
34 paired known scalars, 68 endpoint recoveries and six coverage rejection
controls. Eight additional bad-input/window-limit controls return exit 2 with
empty stdout and the expected error. [pcc:unverified]

#image("traversal.svg", width: 100%)

The graph compares two schedules of the new implementation. It is not a
baseline/candidate timing comparison. Stage counters exclude fixed step setup,
kernel validation, direct reference work and arithmetic inside the norm helper.
No wall times, memory peaks, uncertainty intervals or runtime advantage are reported.

#pagebreak()
== Supplied-case interface and checks
Use #raw("--evaluate MAX_WINDOW BLOCK [--pari-reference]") with the whitespace
header #raw("p a b ell Kx Ky point_count"), followed by exactly that many
points as #raw("x y") or #raw("O") for infinity. The source is the short
Weierstrass model. At most 64 input records and degree at most 10009 for the
optional direct reference are accepted. MAX_WINDOW bounds both full window
sides before allocation; it is not an estimate of peak memory.

The native path checks prime field/degree, nonsingularity, canonical coordinates,
source membership, the prime-order kernel witness and complete coverage.
Every supplied non-kernel image is checked on the resulting codomain. Optional
PARI comparison checks the quotient coefficients and all images. The normalized
map formulas, log-norm backend and coverage contract are unchanged.

The new six-point degree-5 control covers the zero-leftover branch. Eight
negative cases cover off-curve points, wrong kernel order, composite degree,
trailing data, noncanonical coordinates, point-count limit, a too-small control
window and the unchanged target80 under a 1024-side limit. The latter is a
declared-limit rejection, not a failed full construction.

== Requirements still open
The original minimum remains an 80-bit prime endomorphism-conductor gap crossed
by a completely evaluated ordinary-curve isogeny, with matched endpoint scalar
and ECDLP measurements. None of the development controls is a conductor-gap
record. Generic output leaves ordinary classification, conductor gap and runtime
measurements null. No index-calculus arm is added.

The unchanged target needs 427154957034 baby roots. This component does not
establish full-window work/memory feasibility, durable external-memory resume,
or endpoint timings. The frozen measurement protocol still applies.

No cloud trial was submitted. The reviewed GitHub worker does not bundle this
FLINT/PARI evaluator. No EKS tools, AWS CLI, kubectl or configured AWS identity
are present here, and live remaining budget/reserved spend/capacity are not
established. COST_CONTROLS.md requires those checks before a paid build or job.

== Replay, source review and provenance
Build: GCC 14.2.0, FLINT 3.1.3, PARI 2.17.2, MPFR headers 4.2.2, system GMP.
The complete commands, fixed inputs and raw positive/negative responses are in
../README.md and ../validation/. The post-call manifest records actual source,
binary and package hashes; it is not a pre-call checkpoint. Build stderr and
all positive replay stderr files are empty. [pcc:unverified]

Sources: existing constructor derivation and complete-pipeline contract;
#link("https://flintlib.org/doc/fmpz_mod_poly/index.html")[FLINT reference] and
the installed 3.1.3 root-product declaration;
#link("https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/hom_velusqrt.html")[Sage square-root Velu reference].
The fast product/resultant approach is established prior work. Native execution
remains C/C++. KB unavailable: no exposed MCP service, wrapper exit 127. The
project Velu-sqrt site timed out; official Sage documentation was accessible.
Existing target-coverage and benchmark graphs have unchanged evidence. No
benchmark or leaderboard row is added; no novelty or record claim is made.
