# The corrected Hildebrand box-counting lemma

Lean proves the corrected counting lemma from the author’s later preprint,
section4.2/Lemma2. Fix d=k−1≥2 and ε>0. There is a threshold N, depending on
d and ε, such that every prime p≥N satisfies the following bound for all
integer cell shifts m, fixed first support points a₀ and nonzero Fourier
indices j. The actual uniform conditional probability of the printed source
box is at most `1.1*(ε/2)/(2^d*p)`. The threshold is chosen before the cell,
first point and index. [pcc:unverified]

The probability uses **all** injective `(d+1)`-point support maps whose first
point is a₀. The preceding [finite counting proof](../injective_box_audit_20261114/README.md)
constructs a genuine normalized Mathlib PMF on this full conditional space,
proves its cardinality `(p−1).descFactorial d` and derives the exact source
probability bound. This packet binds that actual law and cardinality again
in its main theorem. No support subclass, supplied probability formula or
coordinate-independence premise is substituted. [pcc:unverified]

The source’s unnormalized coordinate width is retained literally:

```
W(d,ε,p) = (ε/2)^(1/d) * p^((d−1)/d) / 2.
```

The box uses the canonical centered representative checked in the preceding
packet. Its ith coordinate satisfies the expression below; Lean proves
explicit identities with the literal printed real-power endpoints.
[pcc:unverified]

```
mᵢ*W/p ≤ centered((a₀−aᵢ₊₁)*j)/p ≤ (mᵢ+1)*W/p.
```

Lean derives both limits needed in the source proof:

- `(p−1).descFactorial d / p^d → 1`, by an actual product identity and
  the limit of each factor `1−i/p`.
- `W → ∞` for d≥2, together with the exact identity
  `W^d = ((ε/2)/2^d)*p^(d−1)`.
- The correction factor `(1+1/W)^d / ((p−1).descFactorial d/p^d) → 1`.
  An exact ratio identity then supplies the `1.1` threshold and bounds the
  actual source probability. No asymptotic, mixing or cardinality premise
  is assumed. [pcc:unverified]

The k=2 boundary remains explicit. Lean proves `W(1,ε,p)=ε/4` and directly
negates divergence of that fixed width. The main theorem also retains the
preceding actual counterexample to the original printed k=2 bound: for
ε=1, m₂=4, a₁=0 and j=1, every prime p≥3 has actual cell probability
`1/(p−1)`, exceeding the printed `11/(40p)`. Lean directly negates any
sufficiently-large-prime threshold for that specialization. The later
author preprint explicitly acknowledges the older failure and treats k=2
separately. This is a credited known issue, with no novelty claim.
[pcc:unverified]

Primary sources checked:

- [Author preprint arXiv:math/0401189v1](https://arxiv.org/abs/math/0401189):
  selected section4.2/pages13–16 and reference14 on36 read; pages15–16
  visually inspected. Lemma2 explicitly requires k≥3, states the printed
  probability constant and endpoints, and derives the asymptotic from
  exact lattice counting. Its following exercise flags the older k=2
  proof failure. This is not a full37-page preprint read.
- [Hildebrand1994, PTRF100,191–203](https://link.springer.com/content/pdf/10.1007/BF01199265.pdf):
  full13 published pages read and visually inspected in the preceding
  review. Original proof197 supplies the exact product(width+1) numerator
  and descending-factorial denominator; the older k=2 error is kept
  separate from the valid mixing theorems.
- Official Mathlib factorial-product, real-power, finite-product limit and
  filter/field APIs, plus the preceding finite-counting sources and exact
  replay. Exact hashes and unavailable-KB fallback are recorded in the
  source review. [pcc:unverified]

Main declaration:
`HildebrandBoxAsymptoticAudit.corrected_box_asymptotic_certificate`, in
[BoxAsymptoticCertificate.lean](lean/BoxAsymptoticCertificate.lean). See
[proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and [visual report](reports/report.pdf).
Exact local **PCC-20261114-cbcb79** replay PASS at 2026-11-14T11:39:39Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/hildebrand_box_asymptotic_audit_20261114/lean
python3 research/hildebrand_box_asymptotic_audit_20261114/restore_attempts.py /tmp/hildebrand-box-asymptotic-builds
```

Lean4.35.0 and Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679 pinned.
Failed and successful build/source/config states retained losslessly.
Standard foundations/default limits, with no holes, custom axioms, native
evaluator, raised limits, new global elaboration flags or linter waivers.
The stronger expected random-support mixing exponent still needs the
spectral argument. No native experiment or paid compute. [pcc:unverified]
