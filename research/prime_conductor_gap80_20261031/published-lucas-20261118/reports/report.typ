#set page(paper: "a4", margin: 16mm, footer: context align(right)[#counter(page).display()])
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: 0.5em)
#let s = json("../summary.json")
= A larger fixed Frobenius conductor
18 November 2026 · ordinary binary curve · draft PR 4723

The fixed published index *n=7351* yields a *#(s.conductor_bits)-bit*
Frobenius conductor for E: y²+xy=x³+1 over GF(2⁷³⁵¹).
It has #s.decimal_digits decimal digits. Complete prime certificate:
*#s.certificate_generated*. Fresh-process certificate replay:
*#s.certificate_replayed*. This is conductor arithmetic, with no evaluated
isogeny or endpoint timing result. [pcc:unverified]

#image("sizes.svg", width: 100%)

== What the two conductors mean
The same source curve has geometric endomorphism conductor *1*. Its
Frobenius conductor after extension is the large integer |U₇₃₅₁|. These
are different orders: Z[π⁷³⁵¹] lies inside the maximal order End(E)=O(K). The conductor of
an unconstructed lower endpoint is not obtained by relabelling this source.

#block(stroke: 0.7pt + rgb("#40749a"), inset: 10pt, width: 100%)[
  *Known source:* End(E)=O(K), D(K)=−7, f(E)=1. \
  *Reconstructed order:* Z[π⁷³⁵¹], f(π)=|U₇₃₅₁|. \
  *Missing bridge:* explicit lower curve, conductor evidence, kernel and map. \
  *Missing measurements:* transported points and paired endpoint sessions.
]

== Requirement status
#table(columns: (1.05fr, 1.6fr), inset: 5pt, stroke: 0.4pt,
  [*Requested object*], [*Current evidence*],
  [Prime conductor ratio ≥80 bits], [Original 80-bit target unchanged; larger fixed conductor arithmetic checked separately],
  [Explicit isogeny and non-kernel images], [Absent for both large cases],
  [Dozens of scalars in several fields], [Existing small-degree 202/96-bit controls retained; no large crossing measured],
  [Scalar, ECDLP and IC timings], [No new timing; wide-field IC unavailable],
  [GitHub and S3], [Draft PR published; configured store remains inaccessible],
)

The complete decimal integer is in attempt-a/arithmetic.json and
conductor.txt. The primary source's later probable-prime entries are not
promoted here. No world-record claim is made. No elapsed certificate time
is an ECDLP measurement.

#pagebreak()
= Derivation and reproducible checks
The base model over F₂ has four points: three affine points and infinity.
Therefore t₁=−1, and π²+π+2=0. Its discriminant −7 is fundamental.
The trace is odd, so E is ordinary. The inclusion
Z[π] ⊆ End(E) ⊆ O(K) forces equality already over F₂; the ordinary
geometric order is maximal and finite base extension preserves it.

Writing πⁿ=aₙ+bₙπ gives tₙ=2aₙ−bₙ. Since
π−π̄=√−7, the exact identity is

  tₙ² − 4·2ⁿ = −7 bₙ²,   f(π) = |bₙ|.

The fixed integer is also Uₙ, with U₀=0, U₁=1 and
Uₙ=−Uₙ₋₁−2Uₙ₋₂. PARI computes both recurrence and matrix power.
The separate C++/GMP oracle uses binary powering in Z[π], with
(a,b)(c,d)=(ac−2bd,ad+bc−bd), and compares the full integers.
Small n=4,7 and the original K-571 value are exact controls.

== Evidence and certificate gates
The frozen source/binary manifest is pre-check.sha256. The fixed attempt
has native arithmetic, stdout, stderr and exit receipts. GP may return
zero after an interpreter error, so the qualification also checks empty
stderr, required output fields and the complete certificate. Fresh-process
replay recomputes the subject and rejects a changed subject.

Certificate-attempt exit: *#s.certificate_attempt_exit*.
Certificate stderr empty: *#s.certificate_stderr_empty*.
Changed-subject rejection: *#s.changed_subject_reject*.
Prime status: *#s.prime_status*. Claim: *PCC-20261118-a08131*.
All producer statements remain unsigned, standing unverified and
verdict `not_covered`. A PARI ECPP certificate is a mathematical prime
certificate, not an independent signed execution receipt. [pcc:unverified]

Local development limits were one fixed example, one GP thread,
512 MiB stack, 300-second timeout, no paid compute and no automatic retry.
This was not a parameter search or performance benchmark. Lean/lake is
not exposed; the readable proof and arithmetic checks are not a compiled
Lean theorem. Construction and all endpoint costs stay unknown.

== Reproduction and sources
Use a fresh directory: GP write appends. Run check-fixed.gp with
PARI/GP 2.17.2, then replay.gp from that directory. Compile ring-oracle.cpp
with C++17 and GMP; compare exact trace/conductor strings. README.md
contains the command and receipt contract.

Sources: OEIS A001607 and A101087; sequence author Gérard P. Michon's
Numericana article; Koblitz–Koblitz–Menezes §11.2 Example 4;
Sutherland 18.783 Lecture 13, Theorems 13.3/13.8 and Corollary 13.20;
PARI primecert/primecertisvalid documentation. Full links and source
digests are in README.md and sources.json. KB unavailable, wrapper exit127.
