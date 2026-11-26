# 2026-11-26: quartic Diem 2LP IC, skipping the doomed full-core fallback in `2lp-sfb-sge`

All numbers are [pcc:unverified].

- **Study:** [`research/quartic_diem_nofb_20261126`](../../research/quartic_diem_nofb_20261126/). Frozen in 8297532db36, with seed manifest `seeds.json`. Results in cc7e5c5488a, lane PR #5216.
- **Validation:** independent validator, verdict validated with errata, gate `valid`. It replayed 268 of 268 rows byte-identically and re-checked all 160 certificates with independent arithmetic. See [ERRATA](../../research/quartic_diem_nofb_20261126/ERRATA.md).

**What was measured.** Diem double-large-prime IC on prime-order plane quartics over F_p, at p = 4099, 16411, 65537 and 131101, with 5 curves per prime and solver seeds 1–4. 160 paired runs, with held-out seeds 3–4 as the primary panel.
- `2lp-sfb-sge`, the baseline: when the square coordinate solve fails, it runs a full-core scalar Wiedemann fallback before the next attempt.
- `2lp-sfb-sge-nofb`: skips that fallback.
- The fallback draws only from a cloned RNG, so the two arms pair exactly.

**Result (frozen rule): IMPROVEMENT [pcc:unverified], with the order holding across the band.**

| w | pooled paired GAE ratio nofb / sge, held-out seeds 3–4 | 95% |
|---|---|---|
| 1 | **0.832** | [0.758, 0.907] |
| 0.1 | 0.833 | [0.761, 0.908] |
| 0 | 0.997 | [0.9959, 0.9984] |

- **Dominance.** nofb is no costlier than sge in group operations and in every native counter, on all 80 pairs. So the order holds at every w ≥ 0, and no tolerance is used.
- **Rescues.** None of the baseline's 16 fallbacks certified. The rank diagnosis shows t undetermined on every failed attempt, so a rescue was impossible.
- **Fallback pairs.** On the 15 fallback pairs, the new arm's linear algebra is 20–27% of the baseline's.

**Against rho.** At p = 131101 and w = 1, IC / ideal negation rho falls from 2.63 for sge to **1.98 [1.84, 2.16]** for nofb. That 1.98× is the distance still remaining. Every prime is UNRESOLVED across the band.

Files:
- `figure.svg` and `figure.png`
- `report.pdf`
- `VALIDATION.json`
- `replay-certificate.json`
- `validator-report.json` and `validator-report.md`
