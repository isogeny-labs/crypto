# Finite Fourier upper bound with checked normalization

Lean proves the finite-abelian Upper Bound Lemma for actual normalized PMFs
and a genuine uniform PMF. The proof derives finite complex Plancherel from
character orthogonality, then combines it with squared Cauchy–Schwarz and
actual probability normalization. It does not assume a Fourier-energy or
mixing bound. [pcc:unverified]

The source convention is explicit: `totalVariation = l1Distance/2`, where
full L1 sums the absolute differences of actual point probabilities. Lean
proves this half-L1 value lies in `[0,1]`, bounds every event discrepancy,
and is attained by the actual event where the first probability exceeds
the second. This identifies the source’s maximum-event distance exactly.
[pcc:unverified]

For every finite abelian group and every PMF q, the resulting bound is the
expression below. The Fourier coefficient is the actual expectation of the
complex character, without an extra normalization factor. [pcc:unverified]

```
totalVariation(q, uniform)^2
  ≤ (1/4) * ∑ nontrivial characters ψ, normSq(E_q[ψ]).
```

Uniform point masses and event probabilities are derived as `1/|G|` and
`|A|/|G|`. The actual uniform coefficient is1 for the trivial character and0
for every other character. The trivial coefficient of the probability
difference vanishes; Lean proves its removal from the spectral sum.
[pcc:unverified]

The actual [independent-walk character law](../walk_character_audit_20261114/README.md)
then gives the walk bound below for every time and starting point. The same
right-hand side bounds every event’s squared discrepancy from `|A|/|G|`.
No supplied convolution-power or spectral formula is used. [pcc:unverified]

```
totalVariation(walk_t, uniform)^2
  ≤ (1/4) * ∑ nontrivial ψ, normSq(∑ᵢ qᵢ ψ(aᵢ))^t.
```

The normalization control uses an actual point mass at0 and the actual
uniform law on `ZMod 2`. Lean computes full L1=1, half-L1=1/2 and nontrivial
spectral energy=1. Thus the half-L1 bound is an equality, while substituting
full L1 into its quarter-factor version gives a directly refuted inequality.
The same example proves any universal half-L1 coefficient must be at least
1/4, already on this two-point group. This checks a convention change; it
refutes neither source under its stated definitions. [pcc:unverified]

Sources checked:

- [Diaconis–Shahshahani1981, published paper](https://www.imo.universite-paris-saclay.fr/~pierre-loic.meliot/symmetric/texts/Diaconis%2C%20Shahshahani%20-%20Generating%20a%20random%20permutation%20with%20random%20transpositions.pdf):
  selected definition/convolution/Plancherel passages and the complete
  Lemma14 proof on168 read; images159/161/162/168 inspected. Its definition
  is full L1 and its corresponding bound has no1/4 factor. This is not a
  full21-page paper read.
- [Diaconis author report269, April1987](https://purl.stanford.edu/ps020md6873):
  selected chapter3B printed29–33/PDF35–39 OCR read; printed29/32/33 and
  contents visually inspected. It defines maximum-event=half-L1 and states
  the quarter-factor Upper Bound Lemma. The public Stanford download was
  checked against the repository’s SHA1 and byte count. This is the1987
  author report, not the1988 published book’s pagination or a full271-page
  read. The published book endpoint returned a bot check.
- [Hildebrand1994](https://link.springer.com/content/pdf/10.1007/BF01199265.pdf):
  full13-page read/inspection credited; Upper Bound Lemma on195 rechecked.
  [Author preprint math/0401189v1](https://arxiv.org/abs/math/0401189):
  selected sections4.1/4.2 on12–16 and reference14p36 read; visual15–16
  credited. Its explicit unitary convention, unnormalized probability
  transform and nontrivial-character exclusion are retained.
- Official Mathlib finite-character orthogonality, finite sums and squared
  Cauchy–Schwarz, plus existing actual PMF/uniform-law proofs. Mathlib’s
  general measure `tvDist` uses the full-variation convention; this packet
  uses explicit finite PMF distances and does not claim that separate
  measure-interface conversion. Primary PDFs/scans/OCR stay local; their
  hashes, selected scopes and processing metadata are retained.
  [pcc:unverified]

Main declaration:
`FiniteFourierUpperBoundAudit.finite_fourier_upper_bound_certificate`, in
[UpperBoundCertificate.lean](lean/UpperBoundCertificate.lean). See
[proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and [visual report](reports/report.pdf).
Exact local **PCC-20261114-e1a4a9** replay PASS at 2026-11-14T12:27:36Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/finite_fourier_upper_bound_audit_20261114/lean
python3 research/finite_fourier_upper_bound_audit_20261114/restore_attempts.py /tmp/finite-fourier-upper-bound-builds
```

The general nonabelian matrix statement and the stronger expected random-support
mixing exponent remain open. The latter still needs spectral averaging with
the separately proved [corrected box asymptotic](../hildebrand_box_asymptotic_audit_20261114/README.md).
Lean4.35.0/Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679 pinned;
standard foundations/default limits. Failed and successful states retained,
including the first whole gate. A dry PCC extraction initially truncated at
ASCII absolute-value notation; equivalent explicit `abs` syntax was rebuilt
and checked again before any claim issue, so the binding covers the full
statement. No holes, custom axioms, native evaluator, raised limits, new
global elaboration flags, linter waivers, native experiment or paid compute.
[pcc:unverified]
