#set page(paper: "a4", margin: 2cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")
#set par(justify: true)

= Twist points cannot supply index-calculus relations: a measured validation

Study `research/twist_obstruction_20261026/`, 2026-10-26.

- Run: `reports/run-20261026T1456Z/`. The run report is `RUN_REPORT.pdf`.
- Protocol: `PROTOCOL.md`, frozen at commit `ff3bf9d48` before the run.
- Harness: `src/main.rs`, native Rust with std only.
- Aggregation: `summarize/`, native Rust.
- Plotting: `plot.py`, layout only.

Each statement below is tagged *theorem*, *exact computation*, *heuristic
reference* or *not established*.

== Question

Take E and its quadratic twist E′. They share x-coordinates, so they share
summation polynomials. Can the twist's points feed relations for the DLP on
E? The argument says no. φ(E′(F\_q)) is the σ = −1 part of the x-rational
points of E(F\_(q²)), and E(F\_q) is the σ = +1 part. So R = A + B forces
2B = 0.

This study makes three consequences of the argument concrete and measures
them in the ECC2K-130 model: y² + xy = x³ + ax² + b over F\_(2^n) with n
odd. The twist is a ↦ a+1, and φ(x, y) = (x, y + sx) with s² + s = 1.

== Statements and their status

+ *T1 (theorem).* Let P ∈ E(F\_q) and Q ∈ φ(E′(F\_q)). Then P + Q ∈ E(F\_q)
  iff Q ∈ E[2](F\_q) = {O, T}, where T = (0, √b).
  - *Exact computation:* exhaustive at n = 7, 11, 13 on 15 curves.
  - 356,530,036 sums with Q not 2-torsion; *0* are rational.
  - 103,884 sums with Q ∈ {O, T}; all are rational.

+ *T2 (theorem).* Let R = Q₁ + … + Q\_m with every x(Qᵢ) ∈ F\_q. Then the
  sub-sum of the Qᵢ whose σ-parity is opposite to R's lies in {O, T}.
  Consequences:
  - For m = 2 there is no mixed solution.
  - For m = 3 the mixed solutions are exactly (x\_R, x, x) when x\_R ∈ V,
    and (x(R+T), x, x(P\_x+T)) when x(R+T) ∈ V.
  - Every F\_q-root of S\_(m+1) is therefore a valid relation on E once the
    twist sub-sum is dropped. No root is lost.

  *Exact computation:* 4954 decomposition targets, n = 11 to 19, factor
  base V = span(1, …, t^(l−1)).
  - 0 violations on both models.
  - On 1154 of 1154 m = 3 targets, the observed off-parity set equals the
    set predicted from the target and V alone.
  - The positive controls are detected:
    - C1: 18,042 planted (x\_R, x, x) solutions, all found;
    - C2: 95 planted T-pair solutions, all found.

+ *T3 (theorem).* The x-solution set is the same on E\_a and E\_(a+1). For
  binary curves, S₃ = x₁²x₂² + x₁²x₃² + x₂²x₃² + x₁x₂x₃ + b does not involve
  a at all.
  - *Exact computation:* the x-sets are identical on 4954 of 4954 targets.
  - The point sets equal the explicit S₃ roots on 3800 of 3800 m = 2
    targets, and the explicit S₄ = Res(S₃, S₃) roots on 105 of 105 m = 3
    targets.

+ *Heuristic reference, reported as a distance.* Suppose lift types were
  independent of the solution condition. Then a fraction 1 − p\_E^m of
  solutions would contain a twist-lifting coordinate: 0.74–0.77 for m = 2
  and 0.86–0.89 for m = 3. The observed fraction is:
  - m = 2: 0, with every 95 % upper bound ≤ 0.0016;
  - m = 3: 0.041–0.063 at n = 11, 13, and 0 at n = 17, 19.

  All nonzero m = 3 mass is the predicted degenerate family (x\_R, x, x),
  hit because x\_R happens to fall in V (|V|/q = 3.1 % and 1.6 %). Those
  "mixed" solutions carry no new information: the relation is R = R.

#figure(image("reports/run-20261026T1456Z/fig1_off_parity_fraction.svg", width: 100%),
  caption: [Arm A, run-20261026T1456Z: observed off-parity fraction (95 % CP) against the independent-lift heuristic.])
#figure(image("reports/run-20261026T1456Z/fig2_predicted_vs_observed.svg", width: 60%),
  caption: [Predicted against observed off-parity solutions per target (m = 3). Every point lies on the diagonal.])

== Interpretation

The separation holds in this model. The measured behaviour matches the
argument exactly, including its degenerate cases. Three practical
consequences for the index-calculus programme:

- *Twist points are not a source of relations.* Adding φ(E′(F\_q)) to the
  factor base adds columns that no non-degenerate relation on E can use.
  The roughly half of x ∈ V that lift to the twist contribute nothing, and
  the factor base loses nothing by excluding them. The same picture
  appeared on prime fields in
  `research/pair46_factor_bases_20261015/05-qr-window/`: the twist-aware
  filter changed the candidate counts and added no relations.
- *Rationality checking is lossless.* Every F\_q-root tuple of S\_(m+1) is a
  valid E-relation after the twist sub-sum is dropped, and for m = 2 every
  root tuple is pure. No y-recovery or lift-checking step in a relation
  pipeline discards a usable root.
- *A correctness invariant for decomposition code.* Any pipeline that
  reports an off-parity solution outside the predicted structured set has a
  bug. This study's instrument provides that check, with positive controls.

What could escape the obstruction (*not established*, no candidate known):
- Any construction built from group operations defined over F\_q commutes
  with σ, and once 2 is inverted no nonzero equivariant map sends the −1
  part to the +1 part.
- The ℓ-parts of E and E′ can coincide only when ℓ divides 2t. For
  ℓ > 2√q that forces t = 0, a supersingular curve, which MOV already
  reduces.
- The shared geometry does allow *amortisation*: one descent or Gröbner
  set-up serves both the DLP on E and the DLP on E′. That is an accounting
  change, not an improvement to a single ECDLP (CLAUDE.md success
  criterion).

== Scope and limits

- The computation covers n ∈ {7, 11, 13} (census) and n ∈ {11, 13, 17, 19}
  (decompositions) with m ≤ 3 and the low-degree subspace V. The general
  statements rest on the proof. The computation shows the proof's
  predictions hold on every instance enumerated and that the instrument can
  see off-parity solutions when they are planted.
- The violation counter was exercised only by real data, never by a
  planted violation. It is the `else` branch of the same classifier that
  the C1/C2 off\_O/off\_T branches exercise.
- No ECDLP cost was measured, and no IC leaderboard row, benchmark row or
  rho comparison is made or implied. Wall times are host-dependent and
  recorded only for provenance.
- No `identity.certificate/v1` is issued. The identities here are
  group-theoretic, over F\_(2^n). The v1 checker evaluates over F\_(2^61−1)
  and cannot express them. Independence of S₃ from a is syntactic.
