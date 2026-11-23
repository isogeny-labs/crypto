# Hildebrand's acknowledged k=2 counting boundary

Lean refutes the literal k=2 specialization of Lemma4 in Hildebrand1994.
Fix ε=1, m₂=4, first support point a₁=0 and Fourier index j=1. For every
prime p≥3, the printed cell is the interval `[1/p,5/(4p)]` for the normalized
centered residue, and its printed probability bound is `11/(40p)`.
The actual uniform cell probability is `1/(p−1)`, which is strictly larger.
Lean directly negates existence of any sufficiently-large-prime threshold
that makes this fixed specialization true. [pcc:unverified]

This is an acknowledged boundary issue. The later author preprint
[arXiv:math/0401189v1](https://arxiv.org/abs/math/0401189), selected section4.2,
treats k=2 separately, restricts the corresponding counting lemma to k≥3,
and explicitly asks readers to explain the older k=2 proof failure.
The current result formalizes that known issue; it makes no novelty claim.
It does not refute either mixing Theorem1 or Theorem2, or the later
k≥3 counting lemma. [pcc:unverified]

The proof uses the full source distribution. `Choice p` is the actual
nonzero elements of `ZMod p`, with cardinality p−1 proved from the finite
field unit-group equivalence. A genuine normalized uniform Mathlib `PMF`
is constructed. Lean builds an actual equivalence to **all** injective
two-point support maps whose first value is zero, and proves that mapping
the sampler gives the uniform law on that complete conditional space.
Thus a selected subclass of supports is not substituted. The actual
source expression `(a₁−a₂)*j` is proved equal to `−a₂` for these choices.
[pcc:unverified]

The centered representative is also checked. Its cast is the original
residue, its integer value lies in the source half-open interval
`(−p/2,p/2]`, and it is the unique representative satisfying those
conditions. The original real-power endpoint expressions and the decimal
constant1.1 are retained in definitions, then reduced by Lean to the
stated rational endpoints and bound. [pcc:unverified]

For this cell, the integer representative must be exactly1. That is
equivalent to the actual choice `a₂=−1`. The event is therefore one
point in a uniform space of p−1 points. The exact expectation of its
indicator is `1/(p−1)`. The strict comparison with `11/(40p)` holds for
every prime p≥3. Euclid's infinitude of primes supplies the direct
negation of an eventual prime bound, with no enumeration cutoff.
[pcc:unverified]

The positive control is the valid two-support mixing theorem with any
fixed normalized strictly positive weights. Its actual independent
increment law has a support-independent quadratic time constant over
prime fields. This is imported from the preceding
[general-weight coupling proof](../weighted_cycle_coupling_audit_20261114/README.md),
which does not depend on the invalid counting bound. No supplied mixing
premise or repaired counting bound is assumed. [pcc:unverified]

Primary sources checked:

- [Hildebrand1994, PTRF100,191–203](https://link.springer.com/content/pdf/10.1007/BF01199265.pdf):
  full13 published pages/proofs/discussion/references read and all13 pages
  visually inspected in the preceding review. Original Lemma4 on196 and
  proof197 were rechecked, including enlarged printed formulas and the
  k≥2 section assumption on195.
- [Later author preprint, arXiv:math/0401189v1](https://arxiv.org/abs/math/0401189):
  selected section4.2/pages13–16 and reference14 on36 read; pages15–16
  visually inspected. Reference14 identifies the original1994 paper.
  The full37-page preprint is not marked read.
- The author's [corrections listing](https://www.albany.edu/~martinhi/pubsoops.html)
  was found in the index. Its attachment remains unread after the recorded
  SSO/500 and alternate-host failures. The acknowledgement above comes
  from the accessible author preprint itself. [pcc:unverified]

Main declaration: `HildebrandCellAudit.hildebrand_cell_certificate`, in
[CellCertificate.lean](lean/CellCertificate.lean). See
[proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and [visual report](reports/report.pdf).
Exact local **PCC-20261114-4dc478** replay PASS at 2026-11-14T10:38:01Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/hildebrand_cell_audit_20261114/lean
python3 research/hildebrand_cell_audit_20261114/restore_attempts.py /tmp/hildebrand-cell-builds
```

Wholegate30 declarations/zero failures/zero trusted native exceptions.
All54 build/processing/provenance files retained losslessly, including
failed source/config/log/exit states. Lean4.35.0 and
Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679 pinned. Standard
foundations/default limits; no holes, custom axioms, native evaluator,
raised limits, global elaboration flags or linter waivers. KB/Conductor
unavailable; bounded primary/prior/official fallback before code recorded.
The stronger general random-support exponent and other fourteen-item
gaps remain open. No corpus promotion, full14 closure, native experiment,
paid compute or agent upload. [pcc:unverified]
