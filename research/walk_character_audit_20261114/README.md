# Actual walk characters and quantitative phase contraction

Lean proves the Fourier coefficient law for the actual independent additive
walk PMF. For any finite abelian group, normalized label PMF q, increments a,
additive complex character ψ, starting point x and time t, the actual complex
expectation is `λ^t*ψ(x)`, where `λ=∑ᵢ qᵢ ψ(aᵢ)`. Its actual squared modulus
is `normSq(λ)^t`. Complex expectation and its bind law are derived from actual
finite PMF probabilities and the checked real expectation law. A supplied
spectral formula is not used as a premise. [pcc:unverified]

The quantitative contraction is derived from the unit-phase identity below,
for normalized PMF weights and phases of squared norm1. [pcc:unverified]

```
1 − normSq(E[z]) = (1/2) ∑ᵢ∑ⱼ qᵢ qⱼ normSq(zᵢ − zⱼ).
```

Every pair defect is nonnegative. Keeping the two ordered terms for distinct
labels u,v gives the bound below for the actual character coefficient.
[pcc:unverified]

```
normSq(λ) ≤ 1 − 2*qᵤ*qᵥ*(1 − Re ψ(aᵤ−aᵥ)).
```

If both weights are positive and `normSq(λ)>1−2*qᵤ*qᵥ*δ`, Lean therefore
proves `Re ψ(aᵤ−aᵥ)>1−δ`. In particular δ=0.01 gives the phase-alignment
cutoff0.99 appearing in the source argument. This is a derived quantitative
pair estimate; the source’s full minimum-over-all-pairs threshold is not
claimed closed here. [pcc:unverified]

The standard prime-field character is an actual Mathlib additive character,
`ψⱼ(x)=ZMod.stdAddChar(x*j)`. Lean proves its literal cosine formula using
the natural residue representative, and proves injectivity when p is prime
and j≠0. Hence every injective support map with two distinct positively
weighted labels has `normSq(λ)<1` for every nonzero index. Its unit-circle
values, phase differences and cosine interpretation are derived from the
actual character, rather than supplied as numerical phase data.
[pcc:unverified]

Three controls check the hypotheses:

- At j=0, the actual eigenvalue and every-time squared character moment
  equal1. This is the trivial mode explicitly excluded in the source.
- Over every prime field, the actual pure-false label PMF has weights1/0
  on the distinct support points0/1. At index1 its coefficient equals1,
  directly negating strict contraction after positivity is deleted.
- On `ZMod 4`, actual fair positive label weights and distinct increments0/2
  have coefficient1 at the nonzero index2. Thus nonzero index and distinct
  support do not restore strict contraction after primality is deleted.
  These are domain controls, not refutations of the qualified source.
  [pcc:unverified]

Sources checked:

- [Hildebrand1994, PTRF100,191–203](https://link.springer.com/content/pdf/10.1007/BF01199265.pdf):
  full13-page original read and visually inspected in the preceding review;
  Lemma3 and the phase-alignment argument in Lemma5 on195–198 rechecked.
- [Author preprint arXiv:math/0401189v1](https://arxiv.org/abs/math/0401189):
  selected section4.2/Proposition2 and the explicit j≠0 condition on13–16
  rechecked; selected13–16/reference14p36 read and visual15–16 credited.
  This is not a full37-page preprint read.
- Official Mathlib character orthogonality, standard-character formulas and
  injectivity, selected finite Fourier/basis APIs, complex unit norm and
  trigonometric identities; existing actual weighted-kernel PMF proof.
  Exact hashes and unavailable-KB fallback are in the source review. No
  Diaconis-book or original Upper Bound Lemma source read is claimed, and
  no novelty or global absence claim is made. [pcc:unverified]

Main declaration: `WalkCharacterAudit.walk_character_certificate` in
[CharacterCertificate.lean](lean/CharacterCertificate.lean). See
[proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and [visual report](reports/report.pdf).
Exact local **PCC-20261114-c4aad9** replay PASS at 2026-11-14T12:00:51Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/walk_character_audit_20261114/lean
python3 research/walk_character_audit_20261114/restore_attempts.py /tmp/walk-character-builds
```

The stronger expected random-support mixing exponent still requires the
full spectral sum/Parseval-to-total-variation bound and the averaging argument.
The [corrected box-counting asymptotic](../hildebrand_box_asymptotic_audit_20261114/README.md)
is already proved separately. Static rho collision laws remain a distinct
open question. Lean4.35.0/Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679
pinned; standard foundations/default limits. Failed and successful states
retained, with no holes, custom axioms, native evaluator, raised limits,
global elaboration flags or linter waivers. No native experiment or paid
compute. [pcc:unverified]
