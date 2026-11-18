# Exact counting for injective prime-field supports

Lean proves the finite counting step used in Hildebrand’s box argument with
its lattice correction and sampling denominator retained. For a uniform
injective sample of d elements from a finite population X, the actual number
of samples is `(|X|).descFactorial d`. Every sample has mass equal to the
reciprocal of this number. The probability of coordinate constraints Bᵢ is
exactly the number of admissible injective samples divided by that denominator,
and is at most `∏ᵢ |Bᵢ| / (|X|).descFactorial d`. No coordinate-independence
premise is used. Lean also proves that a sampling PMF cannot exist when d
exceeds the population size. [pcc:unverified]

For every prime p, every d≤p−1 and fixed first support point a₀, Lean constructs
an explicit equivalence from injective d-tuples of nonzero field elements to
**all** injective `(d+1)`-point field support maps whose first point is a₀.
The actual pushforward of the tuple sampler equals the uniform PMF on this
full conditional support space, whose cardinality is `(p−1).descFactorial d`.
This is a complete conditional space, with no selected subclass substituted.
[pcc:unverified]

Let `centered` be the canonical integer representative in `(−p/2,p/2]` checked
in the [preceding packet](../hildebrand_cell_audit_20261114/README.md).
For every nonzero Fourier index j and real intervals Lᵢ≤Uᵢ, this packet proves
that each coordinate constraint has the form below. [pcc:unverified]

```
Lᵢ ≤ centered((a₀ − aᵢ₊₁) * j) / p ≤ Uᵢ
```

has at most `p*(Uᵢ−Lᵢ)+1` possible nonzero choices. It derives this from an
actual injective map to an integer interval, whose cardinality is proved at
most its real width plus one. The resulting actual full-source probability
is at most the expression below. [pcc:unverified]

```
∏ᵢ (p*(Uᵢ−Lᵢ)+1) / (p−1).descFactorial d.
```

The main theorem binds this to the actual source law and event, rather than
to an arbitrary function supplied as a “probability.” [pcc:unverified]

The nonzero-index condition is necessary. For every prime p≥3, index j=0
makes every transformed centered coordinate zero. The zero-width coordinate
box therefore has p−1 choices, exceeding its proposed width-plus-one bound1.
For an actual one-coordinate source sample, that event has probability1,
strictly larger than `1/(p−1)`. Lean directly negates the probability inequality
obtained by deleting j≠0. This is a negative control for a deleted hypothesis,
not a refutation of the qualified source lemma. [pcc:unverified]

The proof closes the exact finite counting core. It does **not** yet establish
the later `1.1` asymptotic approximation: that still requires the prescribed
width-growth conditions and a sufficiently-large-prime estimate for both the
lattice corrections and the falling-factorial denominator. The stronger
expected random-support mixing exponent also remains open. The qualified
mixing theorems and corrected k≥3 lemma are not refuted. [pcc:unverified]

Sources checked:

- [Hildebrand1994, PTRF100,191–203](https://link.springer.com/content/pdf/10.1007/BF01199265.pdf):
  the full13-page original was read and visually inspected in the preceding
  review; the exact numerator `∏(width+1)` and denominator
  `(p−1)…(p−k+1)` occur in the proof on197.
- [Author preprint arXiv:math/0401189v1](https://arxiv.org/abs/math/0401189):
  selected section4.2/pages13–16 and reference14 on36 were read; pages15–16
  visually inspected. Its asymptotic counting lemma is restricted to k≥3.
  The full37-page preprint is not marked read. The acknowledged older k=2
  boundary was formalized in the preceding packet.
- Mathlib’s official finite-embedding, descending-factorial, finite-product
  and integer-interval APIs were read for this proof. Exact source hashes
  and the unavailable-KB fallback are in the source review. No novelty or
  global absence claim is made. [pcc:unverified]

Main declaration: `InjectiveBoxAudit.injective_box_certificate` in
[SamplingCertificate.lean](lean/SamplingCertificate.lean). See the
[proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and [visual report](reports/report.pdf).
Exact local **PCC-20261114-8f6cf7** replay PASS at 2026-11-14T11:23:45Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/injective_box_audit_20261114/lean
python3 research/injective_box_audit_20261114/restore_attempts.py /tmp/injective-box-builds
```

Failed and successful source/configuration snapshots, build logs and exit
codes are retained losslessly. Lean4.35.0 and
Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679 are pinned. Standard
foundations and default limits; no holes, custom axioms, native evaluator,
raised limits, new global elaboration flags or linter waivers. No native
experiment, paid compute or upload. [pcc:unverified]
