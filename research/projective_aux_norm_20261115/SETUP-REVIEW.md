# Source review for generated auxiliary setup

15 November 2026. This is the next setup candidate, recorded after cold006
and before any implementation or native call for it. The existing recovery
still uses its exact preset A,T,G. This review supplies no new algorithm,
finite search result, cost measurement or novelty claim. [pcc:unverified]

The candidate is to sample a short-Weierstrass curve with one rational
nonzero 2-torsion point, repeatedly find rational halves, check the final exact
2-power order by doubling, then use the existing Vélu tower and exact trace
collision test. Fixed-degree root finding could replace a full point count
in this setup. Successful halves must be checked as points and by doubling;
an x-coordinate root alone is insufficient. [pcc:unverified]

Sources actually opened:

- The existing [auxiliary extension](../prime_one_shot_20261029/continuation/theory/auxiliary-curve-extension.md),
  sections 1–2, and its [primary modular-curve review](../prime_one_shot_20261029/continuation/reviews/modular-curve-counting.md),
  sections 3–4. The model-count bound applies only for p>3, d=2^t>=8,
  p>=256d^4. It counts actual rational exact-order-d points via fine moduli.
  Its existing setup uses a polynomial-time point count; replacing that
  step requires its own sampling and total-work analysis.
- Andrew V. Sutherland, MIT 18.783 Fall 2023,
  [Lecture 2](https://math.mit.edu/classes/18.783/2023/LectureNotes2.pdf),
  section 2.2, affine addition and doubling formulas. These give a route to
  the fixed-degree preimage equations. The notes explain the y=0 exception
  and the need to interpret the formulas on the actual elliptic curve.
- Sutherland, [Lecture 3](https://math.mit.edu/classes/18.783/2023/LectureNotes3.pdf),
  sections 3.11–3.11.2, Algorithm 3.45 and its expected root-finding cost.
  It first intersects with X^q-X, then splits by random shifts; the all-roots
  extension recurses on both factors. At fixed degree its expected bit cost
  is polynomial in log q. This is a library algorithm fact, not a measured
  bound for a new implementation.
- The current [FLINT nmod_poly API](https://flintlib.org/doc/nmod_poly.html)
  was opened as an interface reference. The current native coefficient
  implementation already calls the existing finite-field roots interface;
  no new prime-width support is supplied by this review.

The next preregistration must fix and account for the curve/2-torsion
sampling law, every halving choice and failed path, a search cap, trace-good
coset selection and actual main-base admission. In particular, the uniform
coefficient-pair probability cannot simply be copied to a sampler biased
by rational 2-torsion without counting the corresponding pairs. Every
successful setup needs independent exact-order/fibre replay. A tiny-field
panel cannot validate the large-parameter probability bound. The relation
oracle's sufficient degree and useful yield remain separate obligations.
[pcc:unverified]

KB unavailable: no loaded research-kb tools; `scripts/research-kb status`
returns 127 because no executable is installed. The review uses the precise
repository and official-course/API sources above; no exhaustive prior-art
or absence claim is made. Retrieved PDF hashes are in
`checks/setup-literature.sha256`; the report's source discussion includes
this review. No generated-setup source code or scientific run has begun.
