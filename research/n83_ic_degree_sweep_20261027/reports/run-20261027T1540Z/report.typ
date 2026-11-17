#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 9.5pt)
#set par(leading: 0.6em)
#set heading(numbering: none)
#let rows = json("../../evidence/observations.json")
#let certificate = json("../../evidence/prime-certificate.json")
#let completed = rows.filter(r => r.status == "complete").len()
#let finite = rows.filter(r => r.degree_of_regularity_top != none).len()
#let limits = rows.filter(r => r.diagnostic_reason == "time_limit").len()
#let memory = rows.filter(r => r.diagnostic_reason == "memory_limit").len()
#let pre = rows.filter(r => r.last_observed_phase == "input_expansion_or_witness").len()

#text(size: 23pt, weight: "bold")[N83 index-calculus degree sweep]

#text(fill: rgb("#44566c"))[Balanced candidate domains · m = 2–9 · 27 October 2026]

Observations only. This report records bounded Semaev-chain computations; it is not an evidence-review verdict or a complete ECDLP result.

#block(fill: rgb("#edf2f7"), inset: 11pt, radius: 5pt)[
  *#rows.len() attempts; #completed completed computations; #finite finite top-form degrees.*
  #limits attempts reached the process time limit and #memory reported exhaustion of the memory allowance. Missing degrees remain unknown.
]

#image("degree-grid.svg", width: 100%)

The grid is a completion diagram because no pair of completed degree measurements is available. P/N denotes one planted and one natural target in each cell. “Limit” means the fixed process time allowance; “memory limit” means Singular's explicit no-more-memory diagnostic. No timeout supplies a mathematical degree bound.

*Measured workload:* 3 curve families × 8 arities × 2 target kinds × 2 modes = 96 attempts. The modes compute different quantities and are not independent replicates. Eight small same-driver control computations passed. The native field, curve and construction tests passed (4 tests).

*Balanced size:* ell = floor(83/m), namely 41, 27, 20, 16, 13, 11, 10, 9. Candidate x-domain sizes are 2^ell. The actual factor base consists of points whose x-coordinate lifts to the selected curve; it was not exhaustively enumerated.

*Run:* `n83-degree-m2-m9-20261027-v2`. Source data: `evidence/results.json`, `observations.json` and the full hash manifest. Diagram source: `src/bin/report.rs`. No uncertainty interval is estimated from one target of each kind.

#pagebreak()
= Methods, limits and reproduction

*Top-form regularity:* Generate the ideal from each input polynomial's highest homogeneous component. For a zero-dimensional quotient, report one plus its last nonzero graded degree using an exact Hilbert series. This quantity depends on the presentation and is not the regularity of the saturated homogenized affine ideal.

*Affine computation:* Singular `std` in degree reverse lexicographic order, with input reduction and planted-witness checks. The final basis degree would be recorded only after completion. Maximum intermediate solving degree remains unmeasured. No first-fall-degree value is substituted.

*Process limits:* 40 seconds plus a 5-second kill grace, 3 GiB virtual memory per worker, four independent workers on an EKS Spot job requesting four CPUs and 16 GiB. These are whole-process limits, including input expansion and witness checks. #pre attempts ended before a `SOLVER_BEGIN` marker. Wall times in the raw records are diagnostic; no performance claim is made. CPU user/system time and peak RSS were not recorded.

*Curve identities:* Koblitz uses `y^2+xy=x^3+1` over GF(2^83), with modulus `z^83+z^45+z^2+z+1`. General binary uses `y^2+xy=x^3+x^2+b`, `b=2^82+0x51b`, in the same field. Prime uses `y^2=x^3+2x+3` with Proth-certified p = #certificate.p. Full-curve synthetic targets are used; no subgroup attack is claimed. Every relation candidate still needs lifting and sign verification.

*Domain representation:* Binary x-coordinates use the span of the first ell polynomial-basis vectors. Prime x-coordinates use a Boolean-selector encoding of the integer interval `[0,2^ell)`. The encodings differ; these rows do not isolate a causal effect of curve family. Root seed: 20261027, with the fixed family/arity derivation in the frozen source.

= Separate algebraic finding

For this prime presentation, the top ideal has dimension ceil((m−2)/2) for m = 3–9: 1, 1, 2, 2, 3, 3, 4. These are *derived dimensions*, not measured Singular results. Selector squares force all selectors to zero in the radical; the remaining relations are adjacent products of intermediate coordinates. The surviving components correspond to independent vertex sets of a path. Thus the protocol's finite top-form degree is undefined for these cells. This does not obstruct IC or determine affine solving degree. The full argument is in `ALGEBRA.md`.

= Evidence and validation

The frozen protocol predates target data. V1 failed at startup because the non-root worker could not read the runner; no target measurements ran. V2 corrected permissions and a per-arm sample-count field before data. Both protocols and the failure receipt are retained.

Exact commands, source revision, image digest and terminal pod identity are in `request-v2.json`, `submission-v2.json` and `terminal-v2.json`. The unified export is `experiment.export.json`. Raw storage and SHA-256 verification are recorded in `evidence/storage.json` and `raw-manifest.sha256`. No independent replay certificate or speedup is claimed.

Primary method references: #link("https://arxiv.org/html/1503.08001")[Kosters and Yeo, Notes on summation polynomials]; #link("https://www.singular.uni-kl.de/index.php/singular.pdf")[Singular manual]. The full model, source review and interpretation rules are in `PROTOCOL.md`.
