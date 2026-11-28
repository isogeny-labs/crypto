#set page(width: 21cm, height: 29.7cm, margin: 2cm)
#set text(size: 10pt, font: ("Liberation Sans", "DejaVu Sans", "Arial"))
#set par(justify: true, leading: 0.62em)
#show heading: set text(fill: rgb("#1a3a5a"))
#show raw: set text(font: ("Liberation Sans", "DejaVu Sans"), size: 9pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[Kaarelson v2 "A Fast Algorithm for ECDLP": full-paper assessment]
  #linebreak()
  #text(size: 9pt, fill: rgb("#555"))[2026-11-18 UTC · main base `9ec88b7637` · assessment PR \#4672 · extends the 2026-11-17 subexponential-ecdlp-goal lead (PR \#4627)]
]

#block(fill: rgb("#f3f3f3"), inset: 7pt, radius: 4pt, width: 100%)[
  *Scope.* A literature assessment of one external preprint. Every statement
  below about the paper is attributed to the paper; this note carries no
  independent verification and runs no experiment. `[pcc:unverified]`
]

*Source.* Kaarel Kaarelson, _"A Fast Algorithm for Elliptic-Curve Discrete
Logarithms"_, version 2, 2026-11-01 (`github.com/kaarelkaarelson/fast-ecdlp`).
The paper states it was produced by the author directing Claude agents
(coordinating model Claude Opus 5.5; sub-agents Opus, Sonnet, Haiku), main
results obtained 2026-10-30/31, and that its reviewers were AI agents in the
same environment rather than human referees.

#figure(image("verification-map.svg", width: 100%),
  caption: [Result dependencies and verification standing as stated by the paper.])

== The four results, and at what standing

- *Theorem 1 (main).* For every fixed m >= 2, a Las Vegas algorithm claimed to
  solve ECDLP on every curve over F_p, any group order, in expected time
  p^(2/m+o(1)); lower exponent than Pollard rho for m >= 5. The paper states its
  oracle is a Lean theorem (Thm 2(ii)) and that Thm 1 does *not* assume the
  Anthropic engine (E); the coverage bound, point counting, factoring, sparse
  linear algebra and Lemmas 3.8/3.9 are *paper proofs* checked by AI review and
  numerics, not by machine.
- *Theorem 2 (fast decomposition).* The x-string search (Thm 2(ii), Lean
  `L104_Final.subexponentialRate_all`) and its counting form (Thm 2(i),
  `L132_Final.countingRate_all`) are stated as Lean theorems with *no
  hypotheses, engine included*, axiom check `propext, Classical.choice,
  Quot.sound`. One deliberate `sorry` in `ECDLP.subexponentialRate`
  (`L35_Challenge`), stated as never used as a lemma.
- *Theorem 3 (coverage -- unconditional).* On a random isomorphic model E_u, a
  fraction >= c_m(kappa) of points decompose into [1,X], X = floor(p^(1/m)). The paper
  states this *uses no engine and no heuristic*: a Kummer-theory second-moment
  argument (Lemmas N', F, 5.4, 5.5, Prop 5.6). Worth reading regardless of the
  engine's fate.
- *Theorem 7 (growing m, conditional).* Letting m grow gives p^o(1),
  *conditional on (E)* (Anthropic engine statements) *and (N)*. The only
  discrete-log result that uses the engine; "says nothing at any size anyone
  could compute: m\* >= 2 already requires log2 L >= 6.25·10^4."

The two new ideas, in the paper's terms: averaging over isomorphic models
replaces the 1/m! decomposition heuristic by a second-moment bound (Thm 3), and
the counting engine makes the decomposition *search* cost p^o(1) because a
decomposition is witnessed by O(m) field elements (U = O(mL), not Theta(L^2)). The
paper argues (its §9) this refutes a prior conjecture that any succinct
sqrt(p)-size factor base needs p^(1/2−o(1)) per decomposition -- i.e. that interval
factor bases over F_p are not inherently useless.

== The constants: no instantiable curve is affected

#figure(image("crossover-thresholds.svg", width: 92%),
  caption: [Crossover thresholds from the paper's Table 5 against a real 256-bit
  key (values transcribed, not recomputed).])

The paper is explicit (§10) that nothing bears on P-256, secp256k1, Curve25519
or any instantiable curve. The one row that is a Lean theorem end-to-end (edge
network) places the crossover at log2 log2 p about 6.8·10^16; the lowest row buys
about 10^7 only under a hand-audited assumption (R1), a numeric network value and a
free schedule. So p must have far more than 2^(10^8) bits. Against the lab's
success criterion (measured progress over the IC baseline, reported as gap to
matched rho), this is *not a success-criterion row*: an asymptotic existence
result with galactic constants and no measured arm. The paper's own m=3 index
calculus loses to rho with the negation map at every size 10-40 bits --
consistent with the program's instantiable-size negatives.

== Provenance and review

The foundations are unrefereed and the checking was overwhelmingly done by AI,
which the paper states plainly. (E) is an unrefereed Anthropic preprint quoted
but not rebuilt; the Lean pipeline uses the Rudich-Rousseau Lean engine [24],
also unrefereed. The reviewers were AI agents in the same environment, which the
paper names as its main risk. Review did catch real errors before this version
(a 2^m-vs-4^m constant exposed by the Lean formalization; a vacuous density
statement; an unsatisfiable Lean hypothesis; a wrong Lemma N'). Reproducibility
is partial: several computations cannot be checked from the repository.

== What the program can take from this

+ *Coverage theorem (Thm 3) -- independent re-derivation and measurement.*
  Engine-free; lands on the lab goal. The paper reports mean coverage 0.39-0.42
  at p in {1009, 4099, 10007}, m=2, and E_2/X^2 about 4; the lab's native coverage
  harness can reproduce these without touching the engine.
+ *The plain box u=1* (paper open problem 1): whether the undilated factor base
  already has coverage bounded below. The lab's interval-factor-base work is the
  place to measure it at the lab's own primes.
+ *Independent audit of the Lean engine claim* (Thm 2(ii)
  `subexponentialRate_all`, the disclosed `sorry`, the Rudich-Rousseau import) --
  a `lean-instance-gate` / independent-validator target.
+ *Fold the honest conclusion into the lead atlas*: a galactic existence result
  with no bearing on deployed curves, whose m=3 negative corroborates the
  program's own instantiable-size negatives.

*Next technical step.* Re-derive and measure Theorem 3 coverage at
p in {1009, 4099, 10007}, m=2, with the lab's native coverage harness, as an
engine-free check of the one claim the program can use; compare against the
paper's 0.39-0.42 and about 4. The Lean engine `subexponentialRate_all` and its
Rudich-Rousseau import stay an open audit obligation, not relied on here.
