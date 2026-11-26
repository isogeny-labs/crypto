# 2026-11-24: quartic Diem 2LP IC, SGE + coordinate Wiedemann versus scalar Wiedemann

All numbers are [pcc:unverified].

- **Study:** [`research/quartic_diem_la_20261123`](../../research/quartic_diem_la_20261123/) (frozen 612f8dee20e, results 23384701f18, lane PR #5081).
- **Validation:** independent validator, verdict validated with errata, gate `valid`. The validator replayed all 148 rows byte-identically and re-checked all 80 certificates with independent arithmetic. See [ERRATA](../../research/quartic_diem_la_20261123/ERRATA.md).

**What was measured.** Diem double-large-prime IC on prime-order plane quartics over F_p, at p = 4099, 16411, 65537 and 131101. There were 5 curves per prime and 2 solver seeds, giving 80 paired runs. Both arms ran on identical instances and draws, and their relation phases are identical.
- `2lp-sfb`, the baseline: scalar Wiedemann on AᵀDA over the whole peeled core.
- `2lp-sfb-sge`: trim, then structured Gaussian elimination, then non-symmetric coordinate Wiedemann for t on a square subsystem. The parameters were W = 8, FILL = 1.5, EXCESS = 64, fixed on development curves. A failed attempt falls back to the full-core solve, fully counted.

**Result (frozen rule): IMPROVEMENT [pcc:unverified].** This is a component speed-up, not a rho claim.

| w | pooled paired GAE ratio sge / sfb | 95% (cluster bootstrap) |
|---|---|---|
| 1 | 0.371 | [0.340, 0.406] |
| 0.1 | 0.372 | [0.340, 0.407] |
| 0 | 1.0028 | [1.0010, 1.0048] |

The decision holds only because the frozen rule allows a w = 0 ratio up to 1.01. The order reverses at w = 0, so this is **not** a §5a ranking across the band (ERRATA E2).

**Against rho.** At p = 131101 and w = 1, IC / ideal negation rho is 7.75 for sfb and **2.78 [2.02, 3.80]** for sge. That 2.78× is the distance still remaining. Every prime is UNRESOLVED across the band for both arms.

**Next lead.** Runs that fall back carry 46–83% of the sge arm's linear algebra at the three largest primes. In those runs t is undetermined at attempt 1 (ERRATA E6), so a rank test on the square subsystem could skip the fallback.

Files:
- `figure.svg` and `figure.png`
- `report.pdf`
- `VALIDATION.json`
- `replay-certificate.json`
- `validator-report.json` and `validator-report.md`
