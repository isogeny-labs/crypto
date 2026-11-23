# Qualified expected random-support mixing proved in Lean

Lean proves Hildebrand1994Theorem1 for every fixed k=d+1≥2 and every fixed
strictly positive normalized label PMF. For every epsilon>0 it derives
gamma>0 and N before choosing the prime modulus or support. For every prime
p≥N and every starting point, the actual expected half-L1 distance from
uniform at exact time `floor(gamma*p^(2/d))` is less than epsilon. The
expectation is over **all ordered distinct supports**, as in the source.
This includes k=2. [pcc:unverified]

The support law is the genuine uniform PMF on embeddings `Fin(d+1) ↪ Fp`;
its point mass is `1/(p.descFactorial(d+1))`. Lean constructs the actual
bijection with a first point and a relative injective sample, then proves
the expectation decomposes into uniform first point and uniform relative
sample. Independence of the distinct support coordinates is never assumed.
The walk is the actual recursively defined iid-increment PMF from the
[character-law proof](../walk_character_audit_20261114/README.md), and the
distance is the actual half-L1/max-event quantity from the
[finite Fourier proof](../finite_fourier_upper_bound_audit_20261114/README.md).
[pcc:unverified]

For a nonzero frequency, multiplication and canonical centering inject the
nonzero relative field choices into nonzero integers. The actual Gaussian
integral bound and injective-product estimate then give the averaged
character moment. Lean proves the standard frequency map is bijective onto
**every** complex additive character; no nontrivial character is omitted.
The actual Fourier bound and weighted PMF Cauchy–Schwarz give the finite
mean-square bound below. Here omega>0 is derived solely from the fixed
label probabilities. [pcc:unverified]

```
(E_support[distance])² ≤ E_support[distance²]
  ≤ ((p-1)/4) * sqrt(pi/(8*omega²*t/p²))^d
      / ((p-1).descFactorial d).
```

The separately proved denominator limit supplies a threshold with
`(p-1).descFactorial d ≥ p^d/2`. Lean then derives the fourth-power bound
below. This is the fourth power of the expected distance, not the
expectation of its fourth power. [pcc:unverified]

```
(E_support[distance])^4 ≤ (pi/(8*omega²))^d * p²/(4*t^d).
```

For K=pi/(8*omega²), A=K^d, R=A/epsilon^4+1 and gamma=2R, Lean proves
`floor(gamma*p^(2/d)) ≥ R*p^(2/d)` and `t^d ≥ R*p²`. Thus the fourth power
is at most A/(4R), strictly below epsilon^4. The source’s real-power
exponent, floor and quantifier order are retained; no average, mixing or
asymptotic conclusion is supplied as a premise. [pcc:unverified]

The negative control uses an actual pure-label PMF with zero weights on
the other labels. Its actual walk is deterministic. For every prime, all
times and starts, averaging over all ordered distinct supports leaves
expected distance at least1/2. Lean directly negates existence of any
gamma/N giving less than1/4 on sufficiently large primes. This refutes
deleting strict positivity, while retaining the qualified source.
The earlier [literal Gaussian equality counter](../support_gaussian_audit_20261114/README.md)
and [known k2 counting error](../hildebrand_cell_audit_20261114/README.md)
remain separate intermediate issues with valid repairs and author-correction
credit; neither refutes this qualified mixing theorem. [pcc:unverified]

Sources checked:

- [Hildebrand1994](https://link.springer.com/content/pdf/10.1007/BF01199265.pdf):
  full13-page read/visual inspection credited. Theorem1 on192 and proof197–199
  rechecked: fixed k≥2, fixed strictly positive normalized weights, prime
  modulus, uniform ordered distinct support and exact floored time scale.
- [Author preprint math/0401189v1](https://arxiv.org/abs/math/0401189):
  selected sections4.1/4.2 on12–18 and reference14p36 read, visual15–18.
  Not a full37-page read; its separate k2 correction and composite-domain
  warnings remain credited. No novelty or global absence claim.
- Pinned official Mathlib finite-character cardinality/standard-character
  injectivity, finite sums and Cauchy–Schwarz, square roots, powers and floor;
  prior actual uniform sampling/character/variance/Fourier/denominator proofs.
  KB MCP absent and fresh status/two mechanism-negative queries/theorem query
  all127; primary/prior/official fallback recorded. Raw primary PDFs/images/OCR
  remain local, hashes/read scopes retained. [pcc:unverified]

Main declaration: `RandomSupportAverageAudit.random_support_average_certificate`
in [AverageCertificate.lean](lean/AverageCertificate.lean). See
[proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and [visual report](reports/report.pdf).
Exact local **PCC-20261114-4f738d** replay PASS at 2026-11-14T13:26:37Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/random_support_average_audit_20261114/lean
python3 research/random_support_average_audit_20261114/restore_attempts.py /tmp/random-support-average-builds
```

Lean4.35.0/Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679 pinned.
All failed/successful source/config/build states and first whole gate retained.
A dry binding check caught parser termination at a named-argument separator
before issue; equivalent explicit type syntax rebuilt/gated and the complete
statement bound. Default limits,
standard foundations; no holes/custom axioms/native evaluation/raised limits,
global flags/linter waivers/new native experiment/paid compute/agent upload.
The qualified fixed-k expected random-support exponent is now closed.
Static rho walks, collision penalty and limiting constants, together with
the other original fourteen-item proof/execution gaps, remain open.
[pcc:unverified]
