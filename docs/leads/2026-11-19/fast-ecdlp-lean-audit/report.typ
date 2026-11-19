#set page(width: 21cm, height: 29.7cm, margin: 2cm)
#set text(size: 10pt, font: ("Liberation Sans", "DejaVu Sans"))
#set par(justify: true, leading: 0.62em)
#show raw: set text(font: ("Liberation Sans", "DejaVu Sans"), size: 8.5pt)
#show heading: set text(fill: rgb("#1a3a5a"))

#align(center)[
  #text(size: 14pt, weight: "bold")[Kaarelson v2: Lean statement-fidelity audit of the decomposition oracle]
  #linebreak()
  #text(size: 9pt, fill: rgb("#555"))[2026-11-19 UTC - kaarelkaarelson/fast-ecdlp commit d1deb5ab - follows PR #4672, #4745]
]

#block(fill: rgb("#f3f3f3"), inset: 7pt, radius: 4pt, width: 100%)[
  Statement-level audit of the Lean theorems behind Theorem 1, read from source
  at the pinned commit. Checks statement fidelity and the sorry/axiom mechanism;
  does NOT run the build, so it does not confirm the proofs or the axiom prints
  (those need a ~12 min, ~10 GB Mathlib build). `[pcc:unverified]`
]

#figure(image("statement-map.svg", width: 100%),
  caption: [The audit chain: Challenge (sorry) -> Solution (proof) -> Verify (type_of%, axioms)
  -> Comparator; resting on the vendored Rudich-Rousseau engine + Mathlib.])

*Finding 1 - the search statement is faithful and non-vacuous.*
`ECDLP.subexponentialRate m` asserts there is `M : TM2ComputableInTime enc encOut
(lexLeast m)` with finite stack alphabets and `forall k>=1, exists C, forall l,
M.time l <= C*2^(l/k)`. Mathlib's computability type binds M to compute the real
function `lexLeast m` (the generic-decomposition search); the time form is exactly
`2^o(l) = p^o(1)`. Curve membership is type-enforced (`P i : W.toAffine.Point`),
`p` prime and `W.Delta != 0` are gated, and invalid inputs give `none`. No vacuity.
At the statement level this is a correct formalization of Theorem 2(ii).

*Items to verify before relying on the full claim.*
- (2) The Lean factor base is `x < ceil(p^(1/m))` (half-open, includes 0), not the
  paper's `[1, floor(p^(1/m))]`; same size, different set (also differs from the
  coverage reproduction's interval).
- (3) The genericity condition (partial sum nonzero, x distinct from the next
  summand) should be confirmed equal to Definition 3.1. The authors' own in-kernel
  check (L105, p=13) matching brute force is reassuring.
- (4) Challenge<->Solution identity rests on the Comparator (`comparator.json` + the
  `nanoda` second kernel), not `Verify.lean`'s `type_of%` example alone, which is
  sound only if the audited name resolves to the sorry challenge (it imports an
  aggregate `ECDLPArtifact`).
- (5) `Verify.lean` `#print axioms` over the full set expects `[propext,
  Classical.choice, Quot.sound]`; confirming the output needs the build.

*Engine.* The Rudich-Rousseau counting engine is vendored at
`lean/vendor/rudich-factoring/`; the whole search theorem rests on it (unrefereed)
plus Mathlib. Independent review of it is the deepest open obligation.

*Next step.* In an environment with Lean/Mathlib v4.35.0 and ~10 GB disk: run
`lake build` + `lake env lean Verify.lean` (one sorry, clean axioms), run the
Comparator for statement identity, confirm genericity = Definition 3.1, and review
the vendored engine. This runtime was not used for the build; the statement-level
audit stands on its own.
