# Gaussian source-step counterexample and actual support estimates

Lean directly refutes the Gaussian-to-geometric equality printed in
Hildebrand’s proof at allowed k=3. For every a>0 the actual convergent sum
`Σ (m,n):Nat×Nat, exp(-a*(m²+n²))` is **strictly smaller** than
`1/(1-exp(-a))²`. The coefficient `1.1*epsilon/(2*p)` and the source’s real-power
exponent are retained in the direct negation of the affine displayed equality.
Replacing equality by `≤` repairs the step. This does not refute the qualified
mixing theorem. [pcc:unverified]

The proof first establishes actual summability and double-series factorization.
The inequality is strict because `n²≥n` for natural n, with strict inequality
at n=2. The positive-index Gaussian tail is bounded by its integral,
`sqrt(pi/a)/2`; summing the two integer tails gives the actual zero-excluded
integer Gaussian bound `sqrt(pi/a)`, and a bound for every finite integer set
avoiding0. Positivity of a is essential: Lean proves nonsummability at0 and
directly refutes the zero-parameter finite bound using the singleton{1}.
It never treats a divergent totalized infinite sum as its numerical value.
[pcc:unverified]

For genuine uniform injective samples `Fin d ↪ X`, Lean derives the exact
product expectation as the actual product sum divided by
`(|X|).descFactorial d`. When every factor is nonnegative, the numerator is
at most the product of coordinate sums. This follows by embedding injective
samples into all functions; coordinate independence is not assumed.
The condition `d≤|X|` ensures the actual sampling law exists.
[pcc:unverified]

For the actual iid walk, canonical centered residues identify the actual
character’s cosine. Jordan’s inequality and the actual PMF phase-variance
identity then prove the all-time bound below, including its actual expectation
under the recursively defined walk PMF. [pcc:unverified]

```
normSq(E_walk_t[frequencyCharacter j])
  ≤ exp(-8*omega²*t*centeredRadius(support, reference, j)).
```

For every fixed strictly positive finite label law, Lean **derives** omega>0
from its weights before quantifying over prime modulus, support, reference,
frequency, time and starting point. Thus omega is not chosen after seeing the
modulus or support. Trivial frequencies and repeated support points are allowed
in this intermediate estimate; no strict contraction or full mixing is inferred
from those degenerate cases. [pcc:unverified]

Sources checked:

- [Hildebrand1994](https://link.springer.com/content/pdf/10.1007/BF01199265.pdf):
  full13-page read/visual inspection credited; proof197–199 rechecked,
  including the literal Gaussian display199.
- [Author preprint math/0401189v1](https://arxiv.org/abs/math/0401189):
  selected sections4.1/4.2 on12–18 and reference14p36 read, visual15–18;
  new visual17–18 includes the actual equality and following composite-domain
  warning. Not a full37-page read. The separate k=2 issue is author-acknowledged.
  The correction-list attachment remained inaccessible; bounded correction
  queries support no novelty or global absence claim.
- Pinned official Mathlib Gaussian integral, antitone integral test, infinite
  sums, Jordan bound, finite products and minimum APIs; existing actual
  PMF/character/variance/centered-residue/injective-sampling proofs.
  KB MCP absent and wrapper127; primary/prior/official fallback recorded.
  Raw primary PDFs/images/OCR remain local; hashes/read scopes retained.
  [pcc:unverified]

Main declaration: `SupportGaussianAudit.support_gaussian_certificate` in
[GaussianCertificate.lean](lean/GaussianCertificate.lean). See
[proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and [visual report](reports/report.pdf).
Exact local **PCC-20261114-ee1e7f** replay PASS at 2026-11-14T13:04:59Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/support_gaussian_audit_20261114/lean
python3 research/support_gaussian_audit_20261114/restore_attempts.py /tmp/support-gaussian-builds
```

Lean4.35.0 and Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679 pinned.
Default limits, standard foundations; all failed and successful attempts retained,
including the first gate’s sparse-checkout source-discovery failure.
No holes/custom axioms/native evaluation/raised limits/global flags/waivers,
new native experiment, paid compute or agent upload. The full expected
random-support mixing exponent and static rho/collision-penalty/limiting
constants remain open. [pcc:unverified]
