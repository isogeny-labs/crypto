# L4-lifting: report (transcribed by the main session)

The lab's own write of this file was blocked by the harness. The main session copied the content below from the
lab's final hand-back message without editing its claims. Every number is `[pcc:unverified]`. Proof: PROOF.md.
Protocol: PROTOCOL.md (sha256 in PROTOCOL.sha256, recorded before data). Deviations: AMENDMENTS.md. Outputs:
MANIFEST.sha256 (38 files). research-kb and the lead atlas were unavailable; yesterday's lifting outputs were gone,
so everything was rebuilt.

## Summary
- **Theorem X (paper proof; xedni-style lift chosen per trial).** Any relation among lifted points can be taken
  short (Cramer, Hadamard, Minkowski), so per-trial success ≤ |M|/q with
  |M| = Σ_{s<k} C(k, s+1)·((2⌊(γ_s ρ₀)^{s/2}⌋ + 1)^{s+1} − 1)/2. No torsion factor; independent of how the lift
  is built, number-field degree, residue degree, height normalisation. Replaces the closure note's pigeonhole bound.
- **Growing rank.** Xedni bound stops excluding at S* ≈ √(log₂ q / log₂(γρ₀)) (256 bits: 10/8/6/4 for
  ρ₀ = 1.5/10/100/10⁴). Theorem F (lift fixed in advance; Silverman–Suzuki): per-trial ≤ L·T·(1+2√(B/λ))^r/q,
  stops excluding at r* = Θ(log p / log ρ) (256 bits: 70/44/29/17 for ρ = 1.5/10/100/10⁴).
- Unconditional ρ₀ (Petsche 2006 + Silverman 1990, recalled) is ≈ 10³⁵ at 256 bits, so the unconditional
  concrete bound excludes nothing beyond rank 2. Mestre's GRH rank bound O(log N / log log N) is not below either
  threshold; bounded rank over Q (Park–Poonen–Voight–Wood heuristic) would exclude both asymptotically.

## Verdicts
| Sub-question | Verdict |
|---|---|
| (i) Silverman–Suzuki with growing rank | INCONCLUSIVE. Excluded only under bounded rank over Q; open unconditionally and under GRH. Real blocker (unproved): no known fast way to find a small-height rational point reducing to a given target. |
| (ii) 256-bit margins | Rank 7 and 8 excluded at measured ρ (conditional on ρ ≤ 11.0 and ≤ 5.04). Rank 9 INCONCLUSIVE (+8.3 bits median ρ, −4.8 worst). Rank ≥ 10 not excluded. |
| (iii) Number-field / Q-curve lifts | ADVANCES (paper proof): Theorem X covers them, closing the Prop 3 gaps. Exclusion over fields of growing degree stays open. |
| (iv) Other rational points | Partly closed (fixed factor-base points: Thm X; small-height points on a fixed lift: Thm F; generator images via group operations: generic, Shoup). Hybrids open. |
| Toy (growing rank helps?) | KILL at toy scale. |

## 256-bit margins (margin = 128 − log₂|M|; positive = excluded)
| Rank | Lifted pts | Source | Configs | ρ median / 95th / max | Margin @ median | Margin @ max | ρ at margin 0 |
|---|---|---|---|---|---|---|---|
| 7 | 8 | Mestre | 147 | 1.20 / 1.47 / 2.14 | +89.4 | +66.3 | 11.0 |
| 7 | 8 | Mestre 16/32/64-bit coeffs | 119 | max 1.17 | | ≥ +91.0 | 11.0 |
| 8 | 9 | Mestre + 1 searched pt | 44 | 1.45 / 1.86 / 2.57 | +64.7 | +34.9 | 5.04 |
| 9 | 10 | Mestre + 2 searched pts | 14 | 1.69 / 1.92 / 2.07 | +8.3 | −4.8 | 1.92 |

Main caveat: ρ was measured on independent (Mestre) configurations, never on a successful dependent lift.

## Toy (plane cubic, 2–8 lifted points; MITM over {−2..2}^k mod a 61-bit prime, confirmed mod a second prime and over Q)
| Bits | Honest trials | Non-trivial relations | Share with a mod-p relation | Planted found |
|---|---|---|---|---|
| 10 | 2,100 | 1 (R₃ = −R₁ coincidence) | 0–1.0 | 178/178 |
| 12 | 2,100 | 0 | 0–1.0 | 180/180 |
| 14 | 2,100 | 0 | 0.01–1.0 | 180/180 |
| 16 | 2,100 | 0 | 0–1.0 | 180/180 |
| 20 | 2,100 | 0 | 0–0.25 | 180/180 |

≤ 0.22 % (95 %) of mod-p relations were realised over Q. Per-trial success < 4.8e-4 (95 %) at 14–20 bits. At 20 bits
alone the bound (1.4e-3) is not below rho's per-trial rate (≈ 1.0e-3); there the per-trial cost (≥ 25 group ops plus
a lattice reduction) separates them. No finite gap to rho can be quoted.

## Not shown
No lower bound on ρ for successful lifts; no result on finding small-height preimages; hybrids in (iv); cited
theorems recalled, not re-read; loose Hermite constant (1 + s/4) for s ≥ 9; ranks 8–9 only on small-coefficient curves.

## Next action
Measure ρ on planted dependent configurations at 128–256 bits with 9–10 lifted points; then replace the loose Hermite
constant with Cohn–Elkies bounds (re-read from the source), which alone could move rank 9 to excluded.
